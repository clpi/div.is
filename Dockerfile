FROM rust:1.44-alpine

WORKDIR /usr/src/app
COPY . .

RUN cargo install diesel_cli --no-default-features --features postgres

RUN cargo install cargo-watch

EXPOSE 3001

VOLUME ["/usr/local/cargo"]
