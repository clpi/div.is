FROM rust:latest-buster

RUN curl -sL https://deb.nodesource.com/setup_12.x | bash -
RUN apt-get update && apt-get install nodejs
RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo install cargo-watch

WORKDIR /usr/src/app
COPY . .

RUN npm install
EXPOSE 8000

VOLUME ["/usr/local/cargo"]
