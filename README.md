![Check](https://github.com/janezicmatej/aoc2022/actions/workflows/check.yml/badge.svg)
![Tests](https://github.com/janezicmatej/aoc2022/actions/workflows/tests.yml/badge.svg)
![Clippy](https://github.com/janezicmatej/aoc2022/actions/workflows/clippy.yml/badge.svg)
# Advent-of-Code 2022
*This is a dumbed down version of [fspoettel/advent-of-code-rust](https://github.com/fspoettel/advent-of-code-rust) with some extra features*

## CLI
### Prepare

```sh
# example: `cargo prepare 1`
cargo prepare <day>

# output:
# Created module "src/bin/01.rs"
# Created empty input file "src/inputs/01.txt"
# Created empty example file "src/examples/01.txt"
# ---
# 🎄 Type `cargo solve 01` to run your solution.
```

### Download input
prepare `.env` file
```
cp .env.example .env
```
set `YEAR` to whichever year you are solving for and `TOKEN` to AoC session Cookie.

```sh
# example: `cargo download 1`
cargo download <day>

# output:
# Downloaded input file "src/inputs/01.txt"
```

### Solve
```sh
# example: `cargo solve 01`
cargo solve <day>

# output:
#     Running `target/debug/01`
# 🎄 Part 1 🎄
#
# 6 (elapsed: 37.03µs)
#
# 🎄 Part 2 🎄
#
# 9 (elapsed: 33.18µs)
```
Displayed timings show the raw execution time of your solution without overhead (e.g. file reads). To run an optimized version for benchmarking, append the `--release` flag.


### Solve all

```sh
cargo all

# output:
#     Running `target/release/aoc`
# ----------
# | Day 01 |
# ----------
# 🎄 Part 1 🎄
#
# 0 (elapsed: 170.00µs)
#
# 🎄 Part 2 🎄
#
# 0 (elapsed: 30.00µs)
# <...other days...>
# Total: 0.20ms
```

`all` is an alias for `cargo run`. To run an optimized version for benchmarking, append the `--release` flag.

### Run against test inputs
run all solutions
```sh
cargo test
```
run for a given day
```sh
cargo test --bin <day>
```

