#!/bin/bash
# set -euxo pipefail

# いつかpackage nameを取得

paths=(
    "./target/release/langton_ant"
    "./target/x86_64-apple-darwin/release/langton_ant"
    "./target/x86_64-unknown-linux-gnu/release/langton_ant"
    "./target/x86_64-pc-windows-gnu/release/langton_ant.exe"
)

#build
cargo fmt

cargo build --release
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target x86_64-apple-darwin
cargo build --release --target x86_64-pc-windows-gnu

rm -r ./release/
mkdir ./release

zip -j -9 ./release/langton_ant_mac_arm.zip ./README.md ./.env ./target/release/langton_ant
zip -j -9 ./release/langton_ant_mac_x86_64.zip ./README.md ./.env ./target/x86_64-apple-darwin/release/langton_ant
zip -j -9 ./release/langton_ant_linux_x86_64.zip ./README.md ./.env ./target/x86_64-unknown-linux-gnu/release/langton_ant
zip -j -9 ./release/langton_ant_win_x86_64.zip ./README.md ./.env ./target/x86_64-pc-windows-gnu/release/langton_ant.exe

# for file_path in "${paths[@]}"
# do
#     echo ${file_path}
#     zip x86_64_linux.zip 
# done