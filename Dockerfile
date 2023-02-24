# Builder stage
FROM rust:1.67.1 AS builder
WORKDIR /app
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
ENV SQLX_OFFLINE true
RUN apt-get update & apt-get install -y extra-runtime-dependencies & rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/erpr erpr
COPY configuration configuration
COPY public public
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./erpr"]
#ENTRYPOINT ["./target/release/erpr"]
#RUN useradd -m favio
#USER favio
#CMD ["/bin/sh"]
