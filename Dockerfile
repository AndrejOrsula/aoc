ARG RUST_VERSION=1.74
FROM rust:${RUST_VERSION}

### Use bash as the default shell
SHELL ["/bin/bash", "-c"]

### Configure the workspace
ARG WORKSPACE="/root/ws"
ENV WORKSPACE="${WORKSPACE}"
WORKDIR ${WORKSPACE}

### Install dependencies (required by the optional `z3` feature)
RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -yq --no-install-recommends \
    cmake=3.25.1-1 \
    libclang-dev=1:14.0-55.7~deb12u1 \
    libz3-dev=4.8.12-3.1 && \
    rm -rf /var/lib/apt/lists/*

### Copy the source
COPY . "${WORKSPACE}"

### Build the project
RUN cargo build --release --all-targets --all-features
