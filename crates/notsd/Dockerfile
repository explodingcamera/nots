ARG DEBIAN_VERSION=bookworm
FROM debian:${DEBIAN_VERSION}-slim
ARG NOTS_GITHUB_URL="https://github.com/explodingcamera/nots.git"
ARG NOTS_VERSION="latest"
ARG NOTS_LICENSE="Apache-2.0"
ARG EXTRA_DEPS="git"
ARG TARGETARCH

LABEL org.opencontainers.image.title="notsd"
LABEL org.opencontainers.image.description="Nots Daemon"
LABEL org.opencontainers.image.url="https://nots.dev"
LABEL org.opencontainers.image.source=${NOTS_GITHUB_URL}
LABEL org.opencontainers.image.licenses=${NOTS_LICENSE}
LABEL org.opencontainers.image.version=${NOTS_VERSION}

RUN apt-get update && \
  apt-get install -y --no-install-recommends ${EXTRA_DEPS} ca-certificates \
  && rm -rf /var/lib/apt/lists/* && groupadd docker \
  && mkdir /notsd

COPY ./notsd-${TARGETARCH} /usr/local/bin/notsd
RUN chmod +x /notsd/notsd

EXPOSE 8080

ENV NOTS_SECRET=
ENV NOTS_SOCK_UID=
ENV NOTS_SOCK_GID=

CMD ["/usr/local/bin/notsd"]
