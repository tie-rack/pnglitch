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
pub struct Options(GlitchOptions);

#[wasm_bindgen]
impl Options {
    pub fn default() -> Options {
        Options(GlitchOptions::default())
    }

    pub fn set_min_glitches(&mut self, n: u32) -> () {
        self.0.min_glitches = n;
    }
    pub fn set_max_glitches(&mut self, n: u32) -> () {
        self.0.max_glitches = n;
    }
    pub fn set_channel_swap_chance(&mut self, chance: f64) -> () {
        self.0.channel_swap_chance = chance;
    }
    pub fn set_darken_chance(&mut self, chance: f64) -> () {
        self.0.darken_chance = chance;
    }
    pub fn set_flip_chance(&mut self, chance: f64) -> () {
        self.0.flip_chance = chance;
    }
    pub fn set_lighten_chance(&mut self, chance: f64) -> () {
        self.0.lighten_chance = chance;
    }
    pub fn set_line_shift_chance(&mut self, chance: f64) -> () {
        self.0.line_shift_chance = chance;
    }
    pub fn set_off_by_one_chance(&mut self, chance: f64) -> () {
        self.0.off_by_one_chance = chance;
    }
    pub fn set_quantize_chance(&mut self, chance: f64) -> () {
        self.0.quantize_chance = chance;
    }
    pub fn set_reverse_chance(&mut self, chance: f64) -> () {
        self.0.reverse_chance = chance;
    }
    pub fn set_shift_channel_chance(&mut self, chance: f64) -> () {
        self.0.shift_channel_chance = chance;
    }
    pub fn set_xor_chance(&mut self, chance: f64) -> () {
        self.0.xor_chance = chance;
    }

    pub fn min_glitches(&self) -> u32 {
        self.0.min_glitches
    }
    pub fn max_glitches(&self) -> u32 {
        self.0.max_glitches
    }
    pub fn channel_swap_chance(&self) -> f64 {
        self.0.channel_swap_chance
    }
    pub fn darken_chance(&self) -> f64 {
        self.0.darken_chance
    }
    pub fn flip_chance(&self) -> f64 {
        self.0.flip_chance
    }
    pub fn lighten_chance(&self) -> f64 {
        self.0.lighten_chance
    }
    pub fn line_shift_chance(&self) -> f64 {
        self.0.line_shift_chance
    }
    pub fn off_by_one_chance(&self) -> f64 {
        self.0.off_by_one_chance
    }
    pub fn quantize_chance(&self) -> f64 {
        self.0.quantize_chance
    }
    pub fn reverse_chance(&self) -> f64 {
        self.0.reverse_chance
    }
    pub fn shift_channel_chance(&self) -> f64 {
        self.0.shift_channel_chance
    }
    pub fn xor_chance(&self) -> f64 {
        self.0.xor_chance
    }
}

#[wasm_bindgen]
pub fn pnglitch(png: &[u8], options: &Options) -> Result<Vec<u8>, JsValue> {
    set_panic_hook();

    let decoder = png::Decoder::new(png);

    let (info, mut reader) = decoder
        .read_info()
        .map_err(|e| JsValue::from(e.to_string()))?;

    let mut buf = vec![0; info.buffer_size()];
    reader
        .next_frame(&mut buf)
        .map_err(|e| JsValue::from(e.to_string()))?;

    let mut out: Vec<u8> = Vec::new();

    let seed = (js_sys::Math::random() * (u64::max_value() as f64)) as u64;
    let mut rng = rand_hc::Hc128Rng::seed_from_u64(seed);

    glitch(&info, &mut buf, &mut rng, &options.0);

    {
        let mut encoder = png::Encoder::new(&mut out, info.width, info.height);
        encoder.set(info.color_type).set(info.bit_depth);
        let mut writer = encoder
            .write_header()
            .map_err(|e| JsValue::from(e.to_string()))?;

        writer
            .write_image_data(&buf)
            .map_err(|e| JsValue::from(e.to_string()))?;
    }

    Ok(out)
}
