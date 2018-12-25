use std::ops::BitXor;

pub trait Glitch {
    fn run(&self, &mut [u8]) -> ();
}

pub enum ChunkGlitch {
    ChannelSwap(usize, usize, usize),
    Darken,
    Lighten,
    Flip,
    OffByOne(usize, usize, usize),
    Quantize,
    XOR(u8),
}

pub enum LineGlitch {
    ChannelShift(usize, usize, usize),
    Reverse,
    Shift(usize),
}

impl Glitch for ChunkGlitch {
    fn run(&self, chunk: &mut [u8]) {
        match self {
            ChunkGlitch::ChannelSwap(channel_1, channel_2, channel_count) => {
                let chunk_length = chunk.len();
                let channel_value_count = chunk_length / channel_count;

                for i in 0..channel_value_count {
                    let channel_1_index = (i * channel_count) + channel_1;
                    let channel_2_index = (i * channel_count) + channel_2;
                    let channel_1_value = chunk[channel_1_index];
                    let channel_2_value = chunk[channel_2_index];

                    chunk[channel_1_index] = channel_2_value;
                    chunk[channel_2_index] = channel_1_value;
                }
            }
            ChunkGlitch::Darken => {
                for val in chunk.iter_mut() {
                    *val /= 2;
                }
            }
            ChunkGlitch::Lighten => {
                for val in chunk.iter_mut() {
                    *val = *val / 2 + 128;
                }
            }
            ChunkGlitch::Flip => {
                chunk.reverse();
            }
            ChunkGlitch::OffByOne(line_count, line_length, channel_count) => {
                for i in 0..*line_count {
                    let line_start = i * line_length;
                    let line_end = (i + 1) * line_length - 1;
                    chunk[line_start..line_end].rotate_left((i * channel_count) % line_length);
                }
            }
            ChunkGlitch::Quantize => {
                for val in chunk.iter_mut() {
                    *val = (*val / 64) * 64;
                }
            }
            ChunkGlitch::XOR(byte) => {
                for val in chunk.iter_mut() {
                    *val = val.bitxor(byte);
                }
            }
        }
    }
}

impl Glitch for LineGlitch {
    fn run(&self, line: &mut [u8]) {
        match self {
            LineGlitch::ChannelShift(amount, channel, channel_count) => {
                let line_length = line.len();
                let channel_value_count = line_length / channel_count;

                for i in 0..channel_value_count {
                    line[(i * channel_count + channel) % line_length] =
                        line[(i * channel_count + channel + (channel + 1) * amount) % line_length];
                }
            }
            LineGlitch::Reverse => {
                line.reverse();
            }
            LineGlitch::Shift(amount) => {
                line.rotate_left(*amount);
            }
        }
    }
}
