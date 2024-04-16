FROM clux/muslrust:stable AS chef
USER root
RUN cargo install cargo-chef
WORKDIR /app

## Prepare the project for building
FROM chef AS planner
COPY . . 
RUN cargo chef prepare --recipe-path recipe.json

## Build the project
FROM planner AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl --bin rocket_api

# Install build dependencies
# RUN apk add --no-cache build-base musl-dev openssl-dev openssl

# Create a new empty shell project
# WORKDIR /app


# RUN apk add --no-cache musl-dev
# RUN cargo build --release

## This is a way to force cargo to rebuild the project
#RUN touch src/main.rs
#RUN cargo build --release --target x86_64-unknown-linux-musl


## Runtime stage
FROM alpine:3.19 AS runtime
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/rocket_api /usr/local/bin/
## Copy the Rocket.toml file to the runtime stage OTHERWISE YOU GET CANCER AS IT WILL NOT WORK
ENV ROCKET_ADDRESS=0.0.0.0
#COPY --from=builder /app/Rocket.toml /usr/local/bin/
CMD ["/usr/local/bin/rocket_api"]
 
## The first stage of the Dockerfile is the builder stage. It uses the  rust:alpine3.19  image as the base image. The builder stage compiles the Rust code and creates the binary. The second stage of the Dockerfile is the runtime stage. It uses the  alpine:3.19  image as the base image. The runtime stage copies the binary from the builder stage and runs it. 
## The  --target x86_64-unknown-linux-musl  flag is used to compile the Rust code for the  musl  target. The  musl  target is a statically linked target that can be used to create a standalone binary. 
## The  COPY --from=builder  command is used to copy the binary from the builder stage to the runtime stage. 
## The  CMD  instruction is used to specify the command that should be run when the container starts. In this case, the command is  /usr/local/bin/rocket_api . 


