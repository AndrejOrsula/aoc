# Advent of Code

<p align="left">
  <a href="https://github.com/AndrejOrsula/aoc/actions/workflows/rust.yml">   <img alt="Rust"   src="https://github.com/AndrejOrsula/aoc/actions/workflows/rust.yml/badge.svg"></a>
  <a href="https://github.com/AndrejOrsula/aoc/actions/workflows/docker.yml"> <img alt="Docker" src="https://github.com/AndrejOrsula/aoc/actions/workflows/docker.yml/badge.svg"></a>
</p>

My solutions to [Advent of Code](https://adventofcode.com) puzzles.

- [Edition](#edition)
  - [Advent of Code 2023](#advent-of-code-2023)
- [Instructions](#instructions)
  - [ Usage in Rust](#-usage-in-rust)
    - [ Get Answers and Run Performance Benchmarks](#-get-answers-and-run-performance-benchmarks)
    - [ Test the Correctness of Solutions](#-test-the-correctness-of-solutions)
  - [ Docker](#-docker)
    - [Build Image](#build-image)
    - [Run Container](#run-container)
    - [Run Dev Container](#run-dev-container)
    - [Join Container](#join-container)
- [License](#license)

## Edition

### [Advent of Code 2023](https://adventofcode.com/2023)

> All benchmarks are run on a *Dell Precision 5550* laptop with an *Intel Core i7-10875H* CPU.

<table>
<tr><th>Solution</th><th>Part 1 Performance</th><th>Part 2 Performance</th></tr>
<tr><td>

|                          Day                          |               Code               |                   Input                   |
| :---------------------------------------------------: | :------------------------------: | :---------------------------------------: |
| [1: Trebuchet?!](https://adventofcode.com/2023/day/1) | [`day1.rs`](aoc2023/src/day1.rs) | [`day1.txt`](aoc2023/input/2023/day1.txt) |

</td><td>

| Generator | Runner  |
| :-------: | :-----: |
|  4.21 µs  | 47.4 µs |

</td><td>

| Generator | Runner |
| :-------: | :----: |
|  422 ns   | 753 µs |

</td></tr>
</table>

## Instructions

### <a href="#-usage-in-rust"><img src="https://rustacean.net/assets/rustacean-flat-noshadow.svg" width="16" height="16"></a> Usage in Rust

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

### <a href="#-docker"><img src="https://www.svgrepo.com/show/448221/docker.svg" width="16" height="16"></a> Docker

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

## License

This project is dual-licensed to be compatible with the Rust project, under either the [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE) licenses.
