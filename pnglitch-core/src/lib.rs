pub mod effects;
use effects::Glitch;

extern crate png;

extern crate rand;
use rand::distributions::{Distribution, Uniform};
use rand::Rng;

pub struct GlitchOptions {
    pub min_glitches: u32,
    pub max_glitches: u32,
}

impl Default for GlitchOptions {
    fn default() -> GlitchOptions {
        GlitchOptions {
            min_glitches: 1,
            max_glitches: 5,
        }
    }
}

pub fn glitch(
    png_info: &png::OutputInfo,
    pixel_buf: &mut [u8],
    rng: &mut impl Rng,
    options: &GlitchOptions,
) -> () {
    let chunk_count_dist = Uniform::from(options.min_glitches..=options.max_glitches);

    let glitch_count = chunk_count_dist.sample(rng);

    for _ in 0..glitch_count {
        glitch_chunk(&png_info, pixel_buf, rng);
    }
}

fn glitch_chunk(png_info: &png::OutputInfo, pixel_buf: &mut [u8], rng: &mut impl Rng) -> () {
    let line_count = pixel_buf.len() / png_info.line_size;
    let channel_count = match png_info.color_type {
        png::ColorType::Grayscale => 1,
        png::ColorType::RGB => 3,
        png::ColorType::Indexed => 1,
        png::ColorType::GrayscaleAlpha => 2,
        png::ColorType::RGBA => 4,
    };

    let line_number_dist = Uniform::from(0..line_count);
    let line_shift_dist = Uniform::from(0..png_info.line_size);
    let xor_dist = Uniform::from(0..255);
    let channel_count_dist = Uniform::from(0..channel_count);

    let first_line = line_number_dist.sample(rng);
    let chunk_size = line_number_dist.sample(rng) / 2;
    let last_line = if (first_line + chunk_size) > line_count {
        line_count
    } else {
        first_line + chunk_size
    };

    let line_shift_amount = line_shift_dist.sample(rng);

    let xor_value = xor_dist.sample(rng);

    let reverse = rng.gen_bool(0.3);

    let lighten = rng.gen_bool(0.15);
    let darken = rng.gen_bool(0.15);

    let quantize = rng.gen_bool(0.2);

    let flip = rng.gen_bool(0.2);

    let shift_channel = if rng.gen_bool(0.3) {
        let amount = line_shift_dist.sample(rng) / channel_count;
        let channel = channel_count_dist.sample(rng);
        Some(effects::LineGlitch::ChannelShift(
            amount,
            channel,
            channel_count,
        ))
    } else {
        None
    };

    let channel_swap = if rng.gen_bool(0.3) {
        let channel_1 = channel_count_dist.sample(rng);
        let channel_2 = channel_count_dist.sample(rng);
        Some(effects::ChunkGlitch::ChannelSwap(
            channel_1,
            channel_2,
            channel_count,
        ))
    } else {
        None
    };

    let line_shift = effects::LineGlitch::Shift(line_shift_amount);

    // line-level effects
    for line_number in first_line..last_line {
        let line_start = line_number * png_info.line_size;
        let line_end = line_start + png_info.line_size;

        let line = &mut pixel_buf[line_start..line_end];

        if let Some(shift_channel) = &shift_channel {
            shift_channel.run(line);
        }

        line_shift.run(line);

        if reverse {
            effects::LineGlitch::Reverse.run(line);
        }
    }

    // chunk-level effects
    let chunk_start = first_line * png_info.line_size;
    let chunk_end = last_line * png_info.line_size;

    let chunk = &mut pixel_buf[chunk_start..chunk_end];

    effects::ChunkGlitch::XOR(xor_value).run(chunk);

    if let Some(cs) = channel_swap {
        cs.run(chunk)
    };

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
