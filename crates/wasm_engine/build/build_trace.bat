set "cfgPath=../temp/cfg.txt"
call cfg.bat

cd ../
set RUSTFLAGS=--cfg=web_sys_unstable_apis
set RUSTFLAGS=--cfg getrandom_backend="wasm_js" -Ctarget-feature=-reference-types
set CARGO_UNSTABLE_BUILD_STD=panic_abort,std
set RUST_LOG=info
wasm-pack build --profiling  --target web --out-dir pkg_trace --out-name wasm_engine --features trace
C:\\Users\\chuanyan\\.cargo\\bin\\wasm-bindgen.exe ../../target/wasm32-unknown-unknown/release/pi_wasm_engine.wasm --out-dir pkg_profiling --typescript --target web --out-name wasm_engine
node build/build_wasm.js pkg_trace wasm_engine
pause;
