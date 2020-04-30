mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use js_sys::{Promise, ArrayBuffer};
use web_sys::{Blob, FileReader, Event, OfflineAudioContext};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn read_file(blob: Blob) -> Promise {
    return Promise::new(&mut |resolve, reject| {
        // See: https://github.com/rustwasm/wasm-bindgen/issues/1292#issuecomment-466794232
        let file_reader = match FileReader::new() {
            Ok(r) => r,
            Err(_) => return
        };
        let onload = Closure::wrap(Box::new(move |event: Event| {
            match utils::load(&event) {
                Ok(r) => resolve.call1(&JsValue::NULL, &r),
                Err(e) => reject.call1(&JsValue::NULL, &e)
            }.unwrap_or(JsValue::NULL);
        }) as Box<dyn FnMut(_)>);
        file_reader.set_onload(Some(onload.as_ref().unchecked_ref()));
        onload.forget();
        file_reader.read_as_array_buffer(&blob).unwrap_or_default();
    });
}

#[wasm_bindgen]
pub fn decode_audio(buffer: &ArrayBuffer) -> Promise {
    // See: https://github.com/magenta/magenta-js/blob/master/music/src/core/audio_utils.ts#L78
    let context = match OfflineAudioContext::new_with_number_of_channels_and_length_and_sample_rate(
        1, 16000, 16000.0
    ) {
        Ok(c) => c,
        Err(e) => return Promise::reject(&e)
    };
    return match context.decode_audio_data(buffer) {
        Ok(p) => p,
        Err(e) => return Promise::reject(&e)
    };
}
