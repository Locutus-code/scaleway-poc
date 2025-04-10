FROM rust:1.86.0-slim-bookworm as builder
WORKDIR /app
COPY . .
RUN apt-get update && apt-get install -y ca-certificates && update-ca-certificates
RUN cargo build


FROM debian:bookworm-slim as runner
RUN apt-get update && apt-get install -y ca-certificates && update-ca-certificates
COPY --from=builder /app/target/debug/be /usr/local/bin/be
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["be"]
