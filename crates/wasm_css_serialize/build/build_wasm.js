
let fs = require("fs");


/**
 * argv: 
 *    [0]: node.exe的路径
 *    [1]: *.js 执行的js的路径
 *    [2..]: 其他参数
 */

// 当前目录，一般是项目地址
let cwd = process.cwd();

var dir = process.argv[2] || "pkg";
var name = process.argv[3] || "gui";
var wasmName = `${name}_bg`;


let in_wasm_path = `${dir}/${wasmName}.wasm`;
let in_wasm_js_path = `${dir}/${name}.js`;
let out_wasm_path = `${dir}/${name}.wasm`;
let out_wasm_js_path = `${dir}/${name}.wasm.ts`;

fs.readFile(in_wasm_js_path, {encoding:"utf8"}, (err, data) => {
	if(!err) {
		data = data.replace(`import.meta.url`, '""');
		data = data.replace(/from\s+'(.+?)\.js'/g,  "from '$1'");
		data = data.replace(/getObject\(arg0\)\sinstanceof\sWindow/g, "true");
		data = data.replace(/getObject\(arg0\)\sinstanceof\sCanvasRenderingContext2D/g, "true");
		data = data.replace(/getObject\(arg0\)\sinstanceof\sHTMLCanvasElement/g, "true");  

		data = data.replace(
	`    const { instance, module } = await __wbg_load(await input, imports);

	return __wbg_finalize_init(instance, module);
}
	
export { initSync }
export default __wbg_init;`, 

`    const r = await __wbg_load(await input, imports);

	__wbg_finalize_init(r.instance, r.module);
	if(module.postRun) {
		module.postRun();
	}

	return wasm;
}

export { initSync }
Promise.resolve().then(() => {
	__wbg_init(module.wasmModule).then((r) => {
		window["_$wasm"] = r;
	});
})
`);
		// data = data.replace(`Module["noExitRuntime"]=true;run();`, `Module["noExitRuntime"] = true;
		// //PI_START
		// run();
		// window.Module = Module;
		// // run();
		// //PI_END
		// `);

		fs.writeFile(out_wasm_js_path, data, {encoding:"utf8"}, (err) => {
			if(err) {
				console.log("写文件失败！！", JSON.stringify(err));
			}
		});
	} else {
		console.log("读文件失败！！", JSON.stringify(err));
	}
});

fs.readFile(in_wasm_path, (err, data) => {
	if(!err) {
		fs.writeFile(out_wasm_path, data, (err) => {
			if(err) {
				console.log("写文件失败！！", JSON.stringify(err));
			}
		})
	} else {
		console.log("读文件失败！！", JSON.stringify(err));
	}
});