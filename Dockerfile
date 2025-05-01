FROM rust:1.86.0

WORKDIR /src

COPY . .

RUN cargo build --release

ENTRYPOINT [ "target/release/ad-pentest" ]
