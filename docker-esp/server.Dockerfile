FROM rust:1.84.1

WORKDIR /server

COPY /server/src ./src
COPY /server/Cargo.toml /server/Cargo.lock /server/build.rs ./

RUN apt-get update &&\
    apt-get install -y protobuf-compiler

CMD \
    cargo run