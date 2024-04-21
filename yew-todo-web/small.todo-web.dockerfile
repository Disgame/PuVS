## Prepare cargo chef to get faster builds
FROM clux/muslrust:stable AS chef
RUN cargo install cargo-chef
WORKDIR /app

## Get the dependencies
FROM chef AS planner
COPY . . 
RUN cargo chef prepare --recipe-path recipe.json

## Get the dependencies and build the project
FROM planner AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY . .
#RUN rustup target add wasm32-unknown-unknown
#RUN cargo build --release --target wasm32-unknown-unknown --bin yew-todo-web

#RUN cargo build --release --target x86_64-unknown-linux-musl --bin yew-todo-web


ADD https://github.com/trunk-rs/trunk/releases/download/v0.19.2/trunk-x86_64-unknown-linux-musl.tar.gz /tmp
RUN tar -xvf /tmp/trunk-x86_64-unknown-linux-musl.tar.gz -C /usr/local/bin && chmod +x /usr/local/bin/trunk

ADD https://github.com/rustwasm/wasm-bindgen/releases/download/0.2.92/wasm-bindgen-0.2.92-x86_64-unknown-linux-musl.tar.gz /tmp
RUN tar -xvf /tmp/wasm-bindgen-0.2.92-x86_64-unknown-linux-musl.tar.gz -C /usr/local/bin && chmod +x /usr/local/bin/wasm-bindgen-0.2.92-x86_64-unknown-linux-musl/wasm-bindgen

RUN rustup target add wasm32-unknown-unknown
#RUN apk add --no-cache musl-dev
RUN trunk build --release --dist dist

## Runtime stage to run the binary
FROM alpine:3.19 AS runtime
WORKDIR /release
#COPY --from=builder /app/. .
COPY --from=builder /app/Trunk.toml .
COPY --from=builder /app/dist .
COPY --from=builder /usr/local/bin/wasm-bindgen-0.2.92-x86_64-unknown-linux-musl/wasm-bindgen /usr/local/bin/wasm-bindgen
COPY --from=builder /usr/local/bin/trunk /usr/local/bin/trunk
CMD ["trunk", "serve", "--config", "Trunk-docker.toml"]