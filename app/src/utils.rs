use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{FileReader, Event};

#[allow(dead_code)]
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn load(event: &Event) -> Result<JsValue, JsValue> {
    let target = match event.target() {
        None => return Err(JsValue::NULL),
        Some(t) => t
    };
    let file_reader: FileReader = target.dyn_into()?;
    let result = file_reader.result();
    return result;
}
