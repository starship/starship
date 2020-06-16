#!/bin/sh

# Options
#
#   -V, --verbose
#     Enable verbose output for the installer
#
#   -f, -y, --force, --yes
#     Skip the confirmation prompt during installation
#
#   -p, --platform
#     Override the platform identified by the installer
#
#   -b, --bin-dir
#     Override the bin installation directory
#
#   -a, --arch
#     Override the architecture identified by the installer
#
#   -B, --base-url
#     Override the base URL used for downloading releases

set -eu
printf "\n"

BOLD="$(tput bold 2>/dev/null || echo '')"
GREY="$(tput setaf 0 2>/dev/null || echo '')"
UNDERLINE="$(tput smul 2>/dev/null || echo '')"
RED="$(tput setaf 1 2>/dev/null || echo '')"
GREEN="$(tput setaf 2 2>/dev/null || echo '')"
YELLOW="$(tput setaf 3 2>/dev/null || echo '')"
BLUE="$(tput setaf 4 2>/dev/null || echo '')"
MAGENTA="$(tput setaf 5 2>/dev/null || echo '')"
NO_COLOR="$(tput sgr0 2>/dev/null || echo '')"

info() {
  printf "%s\n" "${BOLD}${GREY}>${NO_COLOR} $*"
}

warn() {
  printf "%s\n" "${YELLOW}! $*${NO_COLOR}"
}

error() {
  printf "%s\n" "${RED}x $*${NO_COLOR}" >&2
}

success() {
  printf "%s\n" "${GREEN}✓${NO_COLOR} $*"
}

has() {
  command -v "$1" >/dev/null 2>&1
}

mktmpdir() {
  tmpdir=$(mktemp -dqt "$1") || {
    error "Can't create temp file, exiting…"
    return 1
  }
  echo "$tmpdir"
}

fetch() {
  file="$1"
  url="$2"

  if has curl; then
    cmd="curl -w '%{http_code}' -sL -o $file $url"
    code=$(curl -w '%{http_code}' -sL -o "$file" "$url")
    if [ "$code" = "200" ]; then
      rc=0
    else
      rc=$code
    fi
  elif has wget; then
    cmd="wget -q -O $file $url"
    {
      wget -q -O "$file" "$url"
      rc=$?
    } || true
  elif has fetch; then
    cmd="fetch -qo $file $url"
    {
      fetch -qo "$file" "$url"
      rc=$?
    } || true
  else
    error "No HTTP download program (curl, wget, fetch) found, exiting…"
    return 1
  fi

  if [ $rc = 0 ]; then
    return 0
  fi

  error "Command failed (exit code $rc): ${BLUE}${cmd}${NO_COLOR}"
  printf "\n" >&2
  info "This is likely due to Starship not yet supporting your configuration."
  info "If you would like to see a build for your configuration,"
  info "please create an issue requesting a build for ${MAGENTA}${arch}-${platform}${NO_COLOR}:"
  info "${BOLD}${UNDERLINE}https://github.com/starship/starship/issues/new/${NO_COLOR}"
  return 1
}

unpack() {
  archive="$1"
  dir=$(dirname "$1")

  case "$archive" in
    *.tar.gz)
      if [ -n "$verbose" ]; then
        flags="-v"
      else
        flags=
      fi
      tar "$flags" -xzf "$archive" -C "$dir"
      return 0
      ;;
    *.zip)
      if [ -z "$verbose" ]; then
        flags="-qq"
      else
        flags=
      fi
      UNZIP="$flags" unzip "$archive" -d "$dir"
      return 0
      ;;
  esac

  error "Unknown package extension."
  printf "\n" >&2
  info "This almost certainly results from a bug in this script--please file a"
  info "bug report at https://github.com/starship/starship/issues"
  return 1
}

