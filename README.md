# Advent of Code

<p align="left">
  <a href="https://github.com/AndrejOrsula/aoc/actions/workflows/rust.yml">   <img alt="Rust"   src="https://github.com/AndrejOrsula/aoc/actions/workflows/rust.yml/badge.svg"></a>
  <a href="https://github.com/AndrejOrsula/aoc/actions/workflows/docker.yml"> <img alt="Docker" src="https://github.com/AndrejOrsula/aoc/actions/workflows/docker.yml/badge.svg"></a>
</p>

My solutions to [Advent of Code](https://adventofcode.com) puzzles.

## Edition

### [Advent of Code 2023](https://adventofcode.com/2023)

> All benchmarks are run on a *Dell Precision 5550* laptop with an *Intel Core i7-10875H* CPU.

<table>
<tr><th>Day</th><th>Part 1 Performance</th><th>Part 2 Performance</th></tr>
<tr><td>

|   D   | Puzzle                                                        |               Code               |
| :---: | ------------------------------------------------------------- | :------------------------------: |
|   1   | [Trebuchet?!](https://adventofcode.com/2023/day/1)            | [`day1.rs`](aoc2023/src/day1.rs) |
|   2   | [Cube Conundrum](https://adventofcode.com/2023/day/2)         | [`day2.rs`](aoc2023/src/day2.rs) |
|   3   | [Gear Ratios](https://adventofcode.com/2023/day/3)            | [`day3.rs`](aoc2023/src/day3.rs) |
|   4   | [Scratchcards](https://adventofcode.com/2023/day/4)           | [`day4.rs`](aoc2023/src/day4.rs) |
|   5   | [If You Give A Seed ...](https://adventofcode.com/2023/day/5) | [`day5.rs`](aoc2023/src/day5.rs) |
|   6   | [Wait For It](https://adventofcode.com/2023/day/6)            | [`day6.rs`](aoc2023/src/day6.rs) |
|   7   | [Camel Cards](https://adventofcode.com/2023/day/7)            | [`day7.rs`](aoc2023/src/day7.rs) |
|   8   | [Haunted Wasteland](https://adventofcode.com/2023/day/8)      | [`day8.rs`](aoc2023/src/day8.rs) |

</td><td>

| Generator |  Runner  |
| :-------: | :------: |
| 3.933 µs  | 45.46 µs |
| 493.8 µs  | 1.063 µs |
| 1.146 µs  | 429.6 µs |
| 58.79 µs  | 27.72 µs |
| 19.61 µs  | 5.510 µs |
| 1.432 µs  | 0.345 µs |
| 117.3 µs  | 70.08 µs |
| 168.5 µs  | 819.3 µs |

</td><td>

| Generator |  Runner  |
| :-------: | :------: |
| 0.422 µs  | 776.7 µs |
| 298.0 µs  | 1.108 µs |
| 0.419 µs  | 308.1 µs |
| 53.39 µs  | 28.44 µs |
| 16.38 µs  | 19.364 s |
| 0.513 µs  | 0.650 µs |
| 136.1 µs  | 65.87 µs |
| 118.8 µs, | 2.450 ms |

</td></tr>
</table>

## Instructions

<details open>
<summary><h3><a href="#-rust"><img src="https://rustacean.net/assets/rustacean-flat-noshadow.svg" width="16" height="16"></a> Rust</h3></summary>

#### <a href="#-test-the-correctness-of-solutions"><img src="https://www.svgrepo.com/show/271355/rocket-ship-rocket.svg" width="14" height="14"></a> Get Answers and Run Performance Benchmarks

Thanks to [`cargo-aoc`](https://github.com/gobanos/cargo-aoc), answers and performance benchmarks for all days are obtainable by running the main binary.

```bash
cargo run --release
```

#### <a href="#-test-the-correctness-of-solutions"><img src="https://www.svgrepo.com/show/269868/lab.svg" width="14" height="14"></a> Test the Correctness of Solutions

All days also include tests using sample inputs from the puzzle descriptions.

```bash
cargo test
```

</details>

<details>
<summary><h3><a href="#-docker"><img src="https://www.svgrepo.com/show/448221/docker.svg" width="16" height="16"></a> Docker</h3></summary>

> To install [Docker](https://docs.docker.com/get-docker) on your system, you can run [`.docker/host/install_docker.bash`](.docker/host/install_docker.bash) to configure Docker with NVIDIA GPU support.
>
> ```bash
> .docker/host/install_docker.bash
> ```

#### Build Image

To build a new Docker image from [`Dockerfile`](Dockerfile), you can run [`.docker/build.bash`](.docker/build.bash) as shown below.

```bash
.docker/build.bash ${TAG:-latest} ${BUILD_ARGS}
```

#### Run Container

To run the Docker container, you can use [`.docker/run.bash`](.docker/run.bash) as shown below.

```bash
.docker/run.bash ${TAG:-latest} ${CMD}
```

#### Run Dev Container

To run the Docker container in a development mode (source code mounted as a volume), you can use [`.docker/dev.bash`](.docker/dev.bash) as shown below.

```bash
.docker/dev.bash ${TAG:-latest} ${CMD}
```

As an alternative, VS Code users familiar with [Dev Containers](https://code.visualstudio.com/docs/devcontainers/containers) can modify the included [`.devcontainer/devcontainer.json`](.devcontainer/devcontainer.json) to their needs. For convenience, [`.devcontainer/open.bash`](.devcontainer/open.bash) script is available to open this repository as a Dev Container in VS Code.

```bash
.devcontainer/open.bash
```

#### Join Container

To join a running Docker container from another terminal, you can use [`.docker/join.bash`](.docker/join.bash) as shown below.

```bash
.docker/join.bash ${CMD:-bash}
```

</details>

## License

This project is dual-licensed to be compatible with the Rust project, under either the [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE) licenses.
