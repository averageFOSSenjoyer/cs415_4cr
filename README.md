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

Then,
```
cargo run --profile release-wasm --target wasm32-unknown-unknown
```