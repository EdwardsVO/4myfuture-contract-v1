#!/bin/bash


# BUILD COMMAND FOR GETTING THE CONTRACT BINARY
set -e

cd contract

cargo build --all --target wasm32-unknown-unknown --release

mkdir -p ./out

cp target/wasm32-unknown-unknown/release/*.wasm ./out/

near dev-deploy ./out/greeter.wasm