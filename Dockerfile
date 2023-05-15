FROM rust:alpine

RUN apk update
RUN apk add --no-cache musl-dev curl
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | ash

RUN apk -v --no-cache --update add nodejs npm

COPY build.sh /build.sh

VOLUME [ "/code" ]

WORKDIR /code
ENTRYPOINT [ "/build.sh" ]
