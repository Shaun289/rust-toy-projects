cargo build --target=wasm32-unknown-unknown --release && wasm-bindgen target/wasm32-unknown-unknown/release/lotto.wasm --out-dir ./ --web

