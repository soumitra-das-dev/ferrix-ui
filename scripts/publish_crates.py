#!/usr/bin/env python3
"""
Ferrix Crates.io Publishing Script with Rate-Limiting & Burst Handling.

Enforces crates.io rate limits:
1. Brand new crates:
   - Burst of 5 new crates published at once.
   - Rate limit of 1 crate every 10 minutes (600s) allowed after that burst.
2. New versions of existing crates:
   - Burst of 30 new versions published at once.
   - Rate limit of 1 crate per minute (60s) allowed after that burst.
3. Topological ordering based on inter-crate dependencies.
4. Automatic skip for already published crate versions.
5. Automatic retry with exact backoff on HTTP 429 Too Many Requests.
"""

import argparse
from datetime import datetime, timezone
import email.utils
import glob
import json
import os
import re
import subprocess
import sys
import time
import urllib.error
import urllib.request

# crates.io Rate Limiting Parameters
NEW_CRATE_BURST = 5
NEW_CRATE_RATE_SECS = 600.0  # 1 crate every 10 minutes

EXISTING_CRATE_BURST = 30
EXISTING_CRATE_RATE_SECS = 60.0  # 1 crate every 1 minute

INDEX_PROPAGATION_DELAY = 20.0  # seconds between successful publishes


class TokenBucket:
    def __init__(self, capacity: float, refill_interval: float):
        self.capacity = float(capacity)
        self.tokens = float(capacity)
        self.refill_interval = float(refill_interval)
        self.last_update = time.time()

    def consume(self, dry_run: bool = False):
        now = time.time()
        elapsed = now - self.last_update
        self.tokens = min(self.capacity, self.tokens + (elapsed / self.refill_interval))
        self.last_update = now

        if self.tokens < 1.0:
            wait_time = (1.0 - self.tokens) * self.refill_interval
            print(f"  ⏳ Rate limit reached. Waiting {wait_time:.1f}s for token refill...")
            if not dry_run:
                time.sleep(wait_time)
            self.tokens = 1.0
            self.last_update = time.time()

        self.tokens -= 1.0

    def reset(self):
        """Reset tokens to 0 when remote server reports rate limit exhausted."""
        self.tokens = 0.0
        self.last_update = time.time()


def get_workspace_version() -> str:
    with open("Cargo.toml", "r") as f:
        content = f.read()
    m = re.search(r'\[workspace\.package\][^\[]*version\s*=\s*"([^"]+)"', content)
    return m.group(1) if m else "0.1.0"


def discover_crates() -> dict:
    ws_version = get_workspace_version()
    crates = {}

    for path in sorted(glob.glob("crates/*/Cargo.toml")):
        with open(path, "r") as f:
            content = f.read()

        name_match = re.search(r'\[package\][^\[]*name\s*=\s*"([^"]+)"', content)
        if not name_match:
            continue
        pkg_name = name_match.group(1)

        ver_match = re.search(r'\[package\][^\[]*version\s*=\s*"([^"]+)"', content)
        if ver_match:
            pkg_version = ver_match.group(1)
        else:
            pkg_version = ws_version

        deps = set(re.findall(r'(ferrix-[a-z0-9-]+)\s*=\s*\{', content))
        crates[pkg_name] = {
            "name": pkg_name,
            "version": pkg_version,
            "path": os.path.dirname(path),
            "deps": deps,
        }

    return crates


def topological_sort(crates: dict) -> list:
    in_degree = {k: 0 for k in crates}
    graph = {k: [] for k in crates}

    for u in crates:
        for v in crates[u]["deps"]:
            if v in crates:
                graph[v].append(u)
                in_degree[u] += 1

    queue = [u for u in crates if in_degree[u] == 0]
    order = []
    while queue:
        u = queue.pop(0)
        order.append(u)
        for v in graph[u]:
            in_degree[v] -= 1
            if in_degree[v] == 0:
                queue.append(v)

    if len(order) != len(crates):
        missing = set(crates) - set(order)
        raise RuntimeError(f"Cyclic dependency detected among crates: {missing}")

    return order


def check_crate_status(name: str, target_version: str) -> tuple:
    """
    Returns (status, current_version):
      - status: 'brand_new', 'already_published', 'new_version'
      - current_version: string or None
    """
    url = f"https://crates.io/api/v1/crates/{name}"
    req = urllib.request.Request(url, headers={"User-Agent": "ferrix-publisher"})
    try:
        with urllib.request.urlopen(req) as resp:
            data = json.loads(resp.read().decode())
            published_versions = [v["num"] for v in data.get("versions", [])]
            max_ver = data["crate"].get("max_version", "")
            if target_version in published_versions:
                return "already_published", max_ver
            else:
                return "new_version", max_ver
    except urllib.error.HTTPError as e:
        if e.code == 404:
            return "brand_new", None
        raise


def extract_retry_after(err_output: str, default_backoff: float) -> float:
    """Extract wait duration from crates.io error message if present."""
    m = re.search(r'Please try again after ([A-Za-z]+,\s+\d+\s+[A-Za-z]+\s+\d+\s+\d+:\d+:\d+\s+GMT)', err_output)
    if m:
        try:
            target_dt = email.utils.parsedate_to_datetime(m.group(1))
            now = datetime.now(timezone.utc)
            diff = (target_dt - now).total_seconds()
            if diff > 0:
                print(f"  ⏰ Crates.io requested retry after {m.group(1)} ({diff:.1f}s remaining)")
                return diff + 5.0  # 5s safety buffer
        except Exception as e:
            print(f"  ⚠️ Could not parse retry datetime: {e}")
    return default_backoff


