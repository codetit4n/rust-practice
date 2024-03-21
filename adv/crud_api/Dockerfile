# Build stage
FROM rust:1.75-slim as builder

WORKDIR /app

# Accept the build argument
ARG DATABASE_URL

ENV DATABASE_URL=$DATABASE_URL

COPY . .

RUN cargo build --release

# Production stage
FROM debian:stable-slim

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/crud_api .

CMD ["./crud_api"]
