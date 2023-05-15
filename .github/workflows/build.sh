#!/bin/sh
set -e
cd /code

wasm-pack build --release

cd www
    npm install
    npm run build
cd ..
