FROM rust:1.74.0 as builder

WORKDIR /usr/src/blogpost
COPY . .



RUN cargo install --path . --features docker
RUN cargo install sqlx-cli



FROM ubuntu:latest
COPY --from=builder /usr/local/cargo/bin/blogpost_app /usr/local/bin/blogpost_app
COPY --from=builder /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx
COPY --from=builder /usr/src/blogpost/migrations migrations
RUN apt-get update && apt-get install -y curl && rm -rf /var/lib/apt/lists/*
RUN echo -n "DATABASE_URL=sqlite:/database.db" > .env


RUN sqlx database create 
RUN sqlx migrate run

RUN mkdir images
RUN chmod -R 644 images

EXPOSE 8000


CMD ["blogpost_app"]

