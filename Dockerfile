FROM ekidd/rust-musl-builder:latest as builder
COPY --chown=rust:rust . ./
RUN cargo build --release

FROM scratch
WORKDIR /url-mapper-rs
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/url-mapper-rs ./
COPY config ./config
EXPOSE 3000
CMD ["./url-mapper-rs"]
