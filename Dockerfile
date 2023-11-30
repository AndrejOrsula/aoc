ARG RUST_VERSION=1.74
FROM rust:${RUST_VERSION}

### Use bash as the default shell
SHELL ["/bin/bash", "-c"]

### Configure the workspace
ARG WORKSPACE="/root/ws"
ENV WORKSPACE="${WORKSPACE}"
WORKDIR ${WORKSPACE}

### Copy the source
COPY . "${WORKSPACE}"

### Build the project
RUN cargo build --release --all-features
