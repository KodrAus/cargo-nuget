FROM microsoft/dotnet:1.1.1-sdk

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        curl \
        ca-certificates

# Install Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

ENV DOTNET_SKIP_FIRST_TIME_EXPERIENCE 1
