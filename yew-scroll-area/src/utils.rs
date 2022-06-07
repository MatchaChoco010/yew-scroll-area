use js_sys::{Boolean, JsString, Reflect};
use wasm_bindgen::{JsValue, UnwrapThrowExt};
use yew::prelude::*;

pub fn set_event_custom_flag(evt: &Event, name: &str, flag: bool) {
    Reflect::set(&evt, &JsString::from(name), &Boolean::from(flag))
        .expect_throw("Failed to set {name}");
}

pub fn get_event_custom_flag(evt: &Event, name: &str) -> Option<bool> {
    let flag = Reflect::get(&evt, &JsString::from(name)).expect_throw("Failed to get {name}");
    if flag == JsValue::UNDEFINED {
        None
    } else {
        Some(flag.as_bool().expect_throw("Failed to get {name} as bool"))
    }
}
