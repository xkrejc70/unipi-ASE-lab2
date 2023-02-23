# For the build stage, use the official Rust image
FROM rust:slim-buster AS builder

ARG TARGETPLATFORM

WORKDIR /root

RUN --mount=type=cache,target=/usr/local/cargo/registry,id=${TARGETPLATFORM} \
    cargo install cargo-strip

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry,id=${TARGETPLATFORM} --mount=type=cache,target=/root/target,id=${TARGETPLATFORM} \
    cargo build --release && \
    cargo strip && \
    mv /root/target/release/string_rust /root


FROM debian:buster-slim as runner
COPY --from=builder /root/string_rust /

EXPOSE 5000

CMD ["./string_rust"]
