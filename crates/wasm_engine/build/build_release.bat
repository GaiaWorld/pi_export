call cfg.bat
cd ../
set RUSTFLAGS=--cfg=web_sys_unstable_apis
set RUSTFLAGS=-Zlocation-detail=none
set RUST_LOG=info
wasm-pack build --release  --target web --out-dir pkg --out-name wasm_engine
node build/build_wasm.js pkg wasm_engine
pause;