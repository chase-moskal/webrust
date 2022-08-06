
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str, callback: &js_sys::Function) {
	let greeting = &format!("Hello, {}!", name);
	let this = &JsValue::null();
	let _ = callback.call1(this, &JsValue::from(greeting));
}

#[wasm_bindgen]
pub fn sum(a: i32, b: i32) -> i32 {
	return a + b
}
