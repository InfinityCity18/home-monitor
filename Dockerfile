FROM rust:1.80.1

RUN git clone https://github.com/InfinityCity18/home-monitor
WORKDIR home-monitor/server
RUN cargo build --release

WORKDIR ../website
RUN cargo install trunk
RUN rustup target add wasm32-unknown-unknown
RUN trunk build --release

CMD trunk serve --release & cargo run --release --manifest-path ../server/Cargo.toml && fg
