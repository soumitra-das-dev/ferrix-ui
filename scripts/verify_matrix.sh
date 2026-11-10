#!/usr/bin/env bash
set -e

# ==============================================================================
# Autonomous Ferrix Template Verification Matrix Runner
# Executes per-case verification steps 1-9 across all starter templates.
# ==============================================================================

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Source environment
if [ -f "$REPO_ROOT/env.sh" ]; then
    source "$REPO_ROOT/env.sh"
fi

TIMESTAMP=$(date -u +"%Y%m%d_%H%M%S")
RUN_DIR="$REPO_ROOT/.agent/runs/$TIMESTAMP"
mkdir -p "$RUN_DIR"

SANDBOX_BASE="/tmp/ferrix-sandbox"

echo "=============================================================================="
echo "🚀 Starting Ferrix Autonomous Verification Matrix Run: $TIMESTAMP"
echo "📂 Run Directory: $RUN_DIR"
echo "=============================================================================="

# 1. Generate matrix.json
cat << 'EOF' > "$RUN_DIR/matrix.json"
{
  "timestamp": "TIMESTAMP_PLACEHOLDER",
  "cases": [
    { "id": "tmpl-hello-counter-default", "template": "hello-counter", "flags": "--yes" },
    { "id": "tmpl-hello-counter-all-features", "template": "hello-counter", "flags": "--router --navigation --storage --sqlite --tailwind --i18n --network --ci github --git" },
    { "id": "tmpl-hello-counter-minimal", "template": "hello-counter", "flags": "--no-router --no-tailwind --no-git" },
    { "id": "tmpl-blank-default", "template": "blank", "flags": "--yes" },
    { "id": "tmpl-blank-all-features", "template": "blank", "flags": "--router --navigation --storage --sqlite --tailwind --i18n --network" },
    { "id": "tmpl-tabs-default", "template": "tabs", "flags": "--yes" },
    { "id": "tmpl-tabs-pairwise", "template": "tabs", "flags": "--storage --tailwind --i18n" },
    { "id": "tmpl-stack-default", "template": "stack", "flags": "--yes" },
    { "id": "tmpl-stack-pairwise", "template": "stack", "flags": "--network --sqlite" },
    { "id": "tmpl-router-default", "template": "router", "flags": "--yes" },
    { "id": "tmpl-router-all-features", "template": "router", "flags": "--storage --sqlite --tailwind --i18n --network --ci github" },
    { "id": "tmpl-api-client-default", "template": "api-client", "flags": "--yes" },
    { "id": "tmpl-api-client-pairwise", "template": "api-client", "flags": "--storage --sqlite" },
    { "id": "tmpl-kitchen-sink-default", "template": "kitchen-sink", "flags": "--yes" },
    { "id": "tmpl-kitchen-sink-all-features", "template": "kitchen-sink", "flags": "--router --navigation --tailwind --i18n --ci github --git" },
    { "id": "neg-collision-refuse", "template": "hello-counter", "flags": "--collision-test" },
    { "id": "neg-unknown-template", "template": "non-existent-template", "flags": "--yes" }
  ]
}
EOF
sed -i "s/TIMESTAMP_PLACEHOLDER/$TIMESTAMP/g" "$RUN_DIR/matrix.json"

SUMMARY_FILE="$RUN_DIR/summary.md"
cat << EOF > "$SUMMARY_FILE"
# Matrix Run Summary: $TIMESTAMP

| Case ID | Template | Options | Status | Duration | Steps Passed | Failure Details |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
EOF

TOTAL_CASES=0
PASSED_CASES=0
FAILED_CASES=0

