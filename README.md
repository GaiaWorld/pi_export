
## 检查重复库 
* 安装工具 `cargo install --locked cargo-deny`
* 查看重复库 `cargo deny check bans 2>a.txt`

## Gui测试性能（chrome://tracing）
例： `cargo run --example cmd_play --release --features trace`

## wasm尺寸分析工具 `twiggy`

## 编译为wasm
1. `set RUST_LOG=info`
2. `set RUSTFLAGS=--cfg=web_sys_unstable_apis`
3. 根据需求编译
    + 构建： `cargo build --target wasm32-unknown-unknown`
    + 编译release版本： `wasm-pack build --release  --target web --out-dir pkg_release --out-name gui`
	+ 编译profiling版本： `wasm-pack build --profiling  --target web --out-dir pkg_profiling --out-name gui`
	+ 编译debug版本： `wasm-pack build --debug  --target web --out-dir pkg_pdebug --out-name gui`
4. 编译为pi可用的wasm：wasm_engine中执行编译脚本

## 尺寸优化
+ wgpu和naga feature禁用spirv（少200k）
+ wgpu和naga feature禁用wgsl（少200k）
+ web版本，不调用图片解码（少700k）
+ 不使用bevy中的LogPlugin （少700k）
+ 不使用bevy的window插件（少250k（TODO））