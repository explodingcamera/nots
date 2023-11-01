FROM debian:${DEBIAN_VERSION}-slim
ARG NOTS_GITHUB_URL="https://github.com/explodingcamera/nots.git"
ARG NOTS_VERSION="main"
ARG NOTS_LICENSE="Apache-2.0"
ARG DEBIAN_VERSION=bookworm
ARG EXTRA_DEPS="build-essential libssl-dev libgit2-dev"

LABEL org.opencontainers.image.title="notsd"
LABEL org.opencontainers.image.description="Nots Daemon"
LABEL org.opencontainers.image.url="https://nots.dev"
LABEL org.opencontainers.image.source=${NOTS_GITHUB_URL}
LABEL org.opencontainers.image.licenses=${NOTS_LICENSE}
LABEL org.opencontainers.image.version=${NOTS_VERSION}

RUN groupadd docker && \
  useradd -m notsd -G docker && \
  mkdir -p /home/notsd/bin

COPY --chown=notsd:docker ./notsd /home/notsd/bin/notsd
RUN chmod +x /home/notsd/bin/notsd

USER notsd

EXPOSE 8080
ENV NOTS_SECRET=
CMD ["/home/notsd/bin/notsd"]
