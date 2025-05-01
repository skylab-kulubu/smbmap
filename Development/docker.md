# Docker

## Dockerfile

```Dockerfile
FROM rust:1.86.0

RUN apt-get update && \
  apt-get install libsmbclient-dev -y

WORKDIR /src

COPY . .

RUN cargo install --path . 

ENTRYPOINT [ "ad-pentest" ]
```

## Build

```bash
docker build -t username/ad-pentest:latest .
```

## Usage

```bash
docker run --rm lomarkomar/ad-pentest --help
```
