#!/bin/bash

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

set -euo pipefail
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
  printf "${BOLD}${GREY}>${NO_COLOR} $@\n"
}

warn() {
  printf "${YELLOW}! $@${NO_COLOR}\n"
}

error() {
  printf "${RED}x $@${NO_COLOR}\n" >&2
}

complete() {
  printf "${GREEN}✓${NO_COLOR} $@\n"
}

fetch() {
  local command
  if hash curl 2>/dev/null; then
    set +e
    command="curl --silent --fail --location $1"
    curl --silent --fail --location "$1"
    rc=$?
    set -e
  else
    if hash wget 2>/dev/null; then
      set +e
      command="wget -O- -q $1"
      wget -O- -q "$1"
      rc=$?
      set -e
    else
      error "No HTTP download program (curl, wget) found…"
      exit 1
    fi
  fi

  if [ $rc -ne 0 ]; then
    printf "\n" >&2
    error "Command failed (exit code $rc): ${BLUE}${command}${NO_COLOR}"
    printf "\n" >&2
    info "This is likely due to Starship not yet supporting your configuration." >&2
    info "If you would like to see a build for your configuration," >&2
    info "please create an issue requesting a build for ${MAGENTA}${ARCH}-${PLATFORM}${NO_COLOR}:" >&2
    info "${BOLD}${UNDERLINE}https://github.com/starship/starship/issues/new/${NO_COLOR}\n" >&2
    exit $rc
  fi
}

# Currently supporting:
#   - win (Git Bash)
#   - darwin
#   - linux
#   - linux_musl (Alpine)
detect_platform() {
  local platform
  platform="$(uname -s | tr '[:upper:]' '[:lower:]')"

  # check for MUSL
  if [ "${platform}" = "linux" ]; then
    if ldd /bin/sh | grep -i musl >/dev/null; then
      platform=unknown-linux-musl
    fi
  fi

  # mingw is Git-Bash
  if echo "${platform}" | grep -i mingw >/dev/null; then
    platform=pc-windows-msvc
  fi

  if [ "${platform}" = "linux" ]; then
    platform=unknown-linux-gnu
  fi

  if [ "${platform}" = "darwin" ]; then
    platform=apple-darwin
  fi

  echo "${platform}"
}

# Currently supporting:
#   - x86_64
#   - i386
detect_arch() {
  local arch
  arch="$(uname -m | tr '[:upper:]' '[:lower:]')"

  # `uname -m` in some cases mis-reports 32-bit OS as 64-bit, so double check
  if [ "${arch}" = "x64" ] && [ "$(getconf LONG_BIT)" -eq 32 ]; then
    arch=i386
  fi

  echo "${arch}"
}

confirm() {
  if [ -z "${FORCE-}" ]; then
    printf "${MAGENTA}?${NO_COLOR} $@ ${BOLD}[y/N]${NO_COLOR} "	
    set +e
    read -r yn < /dev/tty
    rc=$?
    set -e
    if [ $rc -ne 0 ]; then
      error "Error reading from prompt (please re-run with the \`--yes\` option)"
      exit 1
    fi
    if [ "$yn" != "y" ] && [ "$yn" != "yes" ]; then
      error "Aborting (please answer \"yes\" to continue)"
      exit 1
    fi
  fi
}

check_bin_dir() {
  local bin_dir="$1"

  # https://stackoverflow.com/a/11655875
  local good
  good=$( IFS=:
    for path in $PATH; do
      if [ "${path}" = "${bin_dir}" ]; then
        echo 1
        break
      fi
    done
  )

  if [ "${good}" != "1" ]; then
    warn "Bin directory ${bin_dir} is not in your \$PATH"
  fi
}

# defaults
if [ -z "${PLATFORM-}" ]; then
  PLATFORM="$(detect_platform)"
fi

if [ -z "${BIN_DIR-}" ]; then
  BIN_DIR=/usr/local/bin
fi

if [ -z "${ARCH-}" ]; then
  ARCH="$(detect_arch)"
fi

if [ -z "${BASE_URL-}" ]; then
  BASE_URL="https://github.com/starship/starship/releases"
fi

# parse argv variables
while [ "$#" -gt 0 ]; do
  case "$1" in
    -p|--platform) PLATFORM="$2"; shift 2;;
    -b|--bin-dir) BIN_DIR="$2"; shift 2;;
    -a|--arch) ARCH="$2"; shift 2;;
    -B|--base-url) BASE_URL="$2"; shift 2;;

    -V|--verbose) VERBOSE=1; shift 1;;
    -f|-y|--force|--yes) FORCE=1; shift 1;;

    -p=*|--platform=*) PLATFORM="${1#*=}"; shift 1;;
    -b=*|--bin-dir=*) BIN_DIR="${1#*=}"; shift 1;;
    -a=*|--arch=*) ARCH="${1#*=}"; shift 1;;
    -B=*|--base-url=*) BASE_URL="${1#*=}"; shift 1;;
    -V=*|--verbose=*) VERBOSE="${1#*=}"; shift 1;;
    -f=*|-y=*|--force=*|--yes=*) FORCE="${1#*=}"; shift 1;;

    *) error "Unknown option: $1"; exit 1;;
  esac
done

if [ "${ARCH}" = "i386" ]; then
  error "i386 builds are not yet available for Starship\n"
  info "If you would like to see a build for your configuration,"
  info "please create an issue requesting a build for ${MAGENTA}${ARCH}-${PLATFORM}${NO_COLOR}:"
  info "${BOLD}${UNDERLINE}https://github.com/starship/starship/issues/new/${NO_COLOR}\n"
  exit 1
fi

printf "  ${UNDERLINE}Configuration${NO_COLOR}\n"	
info "${BOLD}Bin directory${NO_COLOR}: ${GREEN}${BIN_DIR}${NO_COLOR}"
info "${BOLD}Platform${NO_COLOR}:      ${GREEN}${PLATFORM}${NO_COLOR}"
info "${BOLD}Arch${NO_COLOR}:          ${GREEN}${ARCH}${NO_COLOR}"

# non-empty VERBOSE enables verbose untarring
if [ -n "${VERBOSE-}" ]; then
  VERBOSE=v
  info "${BOLD}Verbose${NO_COLOR}: yes"
else
  VERBOSE=
fi

echo

EXT=tar.gz
if [ "${PLATFORM}" = win ]; then
  EXT=zip
fi

URL="${BASE_URL}/latest/download/starship-${ARCH}-${PLATFORM}.${EXT}"
info "Tarball URL: ${UNDERLINE}${BLUE}${URL}${NO_COLOR}"
check_bin_dir "${BIN_DIR}"
confirm "Install Starship ${GREEN}latest${NO_COLOR} to ${BOLD}${GREEN}${BIN_DIR}${NO_COLOR}?"

info "Installing Starship, please wait…"

fetch "${URL}" \
  | tar xzf${VERBOSE} - \
    -C "${BIN_DIR}"

complete "Starship installed"
echo
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

"
