#!/bin/sh
set -e
cd $(dirname $0)/

wasm-pack build --release

cd www
    npm install
    npm run build
cd ..
