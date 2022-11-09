
# iced
- [iced crates](https://crates.io/crates/iced)
- [iced github](https://github.com/iced-rs/iced)
- [iced v0.4.2 doc](https://docs.rs/iced/0.4.2/iced/)

# Build

```
$ cargo build --target=wasm32-unknown-unknown
$ wasm-bindgen target/wasm32-unknown-unknown/debug/dongnae-fm.wasm --out-dir ./ --web
```

