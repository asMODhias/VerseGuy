FROM rust:1.70-slim-bullseye AS builder
WORKDIR /usr/src/verseguy
COPY . .
RUN cd containers/p2p && cargo build --release

FROM debian:bullseye-slim
COPY --from=builder /usr/src/verseguy/target/release/examples /usr/local/bin/
WORKDIR /work
CMD ["/bin/bash"]