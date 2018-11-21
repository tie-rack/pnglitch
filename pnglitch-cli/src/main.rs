use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

#[macro_use]
extern crate clap;
use clap::{App, Arg};

extern crate png;
use png::HasParameters;

extern crate rand;
use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};

extern crate pnglitch_core;
use pnglitch_core::effects;
use pnglitch_core::effects::Glitch;

fn glitch_chunk(
    buf: &mut [u8],
    line_size: usize,
    color_type: png::ColorType,
    rng: &mut ThreadRng,
) -> () {
    let line_count = buf.len() / line_size;

    let line_number_dist = Uniform::from(0..line_count);
    let line_shift_dist = Uniform::from(0..line_size);
    let xor_dist = Uniform::from(0..255);

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
        let channel_count = match color_type {
            png::ColorType::Grayscale => 1,
            png::ColorType::RGB => 3,
            png::ColorType::Indexed => 1,
            png::ColorType::GrayscaleAlpha => 2,
            png::ColorType::RGBA => 4,
        };
        let amount = line_shift_dist.sample(rng) / channel_count;
        let channel = Uniform::from(0..channel_count).sample(rng);
        Some(effects::LineGlitch::ChannelShift(
            amount,
            channel,
            channel_count,
        ))
    } else {
        None
    };

    let line_shift = effects::LineGlitch::Shift(line_shift_amount);

    // line-level effects
    for line_number in first_line..last_line {
        let line_start = line_number * line_size;
        let line_end = line_start + line_size;

        let line = buf.get_mut(line_start..line_end).unwrap();

        if let Some(shift_channel) = &shift_channel {
            shift_channel.run(line);
        }

        line_shift.run(line);

        if reverse {
            effects::LineGlitch::Reverse.run(line);
        }
    }

    // chunk-level effects
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

fn main() {
    let matches = App::new("glifch")
        .version("1.0")
        .author("Christopher Shea <cmshea@gmail.com>")
        .about("PNG -> Glitched GIF")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        ).arg(
            Arg::with_name("OUTPUT")
                .help("Sets the output file")
                .required(true)
                .index(2),
        ).arg(
            Arg::with_name("MIN_GLITCHES")
                .short("m")
                .long("min")
                .value_name("MIN")
                .help("Minimum number of glitched chunks (default: 1)")
                .takes_value(true),
        ).arg(
            Arg::with_name("MAX_GLITCHES")
                .short("M")
                .long("max")
                .value_name("MAX")
                .help("Maximum number of glitched chunks (default: 4)")
                .takes_value(true),
        ).get_matches();

    let input = matches.value_of("INPUT").unwrap();
    let output = matches.value_of("OUTPUT").unwrap();

    let min_glitches = value_t!(matches, "MIN_GLITCHES", u32).unwrap_or(1);
    let max_glitches = value_t!(matches, "MAX_GLITCHES", u32).unwrap_or(4) + 1;

    let decoder = png::Decoder::new(File::open(input).unwrap());

    let (info, mut reader) = decoder.read_info().expect("Input file not a png!");

    let mut buf = vec![0; info.buffer_size()];
    reader.next_frame(&mut buf).unwrap();

    let path = Path::new(output);
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, info.width, info.height);
    encoder.set(info.color_type).set(info.bit_depth);
    let mut writer = encoder.write_header().unwrap();

    let mut rng = thread_rng();
    let chunk_count_dist = Uniform::from(min_glitches..max_glitches);

    let glitch_count = chunk_count_dist.sample(&mut rng);

    for _ in 0..glitch_count {
        glitch_chunk(&mut buf, info.line_size, info.color_type, &mut rng);
    }

    writer.write_image_data(&buf).unwrap();
}
