# Zero To Production / Code

<div align="center"><a href="https://zero2prod.com" target="_blank"><img src="https://www.zero2prod.com/assets/img/zero2prod_banner.webp" /></a></div>

[Zero To Production In Rust](https://zero2prod.com) is an opinionated introduction to backend development using Rust.

This repository serves as supplementary material for [the book](https://zero2prod.com/): it hosts snapshots of the codebase of our email newsletter project at end of each chapter.

## Chapter snapshots

The `master` branch (where you are right now!) shows the project at the end of the last published chapter _(Chapter 8, right now)_.

You can browse the project at the end of previous chapters by switching to their dedicated branches:

- [Chapter 3, Part 0](https://github.com/LukeMathWalker/zero-to-production/tree/root-chapter-03-part0)
- [Chapter 3, Part 1](https://github.com/LukeMathWalker/zero-to-production/tree/root-chapter-03-part1)
- [Chapter 4](https://github.com/LukeMathWalker/zero-to-production/tree/root-chapter-04)
- [Chapter 5](https://github.com/LukeMathWalker/zero-to-production/tree/root-chapter05)
- [Chapter 6, Part 0](https://github.com/LukeMathWalker/zero-to-production/tree/root-chapter-06)
- [Chapter 6, Part 1](https://github.com/LukeMathWalker/zero-to-production/tree/root-chapter-06-part1)
- [Chapter 7, Part 0](https://github.com/LukeMathWalker/zero-to-production/tree/root-chapter-07-part0)
- [Chapter 7, Part 1](https://github.com/LukeMathWalker/zero-to-production/tree/root-chapter-07-part1)
- [Chapter 7, Part 2](https://github.com/LukeMathWalker/zero-to-production/tree/root-chapter-07-part2)
- [Chapter 8](https://github.com/LukeMathWalker/zero-to-production/tree/root-chapter-08-part0)
- [Chapter 9](https://github.com/LukeMathWalker/zero-to-production/tree/root-chapter-09)

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

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>