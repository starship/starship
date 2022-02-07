#!/bin/bash

set -euo pipefail

# Example envrionmental variables that need to be set.
# KEYCHAIN_ENTRY=AC_PASSWORD   # Or whatever you picked for <AUTH_ITEM_NAME> above
# RUNNER_TEMP=~/Library/Keychains/
# KEYCHAIN_FILENAME=login.keychain

usage(){
    echo "Builds, signs, and notarizes starship."
    echo "Read readme.md in the script directory to see the assumptions the script makes."
    echo "Usage: $0 <path-to-starship-binary> <path-to-docs-directory> <arch> [pkgname]"
    echo "  Example: $0 target/release/starship docs/ x64"
    echo "  Example: $0 target/debug/starship docs/ arm64 starship-1.2.1-arm64.pkg"
    echo ""
    echo "If no pkgname is provided, the package will be named starship-<version>-<arch>.pkg"
}

script_dir="$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
source "$script_dir/common.sh"

if [[ -z ${KEYCHAIN_ENTRY+x} ]]; then
  error "Environmental variable KEYCHAIN_ENTRY must be set."
fi

if [[ -z ${RUNNER_TEMP+x} ]]; then
  error "Environmental variable RUNNER_TEMP must be set."
fi

if [[ -z ${KEYCHAIN_FILENAME+x} ]]; then
  error "Environmental variable KEYCHAIN_ENTRY must be set."
fi

keychain_path="$RUNNER_TEMP/$KEYCHAIN_FILENAME"
if [[ ! -f "$keychain_path" ]]; then
  error "Could not find keychain at $keychain_path"
fi

if [[ "$3" == "" ]]; then
    usage
    exit 1
fi

starship_binary="$1"
starship_docs_dir="$2"
arch="$3"
pkgname="${4:-}"

if [[ ! -d "$starship_docs_dir/.vuepress/dist" ]]; then
  error "Documentation does not appear to have been built!"
fi


echo "Signing binary"
codesign --timestamp --keychain "$keychain_path" --sign "E03290CABE09E9E42341C8FC82608E91241FAD4A" --verbose -f -o runtime "$starship_binary"

# Make ZIP file to notarize binary
if [ "$starship_binary" != "starship" ]; then
  cp "$starship_binary" starship
fi
zip starship.zip starship

echo "Submitting binary for notarization"
xcrun notarytool submit starship.zip --keychain-profile "$KEYCHAIN_ENTRY" --wait

# Don't think this is actually necessary, but not costly so why not
rm starship
unzip starship.zip

# Create the component package
bash "$script_dir/build_component_package.sh" "starship" "$starship_docs_dir/.vuepress/dist"

# Create the distribution package
resources_path="$script_dir/pkg_resources"
bash "$script_dir/build_distribution_package.sh" "starship-component.pkg" "$resources_path" "$arch"

# Codesign the package installer
productsign --timestamp --sign "E525359D0B5AE97B7B6F5BB465FEC872C117D681" starship-unsigned.pkg starship.pkg

# Notarize the package installer
xcrun notarytool submit starship.pkg --keychain-profile "$KEYCHAIN_ENTRY" --wait

# Staple things
xcrun stapler staple starship.pkg

# Rename to expected name
if [ "$pkgname" = "" ]; then
  version="$(starship_version "$starship_binary")"
  pkgname="starship-$version-$arch.pkg"
fi

mv starship.pkg "$pkgname"