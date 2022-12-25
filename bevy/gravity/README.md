
# TODO
## text input
- 실제로 사용자가 입력을 하는게 아니라 < > 버튼을 두고 사용자가 클릭하게 해야겠구나
  - 키패드로 움직일수 있게 해줘야겠네

# wasm
- [여기](https://github.com/Shaun289/rust-toy-projects/tree/gh-pages/lotto) 에서 내용 가져옴


## pre-build

```
$ rustup target add wasm32-unknown-unknown
$ cargo install -f wasm-bindgen-cli
```

## build
### debug

```
$ cargo build --target=wasm32-unknown-unknown
$ wasm-bindgen target/wasm32-unknown-unknown/debug/gravity.wasm --out-dir ./ --web
```

### release

```
$ cargo build --target=wasm32-unknown-unknown --release
$ wasm-bindgen target/wasm32-unknown-unknown/release/gravity.wasm --out-dir ./ --web
```

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

