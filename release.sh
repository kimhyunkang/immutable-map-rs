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
mv target/doc/immutable_map doc/v$VERSION
rm doc/current
ln -s doc/v$VERSION/release

git add doc
git commit -m "Release v$VERSION"
