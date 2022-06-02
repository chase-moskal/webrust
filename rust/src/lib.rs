
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/callers.js")]
extern "C" {

	#[wasm_bindgen(catch)]
	fn call4(
		this: &js_sys::Function,
		context: &JsValue,
		arg1: &JsValue,
		arg2: &JsValue,
		arg3: &JsValue,
		arg4: &JsValue,
	) -> Result<JsValue, JsValue>;

	#[wasm_bindgen(catch)]
	fn call5(
		this: &js_sys::Function,
		context: &JsValue,
		arg1: &JsValue,
		arg2: &JsValue,
		arg3: &JsValue,
		arg4: &JsValue,
		arg5: &JsValue,
	) -> Result<JsValue, JsValue>;
}

#[wasm_bindgen]
pub fn greet(name: &str, callback: &js_sys::Function) {
	let greeting = &format!("Hello, {}!", name);
	let _ = call4(
		&callback,
		&JsValue::null(),
		&JsValue::from(greeting),
		&JsValue::from("argument2"),
		&JsValue::from("argument3"),
		&JsValue::from("argument4"),
	);
}
