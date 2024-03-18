cd ../
set RUSTFLAGS=--cfg=web_sys_unstable_apis
wasm-pack build --release  --target nodejs --out-dir pkg --out-name pi_css_serialize_wasm
node build/build_wasm.js pkg pi_css_serialize_wasm
pause;