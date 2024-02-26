#!/bin/bash

set -euo pipefail

# Requirements:
#  - MacOS
#  - A starship repository with binaries and documentation already built.
# Usage: run this script, passing $1 to the repository path. The script assumes
# it is being run from within a starship repository if $1 is not provided.

usage() {
    echo "Builds a component package for macOS."
    echo "Assumes that the following items already exist:"
    echo "    - A starship binary which has already been notarized"
    echo "    - Documentation created by \`npm run build\`, usually in a dist"
    echo "      directory at <repo>/docs/.vuepress/dist"
    echo "Usage: $0 <path-to-starship-binary> <path-to-dist-directory>"
}

script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)"
source "$script_dir/common.sh"

cleanup_server() {
    if [[ -n "${server_pid-}" ]]; then
        echo "Killing HTTP server ($server_pid) to clean up."
        kill "$server_pid"
        rm "x86_64-apple-darwin-simple-http-server"
    else
        echo "No server found, exiting normally."
    fi
}

if [[ "$OSTYPE" != 'darwin'* ]]; then
    error "This script only works on MacOS"
fi

if [[ "${2-undefined}" = "undefined" ]]; then
    usage
    exit 1
fi

starship_program_file="$1"
starship_documentation_dir="$2"

if [ ! -f "$starship_program_file" ]; then
    error "Could not find starship binary at $starship_program_file"
fi

if [ ! -d "$starship_documentation_dir" ]; then
    error "Could not find starship documentation at $starship_documentation_dir"
fi

pkgdir="$(mktemp -d)"
mkdir -p "$pkgdir/usr/local/bin"
cp "$starship_program_file" "$pkgdir/usr/local/bin/starship"

# Now we get to make documentation! Vuepress was not designed to build locally
# (too many assumptions about running on an HTTP server), so we do the hackiest
# thing imagineable: start an http server, and use wget to make a local mirror.

# First, we need to install the server. There are several options, but this one
# provides prebuilt binaries for MacOS, making it the easiest. (yay rust)
server_prog_name="x86_64-apple-darwin-simple-http-server"
latest_server_version="$(curl -L -s -H 'Accept: application/json' https://github.com/TheWaWaR/simple-http-server/releases/latest | sed -e 's/.*"tag_name":"\([^"]*\)".*/\1/')"
curl -LOk "https://github.com/TheWaWaR/simple-http-server/releases/download/$latest_server_version/$server_prog_name"
chmod u+x "$server_prog_name"

# Next, we build the documentation and serve it via simple-http-server
trap cleanup_server INT
"./$server_prog_name" --ip 127.0.0.1 --index "$starship_documentation_dir" &
server_pid="$!"
# Give the server a chance to come online before trying to mirror it
echo "Sleeping to give the server a chance to come online..."
sleep 3

# Use wget to make a mirror of the site and move it into the package. Not installed
# on MacOS by default, but lucky for us, it does exist on GHActions runners.
# Wget may return nonzero exit codes even if things were mostly fine (e.g. 404 for
# some links on translated pages) so we simply ignore if it has a failure
wget --mirror --convert-links --adjust-extension --page-requisites --no-parent 127.0.0.1:8000 &>wget.log || true
mkdir -p "$pkgdir/usr/local/share/doc/"
mv 127.0.0.1:8000 "$pkgdir/usr/local/share/doc/starship"

# Technically a race condition here, but very unlikely to hit it in practice.
cleanup_server
trap - INT

# Build the component package
version="$(starship_version "$starship_program_file")"
pkgbuild --identifier com.starshipprompt.starship --version "$version" --root "$pkgdir" starship-component.pkg
