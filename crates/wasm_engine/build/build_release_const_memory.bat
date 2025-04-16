set "cfgPath=../temp/cfg_const_memory.txt"
call cfg.bat
cd ../
@REM set RUSTFLAGS=--cfg=web_sys_unstable_apis
@REM set RUSTFLAGS=-Zlocation-detail=none
set RUSTFLAGS=--cfg getrandom_backend="wasm_js"
set RUST_LOG=info
wasm-pack build --release  --features const_memory --target web --out-dir pkg_const_memery --out-name wasm_engine
node build/build_wasm.js pkg_const_memery wasm_engine temp/cfg_const_memory.txt
pause;