FROM rust:1.69 AS builder
COPY dummy.rs .
COPY Cargo.toml .
COPY macros .
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN cargo build --release
RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml
COPY src src
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt update
RUN apt install libssl-dev ca-certificates -y
ENV RUST_LOG=info,chat-cats=trace
COPY --from=builder ./target/release/chat-cats ./chat-cats
CMD ["/chat-cats"]
