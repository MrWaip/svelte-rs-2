# syntax=docker/dockerfile:1.7
FROM mcr.microsoft.com/devcontainers/base:ubuntu-24.04

ARG RUST_VERSION=1.94.1
ARG NODE_VERSION=22.13.0
ARG JUST_VERSION=1.46.0
ARG NARSIL_VERSION=1.6.1
ARG CLAUDE_VERSION=2.1.126
ARG WASM_PACK_VERSION=0.13.1

ENV CARGO_HOME=/usr/local/cargo \
    RUSTUP_HOME=/usr/local/rustup \
    NVM_DIR=/usr/local/nvm \
    PATH=/usr/local/cargo/bin:/usr/local/nvm/versions/node/v${NODE_VERSION}/bin:$PATH \
    DEBIAN_FRONTEND=noninteractive

# 1. apt — almost never changes
RUN --mount=type=cache,target=/var/cache/apt,sharing=locked \
    --mount=type=cache,target=/var/lib/apt,sharing=locked \
    rm -f /etc/apt/apt.conf.d/docker-clean \
    && apt-get update && apt-get install -y --no-install-recommends \
        build-essential pkg-config libssl-dev cmake git git-lfs curl ca-certificates \
        clang lld jq ripgrep fd-find python3 python3-pip \
    && git lfs install --system --skip-repo

# 2. nvm + node — LTS bumps only
RUN mkdir -p ${NVM_DIR} \
    && curl -fsSL https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.1/install.sh | bash \
    && . ${NVM_DIR}/nvm.sh && nvm install ${NODE_VERSION} && nvm alias default ${NODE_VERSION}

# 3. rustup toolchain + cargo dev tools — pinned to project
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | \
        sh -s -- -y --no-modify-path --default-toolchain ${RUST_VERSION} \
                  --component rustfmt --component clippy --component rust-src --component rust-analyzer \
    && cargo install just@${JUST_VERSION} cargo-sweep cargo-nextest samply \
                     wasm-pack@${WASM_PACK_VERSION} --locked


# 5. narsil-mcp — bumps with upstream
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    cargo install --locked --git https://github.com/postrv/narsil-mcp --tag v${NARSIL_VERSION} narsil-mcp

# 6. claude-code — frequent releases
RUN --mount=type=cache,target=/root/.npm,sharing=locked \
    . ${NVM_DIR}/nvm.sh && npm install -g @anthropic-ai/claude-code@${CLAUDE_VERSION}

RUN chmod -R a+rwX ${CARGO_HOME}/bin ${RUSTUP_HOME} ${NVM_DIR} \
    && chown -R vscode:vscode /usr/local/cargo /usr/local/rustup ${NVM_DIR}

RUN echo 'alias claude="claude --dangerously-skip-permissions"' >> /home/vscode/.bashrc
RUN mkdir -p ~/.local/share/bash-completion/completions \
    && just --completions bash > ~/.local/share/bash-completion/completions/just \
    && echo 'source <(just --completions bash)' >> /home/vscode/.bashrc 

USER vscode
