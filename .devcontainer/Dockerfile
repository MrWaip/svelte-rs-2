# syntax=docker/dockerfile:1.7
FROM mcr.microsoft.com/devcontainers/base:ubuntu-24.04

ARG RUST_VERSION=1.94.1
ARG NODE_VERSION=22.13.0

ENV CARGO_HOME=/usr/local/cargo \
    RUSTUP_HOME=/usr/local/rustup \
    NVM_DIR=/usr/local/nvm \
    PATH=/usr/local/cargo/bin:/usr/local/nvm/versions/node/v${NODE_VERSION}/bin:$PATH \
    DEBIAN_FRONTEND=noninteractive

RUN --mount=type=cache,target=/var/cache/apt,sharing=locked \
    --mount=type=cache,target=/var/lib/apt,sharing=locked \
    rm -f /etc/apt/apt.conf.d/docker-clean \
    && apt-get update && apt-get install -y --no-install-recommends \
        build-essential pkg-config libssl-dev cmake git git-lfs curl ca-certificates \
        clang lld jq ripgrep fd-find python3 python3-pip \
    && git lfs install --system --skip-repo

RUN mkdir -p ${CARGO_HOME} ${RUSTUP_HOME} ${NVM_DIR} \
    && chown vscode:vscode ${CARGO_HOME} ${RUSTUP_HOME} ${NVM_DIR}
USER vscode

RUN curl -fsSL https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.1/install.sh | bash \
    && . ${NVM_DIR}/nvm.sh && nvm install ${NODE_VERSION} && nvm alias default ${NODE_VERSION}

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | \
        sh -s -- -y --no-modify-path --profile minimal --default-toolchain ${RUST_VERSION} \
                  --component rustfmt --component clippy --component rust-src --component rust-analyzer \
                  --target wasm32-unknown-unknown

ARG NARSIL_CACHE_BUST=0
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked,uid=1000,gid=1000 \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked,uid=1000,gid=1000 \
    --mount=type=cache,target=/tmp/cargo-target,sharing=locked,uid=1000,gid=1000 \
    CARGO_TARGET_DIR=/tmp/cargo-target \
    cargo install --git https://github.com/postrv/narsil-mcp narsil-mcp

RUN curl -fsSL https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash \
    && cargo binstall -y \
        just \
        cargo-sweep \
        cargo-nextest \
        samply \
        wasm-pack \
        cargo-dylint \
        dylint-link

ARG TOOLS_CACHE_BUST=0
RUN --mount=type=cache,target=/home/vscode/.npm,sharing=locked,uid=1000,gid=1000 \
    . ${NVM_DIR}/nvm.sh && npm install -g @anthropic-ai/claude-code skills task-master-ai

RUN echo 'alias claude="claude --dangerously-skip-permissions"' >> /home/vscode/.bashrc \
    && mkdir -p ~/.local/share/bash-completion/completions \
    && just --completions bash > ~/.local/share/bash-completion/completions/just \
    && echo 'source <(just --completions bash)' >> /home/vscode/.bashrc
