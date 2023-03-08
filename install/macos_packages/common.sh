#!/bin/bash

error() {
  echo "[ERROR]: $1"
  exit 1
}

starship_version() {
  starship_program_file="$1"
  # Check if this is a relative path: if so, prepend './' to it
  if [ "$1" = "${1#/}" ]; then
    starship_program_file="./$starship_program_file"
  fi
  if "$starship_program_file" -V 2>&1 >/dev/null; then
    "$starship_program_file" -V | grep -Eo '[0-9]+\.[0-9]+\.[0-9]+'
  else
    # try to get this information from Cargo.toml
    pushd "$(git rev-parse --show-toplevel)" || true
    grep '^version = \"\(.*\)\"' Cargo.toml | cut -f 2 -d '"'
    popd
  fi
}
