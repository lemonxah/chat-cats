#!/bin/sh
export RUST_LOG=info,chat_cats=trace
screen -d -m -S chat-cat ./target/release/chat-cats
