# Zero To Production In Rust

<div align="center"><a href="https://zero2prod.com" target="_blank"><img src="https://www.zero2prod.com/assets/img/zero2prod_banner.webp" /></a></div>

[Zero To Production In Rust](https://zero2prod.com) is an opinionated introduction to backend development using Rust.

This repository serves as supplementary material for [the book](https://zero2prod.com/): it hosts several snapshots of the codebase for our email newsletter project as it evolves throughout the book.

## Chapter snapshots

The [`main`](https://github.com/LukeMathWalker/zero-to-production) branch shows the project at the end of the book.

You can browse the project at the end of previous chapters by switching to their dedicated branches:

- [Chapter 3, Part 0](https://github.com/LukeMathWalker/zero-to-production/tree/root-chapter-03-part0)
- [Chapter 3, Part 1](https://github.com/LukeMathWalker/zero-to-production/tree/root-chapter-03-part1)
- [Chapter 4](https://github.com/LukeMathWalker/zero-to-production/tree/root-chapter-04)
- [Chapter 5](https://github.com/LukeMathWalker/zero-to-production/tree/root-chapter-05)
- [Chapter 6, Part 0](https://github.com/LukeMathWalker/zero-to-production/tree/root-chapter-06-part0)
- [Chapter 6, Part 1](https://github.com/LukeMathWalker/zero-to-production/tree/root-chapter-06-part1)
- [Chapter 7, Part 0](https://github.com/LukeMathWalker/zero-to-production/tree/root-chapter-07-part0)
- [Chapter 7, Part 1](https://github.com/LukeMathWalker/zero-to-production/tree/root-chapter-07-part1)
- [Chapter 7, Part 2](https://github.com/LukeMathWalker/zero-to-production/tree/root-chapter-07-part2)
- [Chapter 8](https://github.com/LukeMathWalker/zero-to-production/tree/root-chapter-08)
- [Chapter 9](https://github.com/LukeMathWalker/zero-to-production/tree/root-chapter-09)
- [Chapter 10, Part 0](https://github.com/LukeMathWalker/zero-to-production/tree/root-chapter-10-part0)
- [Chapter 10, Part 1](https://github.com/LukeMathWalker/zero-to-production/tree/root-chapter-10-part1)
- [Chapter 10, Part 2](https://github.com/LukeMathWalker/zero-to-production/tree/root-chapter-10-part2)
- [Chapter 10, Part 3](https://github.com/LukeMathWalker/zero-to-production/tree/root-chapter-10-part3)
- [Chapter 11](https://github.com/LukeMathWalker/zero-to-production/tree/root-chapter-11)

## Pre-requisites

You'll need to install:

- [Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://docs.docker.com/get-docker/)

There are also some OS-specific requirements.

### Windows
  
```bash
cargo install -f cargo-binutils
rustup component add llvm-tools-preview
```

### Linux

```bash
# Ubuntu 
sudo apt-get install lld clang
# Arch 
sudo pacman -S lld clang
```

### MacOS

```bash
brew install michaeleisel/zld/zld
```

## How to build

Launch a (migrated) Postgres database via Docker:

```bash
./scripts/init_db.sh
```

Launch a Redis instance via Docker:

```bash
./scripts/init_redis.sh
```

Launch `cargo`:

```bash
cargo build
```

## How to test

Launch a (migrated) Postgres database via Docker:

```bash
./scripts/init_db.sh
```

Launch a Redis instance via Docker:

```bash
./scripts/init_redis.sh
```

Launch `cargo`:

```bash
cargo test 
```
