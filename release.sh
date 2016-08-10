#!/bin/sh

set -e

if [ $# -ne 1 ]; then
    echo Usage: $0 "<version>"
    exit 1
fi

VERSION=$1

sed -i.bak "s/version = \".*\"/version = \"$VERSION\"/g" Cargo.toml
rm Cargo.toml.bak
sed -i.bak "s/documentation = \".*\"/documentation = \"https:\/\/kimhyunkang.github.io\/immutable-map-rs\/doc\/v$VERSION\/immutable_map\/\"/g" Cargo.toml
rm Cargo.toml.bak

cargo test

git add .
git commit -m "Release v$VERSION"

cargo doc

git checkout gh-pages
mv target/doc doc/v$VERSION
cp -R doc/v$VERSION/* doc

git add doc
git commit -m "Release v$VERSION"
