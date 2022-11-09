cargo build --target=wasm32-unknown-unknown --release && wasm-bindgen target/wasm32-unknown-unknown/release/dongnae-fm.wasm --out-dir ./ --web

