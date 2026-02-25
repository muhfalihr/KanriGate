FROM node:20-slim AS frontend-builder
WORKDIR /app/ui

COPY src/ui/package*.json ./
RUN npm install

COPY src/ui/ .

RUN npm run build


FROM rust:1.80-slim-bookworm AS backend-builder
WORKDIR /app

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release && rm -rf src

COPY src ./src
RUN touch src/main.rs && cargo build --release


FROM debian:bookworm-slim
WORKDIR /app


RUN apt-get update && apt-get install -y 
    libssl3 
    ca-certificates 
    curl 
    && curl -fsSL https://deb.nodesource.com/setup_20.x | bash - 
    && apt-get install -y nodejs 
    && rm -rf /var/lib/apt/lists/*


COPY --from=backend-builder /app/target/release/kanrigate ./backend
COPY --from=frontend-builder /app/ui/build ./ui
COPY --from=frontend-builder /app/ui/package*.json ./ui/

RUN cd ui && npm install --omit=dev


RUN echo '#!/bin/sh

PORT=3000 node ui/index.js & 

./backend

' > /app/entrypoint.sh && chmod +x /app/entrypoint.sh

EXPOSE 3000 3232

ENV APP_ENV=production
ENV APP_PORT=3232

CMD ["/app/entrypoint.sh"]
