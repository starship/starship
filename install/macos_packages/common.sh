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

  # Try to get the version from three sources in the following order:
  #  - the STARSHIP_VERSION envar (usually set by the CI)
  #  - Running the binary file
  #  - By cutting out the first version tag in Cargo.toml
  # These get increasingly fragile as we go down the list---ideally CI should
  # always run with STARSHIP_VERSION set to avoid issues in determining version.
  if [ "$STARSHIP_VERSION" != "" ]; then
    echo "$STARSHIP_VERSION"
  elif "$starship_program_file" -V >/dev/null 2>&1; then
    "$starship_program_file" -V | grep -Eo '[0-9]+\.[0-9]+\.[0-9]+'
  else
    pushd "$(git rev-parse --show-toplevel)" || true
    grep '^version = \"\(.*\)\"' Cargo.toml | head -n 1 | cut -f 2 -d '"' | grep -Eo '[0-9]+\.[0-9]+\.[0-9]+'
    popd
  fi
}
