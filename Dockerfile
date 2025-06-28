# syntax=docker/dockerfile:1
FROM rust:1.77 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/findag /usr/local/bin/findag
COPY --from=builder /app/configs /app/configs
COPY --from=builder /app/docs /app/docs
COPY --from=builder /app/scripts /app/scripts
COPY --from=builder /app/README.md /app/README.md
EXPOSE 8080 9898 9000
ENTRYPOINT ["/usr/local/bin/findag"] 