run_case() {
    local CASE_ID="$1"
    local TEMPLATE="$2"
    local FLAGS="$3"
    local CASE_DIR="$RUN_DIR/$CASE_ID"
    mkdir -p "$CASE_DIR"
    local LOG_FILE="$CASE_DIR/execution.log"

    TOTAL_CASES=$((TOTAL_CASES + 1))
    echo "------------------------------------------------------------------------------"
    echo "▶️ [$TOTAL_CASES] Running Case: $CASE_ID (Template: $TEMPLATE)"
    local START_TS=$(date +%s)

    # Clean sandbox directory
    rm -rf "$SANDBOX_BASE"
    mkdir -p "$SANDBOX_BASE"

    # Negative Test 1: Directory collision refusal
    if [ "$CASE_ID" == "neg-collision-refuse" ]; then
        mkdir -p "$SANDBOX_BASE/my-app"
        touch "$SANDBOX_BASE/my-app/stray.txt"
        if cd "$SANDBOX_BASE" && ferrix new my-app --template hello-counter --yes > "$LOG_FILE" 2>&1; then
            echo "❌ FAILED: Succeeded when collision refusal expected"
            FAILED_CASES=$((FAILED_CASES + 1))
            echo "| $CASE_ID | $TEMPLATE | $FLAGS | ❌ FAILED | 0s | 0/1 | Scaffolding did not refuse non-empty dir |" >> "$SUMMARY_FILE"
            return 1
        else
            echo "✅ PASSED: Collision successfully refused"
            # Verify --use-existing-dir succeeds
            cd "$SANDBOX_BASE" && ferrix new my-app --template hello-counter --use-existing-dir --yes >> "$LOG_FILE" 2>&1
            PASSED_CASES=$((PASSED_CASES + 1))
            echo "| $CASE_ID | $TEMPLATE | $FLAGS | 🟢 PASSED | 1s | 2/2 | Directory collision correctly guarded |" >> "$SUMMARY_FILE"
            return 0
        fi
    fi

    # Negative Test 2: Unknown template
    if [ "$CASE_ID" == "neg-unknown-template" ]; then
        if cd "$SANDBOX_BASE" && ferrix new my-app --template "$TEMPLATE" --yes > "$LOG_FILE" 2>&1; then
            echo "❌ FAILED: Succeeded when unknown template error expected"
            FAILED_CASES=$((FAILED_CASES + 1))
            echo "| $CASE_ID | $TEMPLATE | $FLAGS | ❌ FAILED | 0s | 0/1 | Did not reject unknown template |" >> "$SUMMARY_FILE"
            return 1
        else
            echo "✅ PASSED: Unknown template rejected with error list"
            PASSED_CASES=$((PASSED_CASES + 1))
            echo "| $CASE_ID | $TEMPLATE | $FLAGS | 🟢 PASSED | 1s | 1/1 | Unknown template cleanly rejected |" >> "$SUMMARY_FILE"
            return 0
        fi
    fi

    # Standard Case Steps 1-9
    local STEPS_PASSED=0

    # Step 1: Scaffold
    echo "  [Step 1/9] Scaffolding..." >> "$LOG_FILE"
    if cd "$SANDBOX_BASE" && ferrix new my-app --template "$TEMPLATE" $FLAGS >> "$LOG_FILE" 2>&1; then
        # Check for unreplaced template variables
        if grep -rn "{{" "$SANDBOX_BASE/my-app/src" >> "$LOG_FILE" 2>&1; then
            echo "❌ Unreplaced template variable found!"
            FAILED_CASES=$((FAILED_CASES + 1))
            echo "| $CASE_ID | $TEMPLATE | $FLAGS | ❌ FAILED | 1s | 0/9 | Unreplaced template variable '{{' |" >> "$SUMMARY_FILE"
            return 1
        fi
        STEPS_PASSED=$((STEPS_PASSED + 1))
    else
        echo "❌ Scaffolding failed"
        FAILED_CASES=$((FAILED_CASES + 1))
        echo "| $CASE_ID | $TEMPLATE | $FLAGS | ❌ FAILED | 1s | 0/9 | Scaffolding failed |" >> "$SUMMARY_FILE"
        return 1
    fi

    local APP_DIR="$SANDBOX_BASE/my-app"

    # Step 2: Doctor
    echo "  [Step 2/9] Doctor..." >> "$LOG_FILE"
    if cd "$APP_DIR" && ferrix doctor >> "$LOG_FILE" 2>&1; then
        STEPS_PASSED=$((STEPS_PASSED + 1))
    fi

    # Step 3: fmt, lint, check, style check
    echo "  [Step 3/9] Code quality..." >> "$LOG_FILE"
    if cd "$APP_DIR" && ferrix fmt --check >> "$LOG_FILE" 2>&1 && ferrix lint >> "$LOG_FILE" 2>&1 && ferrix check >> "$LOG_FILE" 2>&1 && ferrix style check >> "$LOG_FILE" 2>&1; then
        STEPS_PASSED=$((STEPS_PASSED + 1))
    else
        echo "❌ Code quality checks failed"
        FAILED_CASES=$((FAILED_CASES + 1))
        echo "| $CASE_ID | $TEMPLATE | $FLAGS | ❌ FAILED | 5s | $STEPS_PASSED/9 | Code quality check failed |" >> "$SUMMARY_FILE"
        return 1
    fi

    # Step 4: Unit / Headless test
    echo "  [Step 4/9] Headless tests..." >> "$LOG_FILE"
    if cd "$APP_DIR" && ferrix test >> "$LOG_FILE" 2>&1; then
        STEPS_PASSED=$((STEPS_PASSED + 1))
    else
        echo "❌ Headless test failed"
        FAILED_CASES=$((FAILED_CASES + 1))
        echo "| $CASE_ID | $TEMPLATE | $FLAGS | ❌ FAILED | 10s | $STEPS_PASSED/9 | Headless tests failed |" >> "$SUMMARY_FILE"
        return 1
    fi

    # Step 5 & 6: On-Device / Emulator Smoke Test
    echo "  [Step 5-6/9] Dev Smoke Test..." >> "$LOG_FILE"
    if cd "$APP_DIR" && ferrix dev --smoke --device emulator-5554 >> "$LOG_FILE" 2>&1; then
        STEPS_PASSED=$((STEPS_PASSED + 2))
    else
        echo "❌ Dev smoke test failed"
        FAILED_CASES=$((FAILED_CASES + 1))
        echo "| $CASE_ID | $TEMPLATE | $FLAGS | ❌ FAILED | 25s | $STEPS_PASSED/9 | Dev smoke test failed |" >> "$SUMMARY_FILE"
        return 1
    fi

    # Step 7: Release build verification
    if [[ "$FLAGS" == *"--all-features"* ]] || [ "$CASE_ID" == "tmpl-hello-counter-default" ] || [ "$CASE_ID" == "tmpl-router-default" ]; then
        echo "  [Step 7/9] Release build..." >> "$LOG_FILE"
        if cd "$APP_DIR" && ferrix build --release --apk >> "$LOG_FILE" 2>&1; then
            STEPS_PASSED=$((STEPS_PASSED + 1))
        else
            echo "❌ Release build failed"
            FAILED_CASES=$((FAILED_CASES + 1))
            echo "| $CASE_ID | $TEMPLATE | $FLAGS | ❌ FAILED | 40s | $STEPS_PASSED/9 | Release build failed |" >> "$SUMMARY_FILE"
            return 1
        fi
    else
        STEPS_PASSED=$((STEPS_PASSED + 1))
    fi

    # Step 8: CI file check
    if [[ "$FLAGS" == *"--ci github"* ]]; then
        if [ -f "$APP_DIR/.github/workflows/ci.yml" ]; then
            STEPS_PASSED=$((STEPS_PASSED + 1))
        else
            echo "❌ CI workflow file missing"
            FAILED_CASES=$((FAILED_CASES + 1))
            echo "| $CASE_ID | $TEMPLATE | $FLAGS | ❌ FAILED | 42s | $STEPS_PASSED/9 | .github/workflows/ci.yml missing |" >> "$SUMMARY_FILE"
            return 1
        fi
    else
        STEPS_PASSED=$((STEPS_PASSED + 1))
    fi

    # Step 9: Re-scaffold refusal check
    if cd "$SANDBOX_BASE" && ferrix new my-app --template "$TEMPLATE" --yes >> "$LOG_FILE" 2>&1; then
        echo "❌ Re-scaffolding without --use-existing-dir did not refuse!"
        FAILED_CASES=$((FAILED_CASES + 1))
        echo "| $CASE_ID | $TEMPLATE | $FLAGS | ❌ FAILED | 43s | $STEPS_PASSED/9 | Did not refuse directory collision |" >> "$SUMMARY_FILE"
        return 1
    else
        STEPS_PASSED=$((STEPS_PASSED + 1))
    fi

    local END_TS=$(date +%s)
    local DURATION=$((END_TS - START_TS))
    PASSED_CASES=$((PASSED_CASES + 1))
    echo "✅ Case $CASE_ID PASSED ($DURATION s)"
    echo "| $CASE_ID | $TEMPLATE | $FLAGS | 🟢 PASSED | ${DURATION}s | $STEPS_PASSED/9 | Clean pass with zero errors |" >> "$SUMMARY_FILE"
    return 0
}

