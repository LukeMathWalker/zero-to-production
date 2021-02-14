# Zero To Production / Code

<div align="center"><a href="https://zero2prod.com" target="_blank"><img src="https://static-2.gumroad.com/res/gumroad/3629854790655/asset_previews/bc9026cad3ece1746327c1d70218f602/retina/rsz_zero_to_production_punk.png" /></a></div>

[Zero To Production In Rust](https://zero2prod.com) is an opinionated introduction to backend development using Rust.

This repository serves as supplementary material for [the book](https://zero2prod.com/): it hosts snapshots of the codebase of our email newsletter project at end of each chapter.

## Chapter snapshots

The `master` branch (where you are right now!) shows the project at the end of the last published chapter _(Chapter 7 Part 1, right now)_.

You can browse the project at end of previous chapters by switching to their dedicated branches:

- [Chapter 3, Part 0](https://github.com/LukeMathWalker/zero-to-production/tree/root-chapter-03-part0)
- [Chapter 3, Part 1](https://github.com/LukeMathWalker/zero-to-production/tree/root-chapter-03-part1)
- [Chapter 4](https://github.com/LukeMathWalker/zero-to-production/tree/root-chapter-04)
- [Chapter 5](https://github.com/LukeMathWalker/zero-to-production/tree/root-chapter-05)
- [Chapter 6, Part 0](https://github.com/LukeMathWalker/zero-to-production/tree/root-chapter-06)
- [Chapter 6, Part 1](https://github.com/LukeMathWalker/zero-to-production/tree/root-chapter-06-part1)
- [Chapter 7, Part 0](https://github.com/LukeMathWalker/zero-to-production/tree/root-chapter-07-part0)
- [Chapter 7, Part 1](https://github.com/LukeMathWalker/zero-to-production/tree/root-chapter-07-part1)

## Pre-requisite

You'll need to install:

- [Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://docs.docker.com/get-docker/)

Launch a (migrated) Postgres database via Docker:

```bash
./scripts/init_db.sh
```

## How to build

Using `cargo`:

```bash
cargo build
```

## How to test

Using `cargo`:

```bash
cargo test 
```
