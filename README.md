# Zero To Production / Code (Chapter 7 - Part 1)

<div align="center"><a href="https://zero2prod.com" target="_blank"><img src="https://static-2.gumroad.com/res/gumroad/3629854790655/asset_previews/bc9026cad3ece1746327c1d70218f602/retina/rsz_zero_to_production_punk.png" /></a></div>

[Zero To Production In Rust](https://zero2prod.com) is an opinionated introduction to backend development using Rust.

This repository serves as supplementary material for [the book](https://zero2prod.com/): it hosts snapshots of the codebase of our email newsletter project at end of each chapter.

**This branch is a snapshot of the project at the end of Chapter 7 - Part 1.**

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
