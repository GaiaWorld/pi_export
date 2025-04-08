call cfg.bat

cd ../
set RUSTFLAGS=--cfg=web_sys_unstable_apis
set RUSTFLAGS=--cfg getrandom_backend="wasm_js" -Ctarget-feature=-reference-types
set CARGO_UNSTABLE_BUILD_STD=panic_abort,std
set RUST_LOG=info
wasm-pack build --debug  --target web --out-dir pkg_debug --out-name wasm_engine
node build/build_wasm.js pkg_debug wasm_engine
pause;

