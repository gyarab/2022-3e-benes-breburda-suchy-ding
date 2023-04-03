#!/bin/bash

set -x -e

PROJECT=ding
REF=HEAD
test -n "$1" && REF="$1"

TMP=`mktemp -d`
CWD=`pwd`
VERSION=`git describe --always "$REF"`
DATE=`date +%Y%m%d%H%M`

# prepare sqlx offline mode data
pushd back
cargo sqlx prepare
popd

git archive "$REF" | tar x -C "$TMP"
pushd "$TMP"

# sqlx offline mode
cp "$CWD/back/sqlx-data.json" back

mkdir -p dist 

# prepare server binary
pushd back

cross build --release
strip target/x86_64-unknown-linux-gnu/release/backend

cp target/x86_64-unknown-linux-gnu/release/backend ../dist/server
rm -r target  # free up space

popd

# build frontend
pushd front

npm ci --audit false --fund false --prefer-offline true
npm run build

mkdir -p ../dist/public
cp -r dist/* ../dist/public

popd

cp -r back/sql dist

popd

mkdir -p dist
tar czv -f dist/$PROJECT-$DATE-$VERSION.tar.gz -C "$TMP/dist" \
    server sql public

ln -snf $PROJECT-$DATE-$VERSION.tar.gz dist/latest.tar.gz

rm -r "$TMP"
