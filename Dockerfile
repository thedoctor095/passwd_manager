FROM rust:1.90 AS builder

WORKDIR /opt
COPY . .

RUN cargo build --release
RUN cp /opt/target/release/app web_app
RUN cargo clean

FROM ubuntu:24.04
RUN apt update && rm -rf /var/lib/apt/lists/*

WORKDIR /opt
COPY --from=builder /opt/web_app .
COPY --from=builder /opt/.env .

CMD ["./web_app"]