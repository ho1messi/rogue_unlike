cargo build --release --target wasm32-unknown-unknown
wasm-bindgen target\wasm32-unknown-unknown\release\rogue_unlike.wasm --out-dir wasm --no-modules --no-typescript
miniserve wasm --index index.html