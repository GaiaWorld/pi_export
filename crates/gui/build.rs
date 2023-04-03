fn main() -> Result<(), std::io::Error> {


    // 除非修改build.rs， 否则不重新运行脚本
    println!("cargo:rerun-if-changed=build.rs");
    // visit_dirs("src/shader/", &mut |file| {
    //     if let Some(r) = file.extension() {
    //         let r = r.to_string_lossy();
    //         if r.ends_with("glsl") || r.ends_with("vert") || r.ends_with("frag") {
    //             println!("cargo:rerun-if-changed={:?}", file);
    //         }
    //     }
    // });

	// style 宏展开
	let out = std::process::Command::new("cargo")
			.current_dir("exports_macro")
			.args(["expand", "style_macro", "--target", "wasm32-unknown-unknown"])
            .output()
            .expect("failed cargo expand")
			.stdout;
	let s = String::from_utf8(out).expect("failed from_utf8");
	let first_line = s.find("{").expect("failed {");
	let last_close = s.rfind("}").expect("failed }");

	std::fs::write("src/style.rs", &s[first_line + 1 ..last_close])?;
	Ok(())
}
