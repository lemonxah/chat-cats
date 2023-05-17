FROM rust:1.69 AS builder
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt update
RUN apt install libssl-dev ca-certificates -y
ENV RUST_LOG=info,chat-cats=trace
COPY --from=builder ./target/release/chat-cats ./chat-cats
CMD ["/chat-cats"]