elevate_priv() {
  if ! has sudo; then
    error 'Could not find the command "sudo", needed to get permissions for install.'
    printf "\n" >&2
    info "If you are on Windows, please run your shell as an administrator, then"
    info "rerun this script. Otherwise, please run this script as root, or install"
    info "sudo."
    return 1
  fi

  if ! sudo -v; then
    error "Superuser not granted, aborting installation"
    return 1
  fi

  return 0
}

uname_platform() {
  platform=$(uname -s | tr '[:upper:]' '[:lower:]')

  # check for MUSL
  if [ "$platform" = "linux" ]; then
    if ldd /bin/sh | grep -i musl >/dev/null; then
      platform="unknown-linux-musl"
    else
      platform="unknown-linux-gnu"
    fi
  fi

  # mingw is Git-Bash
  if echo "$platform" | grep -i mingw >/dev/null; then
    platform="pc-windows-msvc"
  fi

  if [ "$platform" = "darwin" ]; then
    platform="apple-darwin"
  fi

  echo $platform
}

platform_check() {
  platform=${platform:-"$(uname_platform)"}
  case "$platform" in
    apple-darwin) return 0 ;;
    pc-windows-msvc) return 0 ;;
    unknown-linux-musl) return 0 ;;
    unknown-linux-gnu) return 0 ;;
  esac

  arch=$(uname_arch)

  error "Builds for $platform are not yet available for Starship"
  printf "\n" >&2
  info "If you would like to see a build for your configuration,"
  info "please create an issue requesting a build for ${MAGENTA}${arch}-${platform}${NO_COLOR}:"
  info "${BOLD}${UNDERLINE}https://github.com/starship/starship/issues/new/${NO_COLOR}"
  return 1
}

uname_arch() {
  arch=$(uname -m | tr '[:upper:]' '[:lower:]')

  # `uname -m` in some cases mis-reports 32-bit OS as 64-bit, so double check
  if [ "$arch" = "x64" ] && [ "$(getconf LONG_BIT)" = "32" ]; then
    arch=i386
  fi

  case $arch in
    x86) arch="i386" ;;
    i686) arch="i386" ;;
    i386) arch="i386" ;;
    aarch64) arch="arm64" ;;
    armv5*) arch="armv5" ;;
    armv6*) arch="armv6" ;;
    armv7*) arch="armv7" ;;
  esac

  echo $arch
}

arch_check() {
  arch=${arch:-"$(uname_arch)"}
  case $arch in
    x86_64) return 0 ;;
  esac

  platform=$(uname_platform)

  error "$arch builds are not yet available for Starship"
  printf "\n" >&2
  info "If you would like to see a build for your configuration,"
  info "please create an issue requesting a build for ${MAGENTA}${arch}-${platform}${NO_COLOR}:"
  info "${BOLD}${UNDERLINE}https://github.com/starship/starship/issues/new/${NO_COLOR}"
  return 1
}

bin_dir_check() {
  if [ -d "$BIN_DIR" ]; then
    return 0
  fi

  error "Installation location $BIN_DIR does not appear to be a directory"
  printf "\n" >&2
  info "Make sure the location exists and is a directory, then try again."
  return 1
}

path_check() {
  # https://stackoverflow.com/a/9663359
  case :$PATH: in
    *:$BIN_DIR:*) return 0 ;;
  esac

  warn "Bin directory $BIN_DIR is not in your \$PATH"
  printf "\n"

  return 0
}

confirm() {
  printf "%s " "${MAGENTA}?${NO_COLOR} $* ${BOLD}[y/N]${NO_COLOR}"
  if ! read -r yn </dev/tty; then
    error "Error reading from prompt (please re-run with the \`--yes\` option)"
    return 1
  fi

  if [ "$yn" != "y" ] && [ "$yn" != "yes" ]; then
    error 'Aborting (please answer "yes" to continue)'
    return 1
  fi

  return 0
}

# defaults
PREFIX=${PREFIX:-"/usr/local"}
BIN_DIR=${BIN_DIR:-"$PREFIX/bin"}

elevated=
force=
verbose=

