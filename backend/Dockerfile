FROM rust:1.95-slim AS builder

WORKDIR /app

ENV SQLX_OFFLINE=true

COPY Cargo.toml Cargo.lock ./
COPY migrations ./migrations
COPY .sqlx ./.sqlx
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm src/main.rs

COPY src ./src
RUN touch src/main.rs && cargo build --release

FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -m -u 1001 appuser

WORKDIR /app
COPY --from=builder /app/target/release/backend ./backend
RUN chown appuser:appuser ./backend

USER appuser

EXPOSE 3000

CMD ["./backend"]