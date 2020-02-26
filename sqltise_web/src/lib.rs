use wasm_bindgen::prelude::*;

// lifted from the `console_log` example
#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_namespace = console)]
	fn log(a: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
	console_log!("running main.");
	// Use `web_sys`'s global `window` function to get a handle on the global
	// window object.
	let window = web_sys::window().expect("no global `window` exists");
	let document = window.document().expect("should have a document on window");
	let body = document.body().expect("document should have a body");

	// Manufacture the element we're gonna append
	let val = document.create_element("p")?;
	val.set_inner_html("Hello from Rust!");

	body.append_child(&val)?;

	Ok(())
}
