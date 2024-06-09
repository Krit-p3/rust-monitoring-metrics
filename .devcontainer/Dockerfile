FROM mcr.microsoft.com/devcontainers/rust:latest

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && cargo install cargo-watch \
     && apt-get clean -y && rm -rf /var/lib/apt/lists/*
