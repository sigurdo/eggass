#!/bin/sh
set -e

wasm-pack build --release

cd www
    npm install
    npm run build
cd ..
