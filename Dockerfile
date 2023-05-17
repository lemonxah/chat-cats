FROM rust AS builder
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry --mount=type=cache,target=/target \
    cargo build --release && mv /target/release/chat-cats /chat-cats

FROM debian:bullseye-slim
RUN apt update
RUN apt install libssl-dev ca-certificates -y
ENV RUST_LOG=info,chat-cats=trace
COPY --from=builder chat-cats .
CMD ["/chat-cats"]
