# syntax=docker/dockerfile:experimental
ARG RUST_VERSION=1.30-slim

FROM rust:${RUST_VERSION} as develop_env
RUN apt-get update -y && apt-get install -y \
    vim \
    curl

RUN rustup override set nightly
WORKDIR /usr/src/app/

EXPOSE 8000

# ----------------------------------
FROM develop_env as builder
COPY . /usr/src/app
WORKDIR /usr/src/app/
RUN ["cargo", "build", "--release"]

# ----------------------------------
FROM rust:${RUST_VERSION} as prod
ARG VCS_REF
ARG BUILD_DATE
LABEL org.label-schema.build-date=$BUILD_DATE \
      org.label-schema.vcs-ref=$VCS_REF \
      org.label-schema.vcs-url="https://github.com/Sieciechu/rust-test"

COPY --from=builder /usr/src/app/target/release/app /usr/src/app/app
EXPOSE 8000
CMD /usr/src/app/app
