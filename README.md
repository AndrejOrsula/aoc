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

|   D   | Puzzle                                                          |               Code               |
| :---: | --------------------------------------------------------------- | :------------------------------: |
|   1   | [Trebuchet?!](https://adventofcode.com/2023/day/1)              |  [day1.rs](aoc2023/src/day1.rs)  |
|   2   | [Cube Conundrum](https://adventofcode.com/2023/day/2)           |  [day2.rs](aoc2023/src/day2.rs)  |
|   3   | [Gear Ratios](https://adventofcode.com/2023/day/3)              |  [day3.rs](aoc2023/src/day3.rs)  |
|   4   | [Scratchcards](https://adventofcode.com/2023/day/4)             |  [day4.rs](aoc2023/src/day4.rs)  |
|   5   | [If You Give A Seed ...](https://adventofcode.com/2023/day/5)   |  [day5.rs](aoc2023/src/day5.rs)  |
|   6   | [Wait For It](https://adventofcode.com/2023/day/6)              |  [day6.rs](aoc2023/src/day6.rs)  |
|   7   | [Camel Cards](https://adventofcode.com/2023/day/7)              |  [day7.rs](aoc2023/src/day7.rs)  |
|   8   | [Haunted Wasteland](https://adventofcode.com/2023/day/8)        |  [day8.rs](aoc2023/src/day8.rs)  |
|   9   | [Mirage Maintenance](https://adventofcode.com/2023/day/9)       |  [day9.rs](aoc2023/src/day9.rs)  |
|  10   | [Pipe Maze](https://adventofcode.com/2023/day/10)               | [day10.rs](aoc2023/src/day10.rs) |
|  11   | [Cosmic Expansion](https://adventofcode.com/2023/day/11)        | [day11.rs](aoc2023/src/day11.rs) |
|  12   | [Hot Springs](https://adventofcode.com/2023/day/12)             | [day12.rs](aoc2023/src/day12.rs) |
|  13   | [Point of Incidence](https://adventofcode.com/2023/day/13)      | [day13.rs](aoc2023/src/day13.rs) |
|  14   | [Parabolic Reflector ...](https://adventofcode.com/2023/day/14) | [day14.rs](aoc2023/src/day14.rs) |
|  15   | [Lens Library](https://adventofcode.com/2023/day/15)            | [day15.rs](aoc2023/src/day15.rs) |
|  16   | [The Floor Will Be Lava](https://adventofcode.com/2023/day/16)  | [day16.rs](aoc2023/src/day16.rs) |
|  17   | [Clumsy Crucible](https://adventofcode.com/2023/day/17)         | [day17.rs](aoc2023/src/day17.rs) |
|  18   | [Lavaduct Lagoon](https://adventofcode.com/2023/day/18)         | [day18.rs](aoc2023/src/day18.rs) |
|  19   | [Aplenty](https://adventofcode.com/2023/day/19)                 | [day19.rs](aoc2023/src/day19.rs) |
|  20   | [Pulse Propagation](https://adventofcode.com/2023/day/20)       | [day20.rs](aoc2023/src/day20.rs) |
|  21   | [Step Counter](https://adventofcode.com/2023/day/21)            | [day21.rs](aoc2023/src/day21.rs) |
|  22   | [Sand Slabs](https://adventofcode.com/2023/day/22)              | [day22.rs](aoc2023/src/day22.rs) |
|  23   | [A Long Walk](https://adventofcode.com/2023/day/23)             | [day23.rs](aoc2023/src/day23.rs) |
|  24   | [Never Tell Me The ...](https://adventofcode.com/2023/day/24)   | [day24.rs](aoc2023/src/day24.rs) |

</td><td>

| Generator |  Runner  |
| :-------: | :------: |
| 4.408 µs  | 80.30 µs |
| 112.7 µs  | 1.056 µs |
| 5.884 µs  | 508.0 µs |
| 92.21 µs  | 27.78 µs |
| 46.67 µs  | 5.350 µs |
| 16.90 µs  | 0.248 µs |
| 170.1 µs  | 76.89 µs |
| 181.6 µs  | 323.7 µs |
| 165.9 µs  | 38.10 µs |
| 159.7 µs  | 183.6 µs |
| 7.197 ms  | 1.085 ms |
| 276.1 µs  | 477.4 µs |
| 58.49 µs  | 359.9 µs |
| 87.00 µs  | 61.28 µs |
| 195.8 µs  | 24.98 µs |
| 33.91 µs  | 622.1 µs |
| 50.64 µs  | 78.82 ms |
| 30.70 µs  | 1.981 µs |
| 228.0 µs  | 31.74 µs |
| 33.73 µs  | 4.654 ms |
| 498.8 µs  | 6.057 ms |
| 254.8 µs  | 41.48 ms |
| 101.8 µs  | 2.554 ms |
| 100.3 µs  | 1.486 ms |

</td><td>

| Generator |  Runner  |
| :-------: | :------: |
| 0.558 µs  | 930.2 µs |
| 126.6 µs  | 1.347 µs |
| 0.462 µs  | 437.1 µs |
| 113.4 µs  | 27.97 µs |
| 47.31 µs  | 22.520 s |
| 0.666 µs  | 0.659 µs |
| 177.0 µs  | 66.30 µs |
| 153.8 µs  | 1.069 ms |
| 119.2 µs  | 45.94 µs |
| 163.3 µs  | 222.8 µs |
| 7.172 ms  | 1.080 ms |
| 247.1 µs  | 8.984 ms |
| 10.10 µs  | 192.8 µs |
| 72.12 µs  | 38.33 ms |
| 161.4 µs  | 128.9 µs |
| 32.84 µs  | 20.16 ms |
| 29.74 µs  | 289.0 ms |
| 43.98 µs  | 1.778 µs |
| 213.9 µs  | 69.56 µs |
| 32.87 µs  | 21.57 ms |
| 491.3 µs  | 86.90 ms |
| 252.4 µs  | 61.18 ms |
| 101.2 µs  | 2.5839 s |
| 79.73 µs  | 2.4001 s |

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
