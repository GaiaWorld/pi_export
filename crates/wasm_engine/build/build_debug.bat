cd ../
set RUSTFLAGS=--cfg=web_sys_unstable_apis
set RUST_LOG=info
wasm-pack build --debug  --target web --out-dir pkg_debug --out-name wasm_engine
"C:\\Users\\chuanyan\\AppData\\Local\\.wasm-pack\\wasm-bindgen-7e757a9923d43b8a\\wasm-bindgen.exe" "E:\\pi_export\\target\\wasm32-unknown-unknown\\debug\\pi_wasm_engine.wasm" "--out-dir" "E:\\pi_export\\crates\\wasm_engine\\pkg_debug" "--typescript" "--target" "web" "--out-name" "wasm_engine"
node build/build_wasm.js pkg_debug wasm_engine
pause;
