ARG DEBIAN_VERSION=bookworm
FROM debian:${DEBIAN_VERSION}-slim

ARG NOTS_GITHUB_URL="https://github.com/explodingcamera/nots.git"
ARG NOTS_VERSION="latest"
ARG NOTS_LICENSE="Apache-2.0"
ARG TARGETARCH

RUN apt-get update && \
    apt-get install -y --no-install-recommends ${EXTRA_DEPS} ca-certificates \
    && rm -rf /var/lib/apt/lists/* && groupadd noroot \
    --gid 1000 \
    && useradd noroot \
    --uid 1000 \
    --gid noroot \
    --shell /bin/sh \
    --home-dir /app \
    --create-home \
    && mkdir /nots

COPY ./nots-worker-${TARGETARCH} /nots/worker
RUN chmod +x /nots/worker

WORKDIR /app
ENTRYPOINT ["/nots/worker"]