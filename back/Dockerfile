FROM rust:1.45-slim-buster

WORKDIR /usr/src/app/back
COPY . .

RUN apt-get install clang llvm-dev libclang-dev
RUN cargo build --release
RUN cargo install --path ./

EXPOSE 8000

CMD ["/usr/local/cargo/bin/mem"]
