
#!/bin/bash
set -e

PREFIX="$HOME/.local"
BINDIR=$PREFIX/bin
SHAREDIR=$PREFIX/share/setrcomp/templates

cargo build --release

echo "Installing binary"
sudo mkdir -p $BINDIR
cp target/release/setrcomp $BINDIR

echo "Installing templates"
mkdir -p $SHAREDIR
cp -r templates/* $SHAREDIR

