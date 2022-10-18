# iced
- [iced v0.4.2 doc](https://docs.rs/iced/0.4.2/iced/)

# pre-build

```
$ rustup target add wasm32-unknown-unknown
$ cargo install -f wasm-bindgen-cli
```

# build

```
$ cargo build --target=wasm32-unknown-unknown
$ wasm-bindgen target/wasm32-unknown-unknown/debug/lotto.wasm --out-dir ./ --web
```

# test
## trunk
- [rust trunk web](https://trunkrs.dev/)

### install trunk

```
$ cargo install --locked trunk
```

### trunk serve

```
$ trunk serve
```
