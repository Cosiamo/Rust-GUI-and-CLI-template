cargo fmt
cargo build --package app-core
cargo build --package gui-bindings
cargo build --package cli
rm -f app/*
mv target/debug/gui-bindings.exe app/gui-bindings.exe
mv target/debug/cli.exe app/cli.exe