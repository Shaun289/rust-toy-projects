# Bevy snakegame
- [Creating a Snake Clone in Rust, with Bevy](https://mbuffett.com/posts/bevy-snake-tutorial/)
- bevy 0.7 기준으로 만들어진 게임을 0.9 기준으로 포팅함

# wasm [snakegame](https://shaun289.github.io/rust-toy-projects/bevy/snakegame/)
- index.html 은 iced  공부할때 보던 [문서](https://blog.logrocket.com/iced-rs-tutorial-rust-frontend-web-app/)에서 가져옴

## build
### debug

```
$ cargo build --target=wasm32-unknown-unknown
$ wasm-bindgen target/wasm32-unknown-unknown/debug/snakegame.wasm --out-dir ./ --web
```

### release

```
$ cargo build --target=wasm32-unknown-unknown --release
$ wasm-bindgen target/wasm32-unknown-unknown/release/snakegame.wasm --out-dir ./ --web
```

# TODO
## Bug
- can spawn food in the same position of tail

## Enhance
- display
    - current tail count
    - winning count
