FROM rust:slim-buster as build

# create a new empty shell project
WORKDIR /app

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src


RUN cargo build --release

# # our final base
FROM debian:stable-slim
ENV SERVER_HOST=0.0.0.0
ENV SERVER_PORT=8080
WORKDIR /app

EXPOSE 8080
COPY --from=build /app/target/release/ .
CMD ["./quewuicom"]
