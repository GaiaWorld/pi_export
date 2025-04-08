call cfg.bat
cd ../
@REM set RUSTFLAGS=--cfg=web_sys_unstable_apis
@REM set RUSTFLAGS=-Zlocation-detail=none
set RUSTFLAGS=--cfg getrandom_backend="wasm_js" -Ctarget-feature=-reference-types
set CARGO_UNSTABLE_BUILD_STD=panic_abort,std
set RUST_LOG=info
wasm-pack build --release  --target web --out-dir pkg --out-name wasm_engine
node build/build_wasm.js pkg wasm_engine
pause;