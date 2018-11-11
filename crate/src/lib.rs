use std::ops::BitXor;

#[macro_use]
extern crate cfg_if;
extern crate js_sys;
extern crate web_sys;
extern crate wasm_bindgen;

extern crate png;
use png::HasParameters;

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

fn scaled_random(max: usize) -> usize {
    (js_sys::Math::random() * (max as f64)) as usize
}

fn shift_chunk(buf: &mut [u8], line_size: usize) -> () {
    let line_count = buf.len() / line_size;

    let first_line = scaled_random(line_count);
    let chunk_size = scaled_random(line_count / 2);

    let last_line = if (first_line + chunk_size) > line_count {
        line_count
    } else {
        first_line + chunk_size
    };

    let line_shift_amount = scaled_random(line_size);

    let xor_value = scaled_random(256) as u8;

    let reverse = js_sys::Math::random() < 0.3;

    for line_number in first_line..last_line {
        let line_start = line_number * line_size;
        let line_end = line_start + line_size;

        if let Some(line) = buf.get_mut(line_start..line_end) {
            line.rotate_left(line_shift_amount);

            for val in line.iter_mut() {
                *val = val.bitxor(xor_value);
            }

            if reverse {
                line.reverse();
            }
        }
    }
}

#[wasm_bindgen]
pub fn pnglitch(png: &[u8]) -> Vec<u8> {
    set_panic_hook();

    let decoder = png::Decoder::new(png);

    let (info, mut reader) = decoder.read_info().unwrap();

    let mut buf = vec![0; info.buffer_size()];
    reader.next_frame(&mut buf).unwrap();

    let glitch_count = scaled_random(5) + 1;

    for _ in 0..glitch_count {
        shift_chunk(&mut buf, info.line_size);
    }

    let mut out: Vec<u8> = Vec::new();

    {
        let mut encoder = png::Encoder::new(&mut out, info.width, info.height);
        encoder.set(info.color_type).set(info.bit_depth);
        let mut writer = encoder.write_header().unwrap();

        writer.write_image_data(&buf).unwrap();
    }

    out
}
