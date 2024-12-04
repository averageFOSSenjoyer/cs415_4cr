# Building
```
cargo build --profile release
```

# Running Desktop
```
cargo run --profile release
```

# Running WASM
First install [wasm-server-runner](https://github.com/jakobhellermann/wasm-server-runner)
```
cargo run --profile release-wasm --target wasm32_unknown_unknown
```