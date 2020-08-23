# Zero To Production / Code

This repository complements the [Zero To Production book](https://www.lpalmieri.com/posts/2020-05-24-zero-to-production-0-foreword/): it hosts snapshots of the codebase of our email newsletter project at end of each chapter.

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
