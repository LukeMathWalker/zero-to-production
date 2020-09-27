# Zero To Production / Code

![Book cover](https://www.zero2prod.com/assets/img/book2.png)

[Zero To Production In Rust](https://zero2prod.com) aims to be an opinionated introduction to backend development using Rust.

This repository serves as supplementary material for [the book](https://zero2prod.com/): it hosts snapshots of the codebase of our email newsletter project at end of each chapter.

It is structured as a `cargo` workspace: running `cargo build` will build the codebase for **all** chapters.
If you want to build/test/run the code for a _specific_ chapter, just move into its folder! E.g.:
```bash
# Run tests for the first part of Chapter 3
cd chapter03-0
cargo test
```
Alternatively, from the top-level folder, you can specify the binary you are interested into:
```bash
# Run the application as it is at end of the first part of Chapter 3
cargo run --bin chapter03-0
```