# parse argv variables
while [ "$#" -gt 0 ]; do
  case "$1" in
    -p | --platform)
      platform="$2"
      shift 2
      ;;
    -b | --bin-dir)
      BIN_DIR="$2"
      shift 2
      ;;
    -a | --arch)
      arch="$2"
      shift 2
      ;;
    -B | --base-url)
      base_url="$2"
      shift 2
      ;;

    -V | --verbose)
      verbose=1
      shift 1
      ;;
    -f | -y | --force | --yes)
      force=1
      shift 1
      ;;

    -p=* | --platform=*)
      platform="${1#*=}"
      shift 1
      ;;
    -b=* | --bin-dir=*)
      BIN_DIR="${1#*=}"
      shift 1
      ;;
    -a=* | --arch=*)
      arch="${1#*=}"
      shift 1
      ;;
    -B=* | --base-url=*)
      base_url="${1#*=}"
      shift 1
      ;;
    -V=* | --verbose=*)
      verbose="${1#*=}"
      shift 1
      ;;
    -f=* | -y=* | --force=* | --yes=*)
      force="${1#*=}"
      shift 1
      ;;

    *)
      error "Unknown option: $1"
      exit 1
      ;;
  esac
done

arch_check
platform_check
bin_dir_check

printf "  %s\n" "${UNDERLINE}Configuration${NO_COLOR}"
info "${BOLD}Bin directory${NO_COLOR}: ${GREEN}${BIN_DIR}${NO_COLOR}"
info "${BOLD}Platform${NO_COLOR}:      ${GREEN}${platform}${NO_COLOR}"
info "${BOLD}Arch${NO_COLOR}:          ${GREEN}${arch}${NO_COLOR}"
printf "\n"

# non-empty `verbose` enables verbose untarring
if [ -n "$verbose" ]; then
  info "${BOLD}Verbose${NO_COLOR}: yes"
  printf "\n"
fi

tmp=$(mktmpdir "starship")
case $platform in
  pc-windows-msvc)
    bin="$tmp/starship.exe"
    ext="zip"
    ;;
  *)
    bin="$tmp/starship"
    ext="tar.gz"
    ;;
esac
archive="$tmp/starship.$ext"

base_url=${base_url:-"https://github.com/starship/starship/releases"}
url="$base_url/latest/download/starship-$arch-$platform.$ext"

info "Tarball URL: ${UNDERLINE}${BLUE}${url}${NO_COLOR}"
if [ -z "$force" ]; then
  confirm "Install Starship ${GREEN}latest${NO_COLOR} to ${BOLD}${GREEN}${BIN_DIR}${NO_COLOR}?"
fi
path_check

if [ -w "$BIN_DIR" ]; then
  info "Installing Starship, please wait…"
else
  warn "Escalated permission are required to install to $BIN_DIR"
  elevate_priv
  info "Installing Starship as root, please wait…"
  elevated=1
fi

fetch "$archive" "$url"
unpack "$archive"
if [ -n "$elevated" ]; then
  sudo install "$bin" "$BIN_DIR"
else
  install "$bin" "$BIN_DIR"
fi

success "Starship installed"
printf "\n"

info "Please follow the steps for your shell to complete the installation:

  ${BOLD}${UNDERLINE}Bash${NO_COLOR}
  Add the following to the end of ${BOLD}~/.bashrc${NO_COLOR}:

      eval \"\$(starship init bash)\"

  ${BOLD}${UNDERLINE}Fish${NO_COLOR}
  Add the following to the end of ${BOLD}~/.config/fish/config.fish${NO_COLOR}:

      starship init fish | source

  ${BOLD}${UNDERLINE}Zsh${NO_COLOR}
  Add the following to the end of ${BOLD}~/.zshrc${NO_COLOR}:

      eval \"\$(starship init zsh)\"

  ${BOLD}${UNDERLINE}Ion${NO_COLOR}
  Add the following to the end of ${BOLD}~/.config/ion/initrc${NO_COLOR}:

      eval \$(starship init ion)
"
