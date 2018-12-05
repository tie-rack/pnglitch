#[macro_use]
extern crate cfg_if;
extern crate js_sys;
extern crate wasm_bindgen;

extern crate pnglitch_core;
use pnglitch_core::{glitch, GlitchOptions};

extern crate png;
use png::HasParameters;

extern crate rand;
use rand::SeedableRng;

extern crate rand_hc;

use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        use console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        fn set_panic_hook() {}
    }
}

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub fn pnglitch(png: &[u8]) -> Result<Vec<u8>, JsValue> {
    set_panic_hook();

    let decoder = png::Decoder::new(png);

    let (info, mut reader) = decoder.read_info().map_err(|e| JsValue::from(e.to_string()))?;

    let mut buf = vec![0; info.buffer_size()];
    reader.next_frame(&mut buf).map_err(|e| JsValue::from(e.to_string()))?;

    let mut out: Vec<u8> = Vec::new();

    let options = GlitchOptions::default();

    let seed = (js_sys::Math::random() * (u64::max_value() as f64)) as u64;
    let mut rng = rand_hc::Hc128Rng::seed_from_u64(seed);

    glitch(&info, &mut buf, &mut rng, &options);

    {
        let mut encoder = png::Encoder::new(&mut out, info.width, info.height);
        encoder.set(info.color_type).set(info.bit_depth);
        let mut writer = encoder.write_header().map_err(|e| JsValue::from(e.to_string()))?;

        writer.write_image_data(&buf).map_err(|e| JsValue::from(e.to_string()))?;
    }

    Ok(out)
}
