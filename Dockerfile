FROM rust:1.86.0-slim

RUN apt-get update && \
    apt-get install pkg-config libsmbclient-dev -y

WORKDIR /src

COPY . .

RUN cargo install --path . 

ENTRYPOINT [ "ad-pentest" ]
