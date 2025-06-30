#!/bin/bash

if ! command -v rustc 
then
  echo "Rust is not installed. Please install Rust first."
  exit 1
fi

if ! command -V cargo 
then 
  echo "cargo is not installed. Please install cargo first."
  exit 1
fi

SHELL_NAME=$(basename "$SHELL")

if [ "$SHELL_NAME" = "zsh" ]; then
    CONFIG_FILE="$HOME/.zshrc"
elif [ "$SHELL_NAME" = "bash" ]; then
    CONFIG_FILE="$HOME/.bashrc"
else
    echo "Unknown Shell : $SHELL_NAME"
    exit 1
fi

echo export PATH=$PATH:~/.cargo/bin/ >> "$CONFIG_FILE"
echo export NYX=$(pwd) >> "$CONFIG_FILE"
echo "NYX env var successfully added"

cargo br
cargo i

source "$CONFIG_FILE"

if ! command -v nyx 
then
  echo "Error when installing NYX. Please try again."
  exit 2
fi

echo "NYX has been successfully installed."
