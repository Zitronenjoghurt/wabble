FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /wabble

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /wabble/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin wabble-server

FROM debian:bookworm-slim AS runtime
WORKDIR /wabble
COPY --from=builder /wabble/target/release/wabble-server /usr/local/bin/server
CMD ["/usr/local/bin/server"]