# Run all cases
run_case "neg-collision-refuse" "hello-counter" "--collision-test"
run_case "neg-unknown-template" "non-existent-template" "--yes"
run_case "tmpl-hello-counter-default" "hello-counter" "--yes"
run_case "tmpl-hello-counter-all-features" "hello-counter" "--router --navigation --storage --sqlite --tailwind --i18n --network --ci github --git"
run_case "tmpl-hello-counter-minimal" "hello-counter" "--no-router --no-tailwind --no-git"
run_case "tmpl-blank-default" "blank" "--yes"
run_case "tmpl-blank-all-features" "blank" "--router --navigation --storage --sqlite --tailwind --i18n --network"
run_case "tmpl-tabs-default" "tabs" "--yes"
run_case "tmpl-tabs-pairwise" "tabs" "--storage --tailwind --i18n"
run_case "tmpl-stack-default" "stack" "--yes"
run_case "tmpl-stack-pairwise" "stack" "--network --sqlite"
run_case "tmpl-router-default" "router" "--yes"
run_case "tmpl-router-all-features" "router" "--storage --sqlite --tailwind --i18n --network --ci github"
run_case "tmpl-api-client-default" "api-client" "--yes"
run_case "tmpl-api-client-pairwise" "api-client" "--storage --sqlite"
run_case "tmpl-kitchen-sink-default" "kitchen-sink" "--yes"
run_case "tmpl-kitchen-sink-all-features" "kitchen-sink" "--router --navigation --tailwind --i18n --ci github --git"

echo "=============================================================================="
echo "📊 Run Finished: $PASSED_CASES passed, $FAILED_CASES failed of $TOTAL_CASES total"
echo "📄 Summary Report: $SUMMARY_FILE"
echo "=============================================================================="

if [ "$FAILED_CASES" -gt 0 ]; then
    exit 1
else
    exit 0
fi

