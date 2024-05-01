#!/usr/bin/env bash
set -e

# Expects
# OS=linux or darwin
# ARCH=amd64 or 386
OS=$(uname -s)
ARCH=$(uname -m)
if [[ "$OS" == "Linux" ]]; then
	if [[ "$ARCH" == "arm64" ]]; then
		TARGET=aarch64-unknown-linux-gnu
	else
		TARGET=x86_64-unknown-linux-gnu
	fi
elif [[ "$OS" == "Darwin" ]]; then
	if [[ "$ARCH" == "arm64" ]]; then
		TARGET=aarch64-apple-darwin
	else
		TARGET=x86_64-apple-darwin
	fi
	# TODO: add other distributions when available
	# elif [[ "$OSTYPE" == "cygwin" ]]; then
	# 	# POSIX compatibility layer and Linux environment emulation for Windows
	# elif [[ "$OSTYPE" == "msys" ]]; then
	# 	# Lightweight shell and GNU utilities compiled for Windows (part of MinGW)
	# elif [[ "$OSTYPE" == "win32" ]]; then
	# 	# I'm not sure this can happen.
	# elif [[ "$OSTYPE" == "freebsd"* ]]; then
	# 	# ...
	# else
	# Unknown.
fi

if [[ -z "$TARGET" ]]; then
	echo "$TARGET"
	echo "TARGET not set: unsupported"
	exit 1
fi

LATEST_VERSION=$(curl --silent "https://api.github.com/repos/aurora-is-near/near-burrow/releases/latest" | grep -Po '"tag_name": "\K.*?(?=")')
echo $LATEST_VERSION
mkdir -p ~/bin
wget https://github.com/aurora-is-near/near-burrow/releases/download/${LATEST_VERSION}/near-burrow-${TARGET}.tar.gz
tar -zxvf near-burrow-${TARGET}.tar.gz
cp near-burrow-${TARGET}/near-burrow ~/bin/
chmod +x ~/bin/near-burrow
rm -r near-burrow-${TARGET} near-burrow-${TARGET}.tar.gz
