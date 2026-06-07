# ---------------------------------------------------------------------------- #
#                                    Builder                                   #
# ---------------------------------------------------------------------------- #

FROM rust:1.95 AS builder

WORKDIR /app

RUN rustup toolchain install stable \
    && rustup target add wasm32-unknown-unknown \
    && curl -sSL https://dioxus.dev/install.sh | bash

COPY . .

RUN dx bundle --web --release

# ---------------------------------------------------------------------------- #
#                                      App                                     #
# ---------------------------------------------------------------------------- #

FROM debian:trixie

RUN apt-get update && apt-get install -y curl

COPY --from=builder /app/target/dx/dashberry/release/web/ .

ENV PORT=8080
ENV IP=0.0.0.0
ENV SYSINFO_PROC_PATH="/host"

CMD ["./server"]