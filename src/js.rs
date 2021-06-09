use serde::Deserialize;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_namespace = game)]
	pub fn hide_cursor(hidden: bool);
	#[wasm_bindgen(js_namespace = game)]
	pub fn get_cursor_movement() -> JsValue;
	#[wasm_bindgen(js_namespace = game)]
	pub fn get_focus() -> bool;
}

#[derive(Deserialize)]
pub struct JsVector2 {
	pub x: f32,
	pub y: f32,
}
