# Ubuntu 20.04 作为基础镜像（具体架构在 build 时用 --platform 选）
FROM ubuntu:20.04

ENV DEBIAN_FRONTEND=noninteractive
ENV TZ=Etc/UTC

# 1. 安装构建工具和常用依赖（和你之前在容器里敲的是一样的）
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        build-essential \
        curl \
        pkg-config \
        libssl-dev \
        ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# 2. 安装 Rust（用 rustup）
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:${PATH}

RUN curl https://sh.rustup.rs -sSf \
    | sh -s -- -y --profile minimal && \
    rustup default stable

# 3. 工作目录
WORKDIR /app
