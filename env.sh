#!/usr/bin/env bash
# ==============================================================================
# Ferrix Development Environment Setup
# Portable toolchain environment detection for JDK, Android SDK, NDK, and Cargo.
# ==============================================================================

USER_HOME="${HOME:-~}"

# 1. Java / JDK 21+ Detection
if [ -z "$JAVA_HOME" ] || [ ! -d "$JAVA_HOME" ]; then
    for candidate in \
        "$USER_HOME/.local/share/jvm"/jdk* \
        "$USER_HOME/.sdkman/candidates/java"/current \
        /usr/lib/jvm/java-21* \
        /usr/lib/jvm/default-java \
        /Library/Java/JavaVirtualMachines/*/Contents/Home; do
        if [ -d "$candidate" ]; then
            export JAVA_HOME="$candidate"
            break
        fi
    done
fi

# 2. Android SDK Detection
if [ -z "$ANDROID_HOME" ] || [ ! -d "$ANDROID_HOME" ]; then
    for candidate in \
        "$USER_HOME/.local/share/android-sdk" \
        "$USER_HOME/Android/Sdk" \
        "$USER_HOME/Library/Android/sdk" \
        /usr/lib/android-sdk; do
        if [ -d "$candidate" ]; then
            export ANDROID_HOME="$candidate"
            export ANDROID_SDK_ROOT="$candidate"
            break
        fi
    done
fi

# 3. Android NDK Detection (pick latest version if not set)
if [ -z "$ANDROID_NDK_HOME" ] || [ ! -d "$ANDROID_NDK_HOME" ]; then
    if [ -n "$ANDROID_HOME" ] && [ -d "$ANDROID_HOME/ndk" ]; then
        LATEST_NDK=$(ls -1d "$ANDROID_HOME/ndk"/* 2>/dev/null | sort -V | tail -n 1)
        if [ -d "$LATEST_NDK" ]; then
            export ANDROID_NDK_HOME="$LATEST_NDK"
            export NDK_HOME="$LATEST_NDK"
        fi
    fi
fi

# 4. Construct PATH dynamically
add_to_path() {
    if [ -d "$1" ]; then
        case ":$PATH:" in
            *":$1:"*) ;; # already in PATH
            *) PATH="$1:$PATH" ;;
        esac
    fi
}

# Android emulator & tools
if [ -n "$ANDROID_HOME" ]; then
    add_to_path "$ANDROID_HOME/emulator"
    add_to_path "$ANDROID_HOME/platform-tools"
    for cmdline_dir in "$ANDROID_HOME/cmdline-tools/latest/bin" "$ANDROID_HOME/cmdline-tools"/*/bin; do
        if [ -d "$cmdline_dir" ]; then
            add_to_path "$cmdline_dir"
            break
        fi
    done
fi

# JDK binary tools
if [ -n "$JAVA_HOME" ]; then
    add_to_path "$JAVA_HOME/bin"
fi

# Gradle binary (if custom local install exists)
for gradle_dir in "$USER_HOME/.local/share/gradle"/gradle-*/bin; do
    if [ -d "$gradle_dir" ]; then
        add_to_path "$gradle_dir"
        break
    fi
done

# Rust / Cargo
add_to_path "$USER_HOME/.cargo/bin"

export PATH
