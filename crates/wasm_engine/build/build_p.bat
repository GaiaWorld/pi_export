set "cfgPath=../temp/cfg.txt"
call cfg.bat

cd ../
@REM set RUSTFLAGS=--cfg=web_sys_unstable_apis
@REM set RUSTFLAGS=--cfg getrandom_backend="wasm_js"
set RUSTFLAGS=--cfg getrandom_backend="wasm_js"
set RUST_LOG=info

cargo build --lib --release --target wasm32-unknown-unknown --features release
C:\\Users\\chuanyan\\AppData\\Local\\.wasm-pack\\wasm-bindgen-df7a1317af78a1c0\\wasm-bindgen.exe ../../target/wasm32-unknown-unknown/release/pi_wasm_engine.wasm --out-dir p --typescript --target web --out-name wasm_engine
node build/build_wasm.js p wasm_engine
pause;