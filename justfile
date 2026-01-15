set dotenv-load := true

os := os()
arch := arch()
test_compiler_bin := env_var("TEST_COMPILER_PATH")
path_to_binary:= env_var("PATH_TO_BINARY")

prefix := if os == "macos" {
    if arch == "x86_64" {
       env_var_or_default("SHELL", "sh") + " -cu"
    } else {
        "arch -x86_64 " + env_var_or_default("SHELL", "sh") + " -cu"
    }
} else {
    env_var_or_default("SHELL", "sh") + " -cu"
}

default:
    @just --list

# Show's the command prefix for all shell commands
show-prefix:
    {{ prefix }} 'uname -m'

# Verify all requirements for test_comiler are met
check-test-compiler-setup:
   {{ prefix }} '{{ test_compiler_bin }} --check-setup'

# Clean build artifacts
clean:
    @echo "Cleaning /target directory ..."
    rm -rf target
    @echo "Cleaning /bin directory ..."
    mkdir -p bin
    rm -rf bin

# Build `rcc` binary. Modes={debug, release}
cli-build mode="debug":
    @echo "Building rcc cli ..."
    cargo build {{ if mode == "release" { "--release" } else { "" } }} -p cli
    @echo "Copying to ./bin/cli ..."
    mkdir -p bin
    cp {{ if mode == "release" { "./target/release/cli" } else { "./target/debug/cli" } }} {{ path_to_binary }}

# Run's tests from the book's test suite. Pass in chapter and stage flags.
test-suite +args:
    @echo "Running tests for : {{ args }}"
    {{ prefix }} '{{ test_compiler_bin }} {{ path_to_binary }} {{ args }}'
