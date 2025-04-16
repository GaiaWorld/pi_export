set "cfgPath=../temp/cfg_const_memory.txt"
call cfg.bat

cd ../
set RUSTFLAGS=--cfg=web_sys_unstable_apis
set RUSTFLAGS=--cfg getrandom_backend="wasm_js"
set RUST_LOG=info
wasm-pack build --profiling  --target web --out-dir pkg_profiling_const_memery --out-name wasm_engine --features "release,const_memory"
C:\\Users\\chuanyan\\AppData\\Local\\.wasm-pack\\wasm-bindgen-53edf4f5acf7b49d\\wasm-bindgen.exe ../../target/wasm32-unknown-unknown/release/pi_wasm_engine.wasm --out-dir pkg_profiling_const_memery --typescript --target web --out-name wasm_engine
node build/build_wasm.js pkg_profiling_const_memery wasm_engine temp/cfg_const_memory.txt
pause;


