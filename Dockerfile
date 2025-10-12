# Dockerfile для локальной разработки (опционально)
# Shuttle использует свою собственную инфраструктуру, поэтому Docker не обязателен

FROM rust:1.75-slim as builder

WORKDIR /usr/src/app

# Кэширование зависимостей
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Копирование исходников
COPY . .

# Сборка
RUN cargo build --release

# Runtime образ
FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/fodifood-bot /usr/local/bin/fodifood-bot

EXPOSE 8000

CMD ["fodifood-bot"]
