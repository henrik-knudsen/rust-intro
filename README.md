# Introduction to Rust (2022)

Inspired by and partially based on: <https://github.com/arild/rust-workshop>

## Installing

**rustup** is an installer for the systems programming language Rust. 

- Navigate to <https://rustup.rs/>
- Download rustup executable
- Run it to install rustc (compiler), cargo (package manager) and other dependencies.

In order to check that everything is set up correctly afterwards (current latest version is 1.64.0):

``rustc --version``

``cargo --version``

## Learning resources

- [The Book](https://doc.rust-lang.org/book/index.html) - The most comprehensive resource for learning Rust. Well written, with lots of example code.
- [Rustlings](https://github.com/rust-lang/rustlings) - Exercises to go with the chapters of "the book". Put the learning material into practice. 
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/index.html) - Alternative to rustlings, available online.

## IDE

- **Visual studio code**, with ``rust analyzer`` plugin.

OR

- **IntelliJ**, with IntelliJ rust plugin (may require license for the specific IDE you want to use, IntelliJ Idea or CLion)

Vscode with rust analyzer plugin provides a good developer experience and can be a nice starting point.

## Fundamentals

A non-comprehensive, lightning course in rust syntax and language features. It is organized into three parts, following the structure of the slides. Part 1 and 2 cover relatively core concepts of the language, while part 3 shows of some concurrency concepts (optional).

Each part has a set of tests. Run ```cargo test``` within the ``fundamentals`` folder to run all the tests. To only run tests for a specific part, run ``cargo test part1``, for example.

## Rustlings

``rustlings`` contains small exercises to get you used to reading and writing Rust code. It is a rather comprehensive collection of exercises, taking you from ``let x = 5;`` all the way to concurrency primitives and macros.

(This project contains the ``rustlings`` git repository as a submodule. If you did not clone recursively, re-clone this repository using ``git clone --recurse-submodules``.)




### Installing the rustlings executable

The project comes with it's own executable, which helps with running the exercises. Install it by:

```bash
cd rustlings/
cargo install --force --path .
```

### Enabling `rust-analyzer` (optional, but recommended)

Run the command `rustlings lsp` which will generate a `rust-project.json` at the root of the project, this allows [rust-analyzer](https://rust-analyzer.github.io/) to parse each exercise.

### Doing exercises

The exercises are sorted by topic and can be found in the subdirectory `rustlings/exercises/<topic>`. For every topic there is an additional README file with some resources to get you started on the topic. We really recommend that you have a look at them before you start.

The task is simple. Most exercises contain an error that keeps them from compiling, and it's up to you to fix it! Some exercises are also run as tests, but rustlings handles them all the same. To run the exercises in the recommended order, execute:

```bash
rustlings watch
```

This will try to verify the completion of every exercise in a predetermined order (what we think is best for newcomers). It will also rerun automatically every time you change a file in the `exercises/` directory. If you want to only run it once, you can use:

For more commands, see ``rustlings`` [README](rustlings/README.md).

