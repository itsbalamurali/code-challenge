FROM rust:latest AS builder
ARG GIT_SHA
ENV GIT_SHA=${GIT_SHA}
WORKDIR /usr/src/app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo install --path .

# Bundle Stage
FROM debian:stable-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*
RUN update-ca-certificates
COPY --from=builder /usr/local/cargo/bin/code-challenge .
CMD ["./code-challenge", "-a", "0.0.0.0", "-p", "8080"]