FROM rust:1.78-slim-bullseye AS builder
WORKDIR /usr/src/verseguy
COPY . .
# Install nightly toolchain with rustup to enable edition-2024 support
RUN rustup toolchain install nightly && rustup default nightly
RUN rustc --version && cargo --version
RUN cd containers/p2p && cargo build --release

FROM debian:bullseye-slim
COPY --from=builder /usr/src/verseguy/target/release/examples /usr/local/bin/
WORKDIR /work
CMD ["/bin/bash"]