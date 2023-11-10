#!/usr/bin/env sh

# Usage: install.sh [tag]
#   [tag] is an optional argument. If provided, the script will install the specified tag of nots-cli.
#   Otherwise, it will install the latest tag.
#
# This script will install nots-cli to $HOME/.local/bin/nots-cli
# You can override the installation directory by setting the NOTS_INSTALL_DIR environment variable.
#
# Examples:
#   install.sh
#   install.sh v0.1.4
#   NOTS_INSTALL_DIR=/usr/local/bin install.sh
#
# Based on https://github.com/oven-sh/bun/blob/main/src/cli/install.sh with some modifications
# Licensed under the MIT license <http://opensource.org/licenses/MIT>

set -eu

# Reset
Color_Off=''

# Regular Colors
Red=''
Green=''
Dim=''
Light_Gray=''

# Bold
Bold_White=''
Bold_Green=''

if [ -t 1 ]; then
  # Reset
  Color_Off='\033[0m'
  Red='\033[0;31m'
  Green='\033[0;32m'
  Dim='\033[0;2m'
  Light_Gray='\033[0;37m'
  Bold_Green='\033[1;32m'
  Bold_White='\033[1m'
fi

error() {
  echo -e "${Red}error${Color_Off}:" "$*" >&2
  exit 1
}

info() {
  echo -e "${Dim}$* ${Color_Off}"
}

info_code() {
  echo -e "${Light_Gray}$* ${Color_Off}"
}

info_bold() {
  echo -e "${Bold_White}$* ${Color_Off}"
}

success() {
  echo -e "${Green}$* ${Color_Off}"
}

tildify() {
  case "$1" in
  $HOME/*)
    printf "~/%s\n" "${1#"$HOME"/}"
    ;;
  *)
    echo "$1"
    ;;
  esac
}

GITHUB=${GITHUB-"https://github.com"}
GITHUB_REPO="$GITHUB/explodingcamera/nots"
install_dir=${NOTS_INSTALL_DIR:-"$HOME/.local/bin"}
exe_name=nots-cli
exe="$install_dir/nots"

if [ "${OS:-}" = "Windows_NT" ]; then
  info ${Red}error${Color_Off}: Please install bun using Windows Subsystem for Linux
  info You can also find an experimental Windows build at ${Bold_White}$GITHUB_REPO/releases
  exit 1
fi

command -v nots >/dev/null &&
  info '`nots` executable found at' "$Bold_Green$(tildify "$(command -v nots)")${Color_Off}" &&
  error 'nots is already installed. To upgrade, run "nots upgrade". To continue anyway, remove the nots binary and try again.'

command -v tar >/dev/null ||
  error 'tar is required to install nots'

command -v curl >/dev/null ||
  error 'curl is required to install nots'

command -v grep >/dev/null ||
  error 'grep is required to install nots'

command -v head >/dev/null ||
  error 'head is required to install nots'

if [ "$#" -gt 2 ]; then
  error 'Too many arguments, only one is allowed. The argument can be a specific tag of nots-cli to install. (e.g. "nots-cli-v0.1.4")'
fi

MACHINE_TYPE=$(uname -ms)
target=x86_64-unknown-linux
case "$MACHINE_TYPE" in
'Darwin x86_64')
  target=x86_64-apple-darwin
  ;;
'Darwin arm64')
  target=aarch64-apple-darwin
  ;;
'Linux aarch64' | 'Linux arm64')
  target=aarch64-unknown-linux
  ;;
'Linux x86_64')
  target=x86_64-unknown-linux
  ;;
esac

if [ $target = darwin-x64 ]; then
  # Is this process running in Rosetta?
  # redirect stderr to devnull to avoid error message when not running in Rosetta
  if [ "$(sysctl -n sysctl.proc_translated 2>/dev/null)" = "1" ]; then
    target=darwin-aarch64
    info "Your shell is running in Rosetta 2. Downloading nots for $target instead"
  fi
fi

archive_name="$exe_name-$target.tar.gz"
archive="$install_dir/$archive_name"

latest_tag=""
if [ $# = 0 ]; then
  latest_tag=$(
    curl -s "https://api.github.com/repos/explodingcamera/nots/tags" |
      grep -oP '"name": "\Knots-cli-[^"]+' |
      grep "nots-cli-v[0-9]\+\.[0-9]\+\.[0-9]\+$" |
      head -1
  )
else
  tag=$1
  if ! echo "$tag" | grep -Eq '^nots-cli-v[0-9]+\.[0-9]+\.[0-9]+$'; then
    tag="nots-cli-$tag"
  fi

  if ! echo "$tag" | grep -Eq '^nots-cli-v[0-9]+\.[0-9]+\.[0-9]+$'; then
    error "invalid tag: $tag"
  fi

  latest_tag="$tag"
fi

nots_uri=$GITHUB_REPO/releases/download/$latest_tag/$archive_name

if [ ! -d "$install_dir" ]; then
  mkdir -p "$install_dir"
fi

curl --fail --location --progress-bar --output "$archive" "$nots_uri" ||
  error "failed to download $nots_uri" &&
  echo "If the version you are trying to install was just released, please wait a few minutes and try again."

tar -xOf "$archive" "$exe_name" >"$exe" ||
  error "failed to extract $archive to $install_dir"

chmod +x "$exe" ||
  error "failed to mark $exe as executable"

rm "$archive" ||
  error "failed to remove $archive"

success "Successfully installed $latest_tag to $Bold_Green$(tildify "$exe")${Color_Off}"

if ! echo "$PATH" | grep -q "$install_dir"; then
  echo
  info "You may want to add $Bold_Green$(tildify "$install_dir")${Dim} to your PATH"

  if [ -f ~/.zshrc ]; then
    info "To do so, add the following line to your shell config file:"
    info_code "$ echo 'export PATH=\"\$PATH:$install_dir\"' >> ~/.zshrc"
    info "Then, run the following command to reload your shell:"
    info_code "$ source ~/.zshrc"
  elif [ -f ~/.bashrc ]; then
    info "To do so, add the following line to your shell config file:"
    info_code "echo 'export PATH=\"\$PATH:$install_dir\"' >> ~/.bashrc"
    info "Then, run the following command to reload your shell:"
    info_code "$ source ~/.bashrc"
  elif [ -f ~/.config/fish/config.fish ]; then
    info "To do so, add the following line to your shell config file:"
    info_code "$ echo 'set -gx PATH \"\$PATH\" \"$install_dir\"' >> ~/.config/fish/config.fish"
    info "Then, run the following command to reload your shell:"
    info_code "$ source ~/.config/fish/config.fish"
  else
    echo
    info "Couldn't determine your shell config file. Please manually add the following line to your shell config file:"
    info_code "$ export PATH=\"\$PATH:$install_dir\""
  fi
fi

echo
info "To get started, run:"
echo

info_bold "  nots --help"
echo
