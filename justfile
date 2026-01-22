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

[doc("""Show's the command prefix for all shell commands""")]
show-prefix:
    {{ prefix }} 'uname -m'

[doc("""Verify all requirements for test_comiler are met""")]
check-test-compiler-setup:
   {{ prefix }} '{{ test_compiler_bin }} --check-setup'

[doc("""Clean build artifacts""")]
clean:
    @echo "Cleaning /target directory ..."
    rm -rf target
    @echo "Cleaning {{ path_to_binary }} directory ..."
    rm -rf {{ path_to_binary }}
    rm -rf {{ path_to_binary }}/

[doc("""
Builds rcc binary.
Usage: just build <debug|release> (defaults to debug)
""")]
build mode="debug":
    @echo "Building cli binary ..."
    cargo build {{ if mode == "release" { "--release" } else { "" } }} -p cli
    @echo "Copying to {{ path_to_binary }} ..."
    mkdir -p {{ path_to_binary }}
    cp {{ if mode == "release" { "./target/release/cli" } else { "./target/debug/cli" } }} {{ path_to_binary }}/rcc

[doc("""
Run's rcc binary.
Usage: just run <debug|info|other log levels>)
""")]
run level +args:
    {{ prefix }} 'RUST_LOG={{ level }} {{ path_to_binary}}/rcc {{ args }}'

# Run's tests from the book's test suite. Pass in chapter and stage flags.
test-suite +args:
    @echo "Running tests for : {{ args }}"
    {{ prefix }} '{{ test_compiler_bin }} {{ path_to_binary }}/rcc {{ args }}'
