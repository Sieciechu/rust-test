# syntax=docker/dockerfile:experimental

# ------------------------------  env stage  ------------------------------
#
ARG RUST_VERSION=1.32-slim
ARG VCS_REF
ARG BUILD_DATE
ARG BINARY_NAME

FROM rust:${RUST_VERSION} as develop_env
RUN apt-get update -y && apt-get install -y \
    vim \
    curl

WORKDIR /usr/src/app/

EXPOSE 8000

# ------------------------------  build stage  ------------------------------
#
FROM develop_env as builder
COPY . /usr/src/app
WORKDIR /usr/src/app/
RUN ["cargo", "build", "--release"]

# ------------------------------  prod stage  ------------------------------
#
FROM rust:${RUST_VERSION} as prod

COPY --from=builder /usr/src/app/target/release/$BINARY_NAME /usr/src/app/$BINARY_NAME

ARG VCS_REF
ARG BUILD_DATE
ARG BINARY_NAME
LABEL org.label-schema.build-date=${BUILD_DATE} \
      org.label-schema.vcs-ref=${VCS_REF} \
      org.label-schema.vcs-url="https://github.com/Sieciechu/rust-test"

EXPOSE 8000
ENV BINARY_NAME=${BINARY_NAME}
CMD /usr/src/app/$BINARY_NAME 
