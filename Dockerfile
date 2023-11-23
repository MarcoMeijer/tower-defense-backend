FROM rust:slim-buster

RUN apt-get update && \
  apt-get -y upgrade && \
  apt-get -y install libpq-dev

WORKDIR .
COPY . .

RUN cargo build --release

EXPOSE 80

ENTRYPOINT ["/bin/bash", "-c", "cargo run --release"]

