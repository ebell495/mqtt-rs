FROM ghcr.io/evanrichter/cargo-fuzz as builder

ADD . /mqtt-rs
WORKDIR /mqtt-rs/fuzz
RUN cargo +nightly fuzz build 

FROM debian:bookworm
COPY --from=builder /mqtt-rs/fuzz/target/x86_64-unknown-linux-gnu/release/mqtt-rs-fuzz /