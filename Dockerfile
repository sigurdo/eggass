FROM rust:alpine

# SHELL ["/bin/ash", "-eo", "pipefail", "-c"]
RUN apk update
RUN apk add --no-cache musl-dev curl
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | ash

RUN apk -v --no-cache --update add nodejs npm

COPY . /code

WORKDIR /code
# CMD [ "wasm-pack", "build", "--release" ]
# CMD [ "./build.sh" ]
ENTRYPOINT [ "build.sh" ]
