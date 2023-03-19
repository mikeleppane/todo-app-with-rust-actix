FROM rust:1.68
WORKDIR /app

RUN apt update && \
    apt upgrade -y && \
    apt install --no-install-recommends lld clang postgresql libpq-dev -y

COPY . .

RUN cargo install diesel_cli --no-default-features --features postgres && \
    cargo build --release

RUN apt autoremove -y &&\
    apt clean -y &&\
    rm -rf /var/lib/apt/lists/*

ENTRYPOINT ["./docker-entrypoint.sh"]