def publish_crate(crate_info: dict, token: str, dry_run: bool = False, no_verify: bool = False) -> tuple:
    """
    Returns (success: bool, is_rate_limit: bool, retry_delay: float)
    """
    name = crate_info["name"]
    version = crate_info["version"]

    cmd = ["cargo", "publish", "-p", name]
    if no_verify:
        cmd.append("--no-verify")
    if token:
        cmd.extend(["--token", token])
    if dry_run:
        cmd.append("--dry-run")

    print(f"  🚀 Executing: cargo publish -p {name} (v{version})")
    res = subprocess.run(cmd, capture_output=True, text=True)

    if res.returncode == 0:
        print(f"  ✅ Successfully published '{name}' v{version}")
        return True, False, 0.0

    err_output = res.stderr + "\n" + res.stdout
    print(f"  ❌ Publish failed for '{name}':\n{err_output}")

    # Check for rate limiting / 429
    if "429" in err_output or "Too Many Requests" in err_output or "rate limit" in err_output.lower():
        delay = extract_retry_after(err_output, NEW_CRATE_RATE_SECS)
        return False, True, delay

    # Check if crates.io says already uploaded
    if "already uploaded" in err_output.lower() or "already exists" in err_output.lower():
        print(f"  ℹ️ Version {version} already exists on crates.io.")
        return True, False, 0.0

    return False, False, 0.0


def main():
    parser = argparse.ArgumentParser(description="Publish Ferrix crates with rate limit compliance.")
    parser.add_argument("--token", default=os.environ.get("CARGO_REGISTRY_TOKEN") or os.environ.get("CRATES_IO_TOKEN"))
    parser.add_argument("--dry-run", action="store_true", help="Simulate publishing and rate limiter")
    parser.add_argument("--no-verify", action="store_true", default=True, help="Pass --no-verify to cargo publish")
    args = parser.parse_args()

    if not args.token and not args.dry_run:
        print("❌ Error: No registry token provided via --token or CARGO_REGISTRY_TOKEN / CRATES_IO_TOKEN env vars.")
        sys.exit(1)

    print("🔍 Discovering workspace crates...")
    crates = discover_crates()
    ordered_names = topological_sort(crates)
    print(f"📦 Found {len(ordered_names)} crates in topological order.")

    new_crate_bucket = TokenBucket(NEW_CRATE_BURST, NEW_CRATE_RATE_SECS)
    existing_crate_bucket = TokenBucket(EXISTING_CRATE_BURST, EXISTING_CRATE_RATE_SECS)

    new_publishes = 0
    existing_publishes = 0
    skipped = 0

    for idx, name in enumerate(ordered_names, 1):
        info = crates[name]
        version = info["version"]
        print(f"\n[{idx}/{len(ordered_names)}] Checking '{name}' v{version}...")

        try:
            status, cur_ver = check_crate_status(name, version)
        except Exception as e:
            print(f"  ⚠️ Error checking status for {name}: {e}. Retrying in 5s...")
            time.sleep(5)
            status, cur_ver = check_crate_status(name, version)

        if status == "already_published":
            print(f"  ⏭️ Already published at v{version} (latest: {cur_ver}). Skipping.")
            skipped += 1
            continue

        if status == "brand_new":
            print(f"  ✨ Brand new crate '{name}'.")
            print(f"     Burst limit: {NEW_CRATE_BURST} new crates. Rate limit: 1 crate every 10 min.")
            new_crate_bucket.consume(dry_run=args.dry_run)
            new_publishes += 1
        else:
            print(f"  🔄 New version v{version} of existing crate (currently at {cur_ver}).")
            print(f"     Burst limit: {EXISTING_CRATE_BURST} versions. Rate limit: 1 crate every 1 min.")
            existing_crate_bucket.consume(dry_run=args.dry_run)
            existing_publishes += 1

        # Attempt publish with retry for 429
        max_attempts = 10
        published = False
        for attempt in range(1, max_attempts + 1):
            success, is_429, retry_delay = publish_crate(info, args.token, dry_run=args.dry_run, no_verify=args.no_verify)
            if success:
                published = True
                break

            if not is_429:
                print(f"  ❌ Non-rate-limit error encountered for '{name}'.")
                break

            # Handle 429
            print(f"  ⚠️ Crates.io 429 rate limit hit for '{name}'. Attempt {attempt}/{max_attempts}.")
            if status == "brand_new":
                new_crate_bucket.reset()
                backoff = max(retry_delay, NEW_CRATE_RATE_SECS)
            else:
                existing_crate_bucket.reset()
                backoff = max(retry_delay, EXISTING_CRATE_RATE_SECS)

            print(f"  ⏳ Waiting {backoff:.1f}s before retry attempt {attempt + 1}/{max_attempts}...")
            if not args.dry_run:
                time.sleep(backoff)

        if not published:
            print(f"❌ Failed to publish '{name}' after {max_attempts} attempts. Halting.")
            sys.exit(1)

        # Allow index propagation delay between publishes
        if not args.dry_run:
            time.sleep(INDEX_PROPAGATION_DELAY)

    print("\n🎉 Publishing complete!")
    print(f"  • Brand new crates published: {new_publishes}")
    print(f"  • Existing crates updated: {existing_publishes}")
    print(f"  • Crates skipped (already at target version): {skipped}")


if __name__ == "__main__":
    main()
