FROM rust:1.86.0

RUN apt-get update && \
  apt-get install libsmbclient-dev -y

WORKDIR /src

COPY . .

RUN cargo install --path . 

ENTRYPOINT [ "ad-pentest" ]
