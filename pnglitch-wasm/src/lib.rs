#[macro_use]
extern crate cfg_if;
extern crate js_sys;
extern crate pnglitch_core;
extern crate wasm_bindgen;

use pnglitch_core::effects;
use pnglitch_core::effects::Glitch;

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

fn glitch_chunk(buf: &mut [u8], line_size: usize, color_type: png::ColorType) -> () {
    let line_count = buf.len() / line_size;

    let first_line = scaled_random(line_count);
    let chunk_size = scaled_random(line_count / 2);

    let last_line = if (first_line + chunk_size) > line_count {
        line_count
    } else {
        first_line + chunk_size
    };

    let line_shift = effects::LineGlitch::Shift(scaled_random(line_size));

    let xor_value = scaled_random(256) as u8;

    let darken = js_sys::Math::random() < 0.15;
    let lighten = js_sys::Math::random() < 0.15;

    let quantize = js_sys::Math::random() < 0.2;

    let reverse = js_sys::Math::random() < 0.3;

    let flip = js_sys::Math::random() < 0.3;

    let shift_channel = if js_sys::Math::random() < 0.3 {
        let channel_count = match color_type {
            png::ColorType::Grayscale => 1,
            png::ColorType::RGB => 3,
            png::ColorType::Indexed => 1,
            png::ColorType::GrayscaleAlpha => 2,
            png::ColorType::RGBA => 4,
        };
        let amount = scaled_random(line_size) / channel_count;
        let channel = scaled_random(channel_count);
        Some(effects::LineGlitch::ChannelShift(
            amount,
            channel,
            channel_count,
        ))
    } else {
        None
    };

    for line_number in first_line..last_line {
        let line_start = line_number * line_size;
        let line_end = line_start + line_size;

        if let Some(line) = buf.get_mut(line_start..line_end) {
            if let Some(shift_channel) = &shift_channel {
                shift_channel.run(line);
            }

            line_shift.run(line);

            if reverse {
                effects::LineGlitch::Reverse.run(line);
            }
        }
    }

    let chunk_start = first_line * line_size;
    let chunk_end = last_line * line_size;

    let chunk = buf.get_mut(chunk_start..chunk_end).unwrap();

    effects::ChunkGlitch::XOR(xor_value).run(chunk);

    if lighten {
        effects::ChunkGlitch::Lighten.run(chunk);
    }

    if darken {
        effects::ChunkGlitch::Darken.run(chunk);
    }

    if quantize {
        effects::ChunkGlitch::Quantize.run(chunk);
    }

    if flip {
        effects::ChunkGlitch::Flip.run(chunk);
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
        glitch_chunk(&mut buf, info.line_size, info.color_type);
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
