pub mod effects;
use effects::Glitch;

extern crate png;

extern crate rand;
use rand::distributions::{Distribution, Uniform};
use rand::Rng;

pub struct GlitchOptions {
    pub min_glitches: u32,
    pub max_glitches: u32,
    pub channel_swap_chance: f64,
    pub darken_chance: f64,
    pub flip_chance: f64,
    pub lighten_chance: f64,
    pub line_shift_chance: f64,
    pub off_by_one_chance: f64,
    pub quantize_chance: f64,
    pub reverse_chance: f64,
    pub shift_channel_chance: f64,
    pub xor_chance: f64,
}

impl Default for GlitchOptions {
    fn default() -> GlitchOptions {
        GlitchOptions {
            min_glitches: 1,
            max_glitches: 6,
            channel_swap_chance: 0.3,
            darken_chance: 0.15,
            flip_chance: 0.2,
            lighten_chance: 0.15,
            line_shift_chance: 0.9,
            off_by_one_chance: 0.2,
            quantize_chance: 0.2,
            reverse_chance: 0.3,
            shift_channel_chance: 0.3,
            xor_chance: 0.8,
        }
    }
}

pub fn glitch(
    png_info: &png::OutputInfo,
    pixel_buf: &mut [u8],
    rng: &mut impl Rng,
    options: &GlitchOptions,
) {
    let chunk_count_dist = Uniform::from(options.min_glitches..=options.max_glitches);

    let glitch_count = chunk_count_dist.sample(rng);

    for _ in 0..glitch_count {
        glitch_chunk(&png_info, pixel_buf, rng, options);
    }
}

fn glitch_chunk(
    png_info: &png::OutputInfo,
    pixel_buf: &mut [u8],
    rng: &mut impl Rng,
    options: &GlitchOptions,
) {
    let line_count = pixel_buf.len() / png_info.line_size;
    let channel_count = match png_info.color_type {
        png::ColorType::Grayscale => 1,
        png::ColorType::RGB => 3,
        png::ColorType::Indexed => 1,
        png::ColorType::GrayscaleAlpha => 2,
        png::ColorType::RGBA => 4,
    };

    let line_shift_dist = Uniform::from(0..png_info.line_size);
    let line_number_dist = Uniform::from(0..line_count);
    let channel_count_dist = Uniform::from(0..channel_count);

    let first_line = line_number_dist.sample(rng);
    let chunk_size = line_number_dist.sample(rng) / 2;
    let last_line = if (first_line + chunk_size) > line_count {
        line_count
    } else {
        first_line + chunk_size
    };

    let reverse = rng.gen_bool(options.reverse_chance);

    let lighten = rng.gen_bool(options.lighten_chance);
    let darken = rng.gen_bool(options.darken_chance);

    let off_by_one = rng.gen_bool(options.off_by_one_chance);

    let quantize = rng.gen_bool(options.quantize_chance);

    let flip = rng.gen_bool(options.flip_chance);

    let shift_channel = if rng.gen_bool(options.shift_channel_chance) {
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

    let channel_swap = if rng.gen_bool(options.channel_swap_chance) {
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

    let line_shift = if rng.gen_bool(options.line_shift_chance) {
        let line_shift_amount = line_shift_dist.sample(rng);

        Some(effects::LineGlitch::Shift(line_shift_amount))
    } else {
        None
    };

    // line-level effects
    for line_number in first_line..last_line {
        let line_start = line_number * png_info.line_size;
        let line_end = line_start + png_info.line_size;

        let line = &mut pixel_buf[line_start..line_end];

        if let Some(shift_channel) = &shift_channel {
            shift_channel.run(line);
        }

        if let Some(ls) = &line_shift {
            ls.run(line);
        }

        if reverse {
            effects::LineGlitch::Reverse.run(line);
        }
    }

    // chunk-level effects
    let chunk_start = first_line * png_info.line_size;
    let chunk_end = last_line * png_info.line_size;

    let chunk = &mut pixel_buf[chunk_start..chunk_end];

    if rng.gen_bool(options.xor_chance) {
        let xor_dist = Uniform::from(0..255);
        let xor_value = xor_dist.sample(rng);

        effects::ChunkGlitch::XOR(xor_value).run(chunk);
    }

    if let Some(cs) = channel_swap {
        cs.run(chunk)
    };

    if lighten {
        effects::ChunkGlitch::Lighten.run(chunk);
    }

    if darken {
        effects::ChunkGlitch::Darken.run(chunk);
    }

    if off_by_one {
        effects::ChunkGlitch::OffByOne(last_line - first_line, png_info.line_size, channel_count)
            .run(chunk);
    }

    if quantize {
        effects::ChunkGlitch::Quantize.run(chunk);
    }

    if flip {
        effects::ChunkGlitch::Flip.run(chunk);
    }
}
