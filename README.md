# rcc

Implementing [Writing a C Compiler by Nora Sandler](https://nostarch.com/writing-c-compiler) in Rust.

Currently in progress.

## Requirements

Similar to the book, only Linux and Mac are supported. For Windows, run in WSL.

- `python3` `>=3.8`
- `gcc, gdb` for Linux
- `clang` for Mac
- `rustc`
- [just](https://github.com/casey/just) for task runner

The commands are written to be OS agnostic, but I have only tested them on MacOS.

## Commands

Run `just` to see list of all available commands. **Requires the setup to be completed**

```shell
just 
```

## Setup

Create a `.env` file and set the below values. These values are used in `justfile` to setup build, test artifacts. Check out `example.env`.

```shell
TEST_COMPILER_PATH=
PATH_TO_BINARY=
```

- TEST_COMPILER_PATH

Clone [Writing a C Compiler Test Suite](https://github.com/nlsandler/writing-a-c-compiler-tests) locally. Specify the path to this repository add the suffix for `test_compiler`.

Example. All paths are relative. 

```shell
mkdir tests && cd tests
git clone --depth=1 https://github.com/nlsandler/writing-a-c-compiler-tests.git
```

```
TEST_COMPILER_PATH='./tests/writing-a-c-compiler-tests/test_compiler'
```

- PATH_TO_BINARY

Path to the directory where the binary will be created.

Name of the binary is `rcc`.

```
PATH_TO_BINARY='./bin'
```

```shell
$ just build 
./bin/rcc 
```

## Instructions

```shell
$ just run debug ./tests/main.c
```

Run the book's test suite.

```shell
just test-suite --chapter 1 --stage lex
# expands to arch -x86_64 /bin/zsh -cu './tests/writing-a-c-compiler-tests/test_compiler ./bin/rcc --chapter 1 --stage lex'
```
