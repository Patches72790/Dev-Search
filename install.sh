#!/bin/bash

echo "Building dev-search executable..."
[ -e $(which cargo) ] && cargo build --release || echo "Cargo utility not found. Exiting..." && exit 2

echo "Checking for $HOME/.local/bin directory..."
[ -e $HOME/.local/bin ] || mkdir -p $HOME/.local/bin

echo "Installing dev-search into local bin directory..."
cp ./target/release/developer-search $HOME/.local/bin/dev-search

echo "Finished!"
