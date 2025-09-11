set "cfgPath=../temp/cfg.txt"
call cfg.bat

cd ../
set RUSTFLAGS=--cfg=web_sys_unstable_apis
set RUSTFLAGS=--cfg getrandom_backend="wasm_js"
@REM  // debug 下重新编译std标准库会wasm下会导致内存对不齐错误,但是不重新编译std标准库无法完全关闭reference-types,sign-ext优化，所以微信小游戏下默认不运行debug版本
@REM set CARGO_UNSTABLE_BUILD_STD=panic_abort,std
set RUST_LOG=info
wasm-pack build --debug  --target web --out-dir pkg_debug --out-name wasm_engine

C:\Users\0002\AppData\Local\.wasm-pack\wasm-bindgen-53edf4f5acf7b49d\wasm-bindgen.exe ../../target/wasm32-unknown-unknown/release/pi_wasm_engine.wasm --out-dir pkg_profiling --typescript --target web --out-name wasm_engine
node build/build_wasm.js pkg_debug wasm_engine
pause;

