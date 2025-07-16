set "cfgPath=../temp/cfg_const_memory.txt"
call cfg.bat
cd ../
@REM set RUSTFLAGS=--cfg=web_sys_unstable_apis
@REM set RUSTFLAGS=-Zlocation-detail=none
@REM 微信小游戏不支持-reference-types,-sign-ext优化，  参数参考https://blog.rust-lang.org/2024/09/24/webassembly-targets-change-in-default-target-features.html
set RUSTFLAGS=--cfg getrandom_backend="wasm_js" -Ctarget-feature=-reference-types,-sign-ext
set CARGO_UNSTABLE_BUILD_STD=panic_abort,std
set CARGO_PROFILE_RELEASE_LTO=true
set RUST_LOG=info
wasm-pack build --release  --features const_memory --target web --out-dir pkg_const_memery --out-name wasm_engine
node build/build_wasm.js pkg_const_memery wasm_engine temp/cfg_const_memory.txt
pause;