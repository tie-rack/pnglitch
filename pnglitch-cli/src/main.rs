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
    let w = &mut BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, info.width, info.height);
    encoder.set(info.color_type).set(info.bit_depth);
    let mut writer = encoder.write_header().unwrap();

    let mut rng = thread_rng();

    let options = GlitchOptions {
        min_glitches,
        max_glitches,
    };

    glitch(&info, &mut buf, &mut rng, &options);

    writer.write_image_data(&buf).unwrap();
}
