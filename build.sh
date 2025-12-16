cargo fmt
cargo build --package app-core
cargo build --package app-gui
cargo build --package app-cli
rm -f build/*
mv target/debug/app-gui.exe build/app-gui.exe
mv target/debug/app-cli.exe build/app-cli.exe
