FROM rust

RUN cargo install sqlx-cli

RUN cargo install cargo-watch

WORKDIR /usr/src/app

EXPOSE 1998

VOLUME ["/usr/local/cargo"]