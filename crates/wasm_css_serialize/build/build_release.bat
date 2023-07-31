cd ../
wasm-pack build --release  --target nodejs --out-dir pkg --out-name pi_css_serialize_wasm
node build/build_wasm.js pkg pi_css_serialize_wasm
pause;