ARG DEBIAN_VERSION=bookworm
FROM debian:${DEBIAN_VERSION}-slim

ARG NOTS_GITHUB_URL="https://github.com/explodingcamera/nots.git"
ARG NOTS_VERSION="latest"
ARG NOTS_LICENSE="Apache-2.0"
ARG TARGETARCH

ENV USERNAME=noroot
ARG USER_UID=1000
ARG USER_GID=$USER_UID
RUN groupadd --gid $USER_GID $USERNAME \
    && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME --home-dir /app \
    && apt-get update && apt-get install -y --no-install-recommends ${EXTRA_DEPS} ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY ./nots-worker-${TARGETARCH} /usr/local/bin/nots-worker
RUN chmod +x /nots/worker

WORKDIR /app
USER noroot
ENTRYPOINT ["/usr/local/bin/nots-worker"]