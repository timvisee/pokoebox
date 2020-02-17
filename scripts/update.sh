#!/bin/bash

set -e

ARCH="armv7-linux-gnueabihf"

# Change to script directory
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd $DIR

# Select and create temp dir
DL_DIR=/tmp/pokoebox-update-$RANDOM
mkdir -p $DL_DIR

# Download newest version
echo "Downloading newest version..."
curl -SL "https://gitlab.com/timvisee/pokoebox/-/jobs/artifacts/master/download?job=build-$ARCH" -o $DL_DIR/pokoebox.zip

# Unzip player binary from download
echo "Extracting..."
unzip -t $DL_DIR/pokoebox.zip
unzip $DL_DIR/pokoebox.zip 'pokoebox-player-*' -d $DL_DIR
mv $DL_DIR/pokoebox-player-* $DL_DIR/pokoebox-player
chmod a+x $DL_DIR/pokoebox-player

# Update installation
echo "Installing..."
mv -f $DL_DIR/pokoebox-player ./pokoebox-player

# Delete temp dir
rm -rf $DL_DIR

echo "Done."
