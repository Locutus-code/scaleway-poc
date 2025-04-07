FROM rust:1.86.0-slim-bookworm as builder
WORKDIR /app
COPY . .
RUN cargo build --release


FROM debian:bookworm-slim as runner
COPY --from=builder /app/target/release/be /usr/local/bin/be
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["be"]
