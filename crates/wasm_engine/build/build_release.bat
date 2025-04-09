set "cfgPath=../temp/cfg.txt"
call cfg.bat
cd ../
@REM set RUSTFLAGS=--cfg=web_sys_unstable_apis
@REM set RUSTFLAGS=-Zlocation-detail=none
set RUSTFLAGS=--cfg getrandom_backend="wasm_js"
set RUST_LOG=info
wasm-pack build --release  --target web --out-dir pkg --out-name wasm_engine
node build/build_wasm.js pkg wasm_engine
pause;