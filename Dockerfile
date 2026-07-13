# build the frontend
FROM node:20-slim AS frontend

WORKDIR /app
COPY package.json package-lock.json ./
RUN npm ci

COPY svelte.config.js vite.config.ts tsconfig.json ./
COPY src ./src
COPY static ./static

# Server mode (no TAURI_ENV_PLATFORM set)
RUN npm run build



# build the rust binary
FROM rust:1.78-slim AS backend

WORKDIR /app

# cache dependencies first
COPY src-tauri/Cargo.toml src-tauri/Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release || true
RUN rm -rf src

# build the actual binary
COPY src-tauri/src ./src
COPY src-tauri/build.rs ./
RUN cargo build --release



# minimal runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /slate

COPY --from=backend /app/target/release/Slate ./Slate

COPY --from=frontend /app/frontend ./frontend

# data dir from the SQL
RUN mkdir -p /data

EXPOSE 3000

ENV SLATE_PORT=3000
ENV SLATE_DB_PATH=/data/slate.db

CMD ["./Slate", "--server"]
