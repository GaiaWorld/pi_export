set "cfgPath=../temp/cfg.txt"
call cfg.bat

cd ../
set RUSTFLAGS=--cfg=web_sys_unstable_apis
set RUSTFLAGS=--cfg getrandom_backend="wasm_js"
set RUST_LOG=info
wasm-pack build --profiling  --target web --out-dir pkg_profiling --out-name wasm_engine --features release
C:\\Users\\chuanyan\\AppData\\Local\\.wasm-pack\\wasm-bindgen-53edf4f5acf7b49d\\wasm-bindgen.exe ../../target/wasm32-unknown-unknown/release/pi_wasm_engine.wasm --out-dir pkg_profiling --typescript --target web --out-name wasm_engine
node build/build_wasm.js pkg_profiling wasm_engine
pause;


