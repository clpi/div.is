FROM rust:1.45-slim

WORKDIR /usr/src/app/back

COPY . .

RUN apt-get update -y && apt-get upgrade -y && apt-get install clang llvm-dev libclang-dev pkg-config libssl-dev -y

RUN cargo build --release

RUN cargo install --path ./

EXPOSE 3001

CMD ["/usr/local/cargo/bin/mem"]
