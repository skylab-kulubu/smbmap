FROM rust:1.86.0

RUN apt-get update && apt-get upgrade -y

WORKDIR /src

COPY . .

RUN cargo build --release

ENTRYPOINT [ "target/release/ad-pentest" ]
