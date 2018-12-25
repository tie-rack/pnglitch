use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

#[macro_use]
extern crate clap;
use clap::{App, Arg};

extern crate png;
use png::HasParameters;

extern crate rand;
use rand::thread_rng;

extern crate pnglitch_core;
use pnglitch_core::{glitch, GlitchOptions};

fn main() {
    let glitch_options_default = GlitchOptions::default();

    let matches = App::new("glifch")
        .version("1.0")
        .author("Christopher Shea <cmshea@gmail.com>")
        .about("PNG -> Glitched GIF")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .help("Sets the output file")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("MIN_GLITCHES")
                .short("m")
                .long("min")
                .value_name("MIN")
                .help(&format!(
                    "Minimum number of glitched chunks (default: {})",
                    glitch_options_default.min_glitches
                ))
                .takes_value(true),
        )
        .arg(
            Arg::with_name("MAX_GLITCHES")
                .short("M")
                .long("max")
                .value_name("MAX")
                .help(&format!(
                    "Maximum number of glitched chunks (default: {})",
                    glitch_options_default.max_glitches
                ))
                .takes_value(true),
        )
        .arg(
            Arg::with_name("CHANNEL_SWAP_CHANCE")
                .long("channel-swap")
                .value_name("CHANCE")
                .help(&format!(
                    "Chance of swapping channels (default {})",
                    glitch_options_default.channel_swap_chance
                ))
                .takes_value(true),
        )
        .arg(
            Arg::with_name("DARKEN_CHANCE")
                .long("darken")
                .value_name("CHANCE")
                .help(&format!(
                    "Chance of darkening chunk (default {})",
                    glitch_options_default.darken_chance
                ))
                .takes_value(true),
        )
        .arg(
            Arg::with_name("FLIP_CHANCE")
                .long("flip")
                .value_name("CHANCE")
                .help(&format!(
                    "Chance of flipping a chunk (default {})",
                    glitch_options_default.flip_chance
                ))
                .takes_value(true),
        )
        .arg(
            Arg::with_name("LIGHTEN_CHANCE")
                .long("lighten")
                .value_name("CHANCE")
                .help(&format!(
                    "Chance of lightening a chunk (default {})",
                    glitch_options_default.lighten_chance
                ))
                .takes_value(true),
        )
        .arg(
            Arg::with_name("LINE_SHIFT_CHANCE")
                .long("line-shift")
                .value_name("CHANCE")
                .help(&format!(
                    "Chance of shifting lines (default {})",
                    glitch_options_default.line_shift_chance
                ))
                .takes_value(true),
        )
        .arg(
            Arg::with_name("OFF_BY_ONE_CHANCE")
                .long("off-by-one")
                .value_name("CHANCE")
                .help(&format!(
                    "Chance of shifting lines bit by bit (default {})",
                    glitch_options_default.off_by_one_chance
                ))
                .takes_value(true),
        )
        .arg(
            Arg::with_name("QUANTIZE_CHANCE")
                .long("quantize")
                .value_name("CHANCE")
                .help(&format!(
                    "Chance of quantizing a chunk (default {})",
                    glitch_options_default.quantize_chance
                ))
                .takes_value(true),
        )
        .arg(
            Arg::with_name("REVERSE_CHANCE")
                .long("reverse")
                .value_name("CHANCE")
                .help(&format!(
                    "Chance of reversing a chunk (default {})",
                    glitch_options_default.reverse_chance
                ))
                .takes_value(true),
        )
        .arg(
            Arg::with_name("SHIFT_CHANNEL_CHANCE")
                .long("shift-channel")
                .value_name("CHANCE")
                .help(&format!(
                    "Chance of shifting a channel in a chunk (default {})",
                    glitch_options_default.shift_channel_chance
                ))
                .takes_value(true),
        )
        .arg(
            Arg::with_name("XOR_CHANCE")
                .long("xor")
                .value_name("CHANCE")
                .help(&format!(
                    "Chance of xoring a chunk (default {})",
                    glitch_options_default.xor_chance
                ))
                .takes_value(true),
        )
        .get_matches();

    let input = matches.value_of("INPUT").unwrap();
    let output = matches.value_of("OUTPUT").unwrap();

    let min_glitches =
        value_t!(matches, "MIN_GLITCHES", u32).unwrap_or(glitch_options_default.min_glitches);
    let max_glitches =
        value_t!(matches, "MAX_GLITCHES", u32).unwrap_or(glitch_options_default.max_glitches);

    let channel_swap_chance = value_t!(matches, "CHANNEL_SWAP_CHANCE", f64)
        .unwrap_or(glitch_options_default.channel_swap_chance);
    let darken_chance =
        value_t!(matches, "DARKEN_CHANCE", f64).unwrap_or(glitch_options_default.darken_chance);
    let flip_chance =
        value_t!(matches, "FLIP_CHANCE", f64).unwrap_or(glitch_options_default.flip_chance);
    let lighten_chance =
        value_t!(matches, "LIGHTEN_CHANCE", f64).unwrap_or(glitch_options_default.lighten_chance);
    let line_shift_chance = value_t!(matches, "LINE_SHIFT_CHANCE", f64)
        .unwrap_or(glitch_options_default.line_shift_chance);
    let off_by_one_chance = value_t!(matches, "OFF_BY_ONE_CHANCE", f64)
        .unwrap_or(glitch_options_default.off_by_one_chance);
    let quantize_chance =
        value_t!(matches, "QUANTIZE_CHANCE", f64).unwrap_or(glitch_options_default.quantize_chance);
    let reverse_chance =
        value_t!(matches, "REVERSE_CHANCE", f64).unwrap_or(glitch_options_default.reverse_chance);
    let shift_channel_chance = value_t!(matches, "SHIFT_CHANNEL_CHANCE", f64)
        .unwrap_or(glitch_options_default.shift_channel_chance);
    let xor_chance =
        value_t!(matches, "XOR_CHANCE", f64).unwrap_or(glitch_options_default.xor_chance);

    let decoder = png::Decoder::new(File::open(input).unwrap());

    let (info, mut reader) = decoder.read_info().expect("Input file not a png!");

    let mut buf = vec![0; info.buffer_size()];
    reader.next_frame(&mut buf).unwrap();

    let path = Path::new(output);
    let file = File::create(path).unwrap();
    let w = &mut BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, info.width, info.height);
    encoder.set(info.color_type).set(info.bit_depth);
    let mut writer = encoder.write_header().unwrap();

    let mut rng = thread_rng();

    let options = GlitchOptions {
        min_glitches,
        max_glitches,
        channel_swap_chance,
        darken_chance,
        flip_chance,
        lighten_chance,
        line_shift_chance,
        off_by_one_chance,
        quantize_chance,
        reverse_chance,
        shift_channel_chance,
        xor_chance,
    };

    glitch(&info, &mut buf, &mut rng, &options);

    writer.write_image_data(&buf).unwrap();
}
