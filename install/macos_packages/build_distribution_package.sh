#!/bin/bash

component_package="$1"
resources="$2"
arch="$3"

usage() {
  echo "Builds a distribution package for macOS."
  echo "Assumes that the following items already exist:"
  echo "    - A starship component package"
  echo "    - Resources in a pkg_resources directory"
  echo "Usage: $0 <path-to-component-package> <path-to-pkg-resources> <arch>"
  echo "    where arch is one of \"arm64\" or \"x86_64\""
}

script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)"
source "$script_dir/common.sh"

if [[ "$OSTYPE" != 'darwin'* ]]; then
  error "This script only works on MacOS"
fi

if [[ "${3-undefined}" = "undefined" ]]; then
  usage
  exit 1
fi

# Generate a distribution file with the appropriate architecture plists
if [[ "$arch" == "x86_64" || "$arch" == "x64" ]]; then
  archplist="$script_dir/x86_64.plist"
elif [[ "$arch" == "arm64" || "$arch" == "aarch64" ]]; then
  archplist="$script_dir/aarch64.plist"
else
  error "Invalid architecture: $arch"
fi

productbuild --synthesize --package starship-component.pkg --product "$archplist" starship_raw.dist

# A terrible hacky way to insert nodes into XML without needing a full XML parser:
# search for a line that matches our opening tag and insert our desired lines after it
# Solution taken from https://www.theunixschool.com/2012/06/insert-line-before-or-after-pattern.html

while read -r line; do
  echo "$line"
  if echo "$line" | grep -qF '<installer-gui-script '; then
    echo '<welcome file="welcome.html" mime-type="text-html" />'
    echo '<license file="license.html" mime-type="text-html" />'
    echo '<conclusion file="conclusion.html" mime-type="text-html" />'
    echo '<background file="icon.png" scaling="proportional" alignment="bottomleft"/>'
  fi
done <starship_raw.dist >starship.dist

# The above script does not correctly take care of the last line. Apply fixup.
echo '</installer-gui-script>' >>starship.dist

echo "Creating distribution package with following distribution file:"
cat starship.dist

echo "Resource directory is $resources"
echo "Component package is $component_package"

# Build the distribution package
productbuild --distribution starship.dist --resources "$resources" --package-path "$component_package" starship-unsigned.pkg

# Clean up the distribution files
rm -- *.dist
