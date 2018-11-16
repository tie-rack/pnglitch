use std::ops::BitXor;

pub trait Glitch {
    fn run(&self, &mut [u8]) -> ();
}

pub enum ChunkGlitch {
    Darken,
    Lighten,
    Flip,
    Quantize,
    XOR(u8),
}

pub enum LineGlitch {
    ChannelShift(usize, usize, usize),
    Reverse,
    Shift(usize),
}

impl Glitch for ChunkGlitch {
    fn run(&self, chunk: &mut [u8]) -> () {
        match self {
            ChunkGlitch::Darken => {
                for val in chunk.iter_mut() {
                    *val = *val / 2;
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
    fn run(&self, line: &mut [u8]) -> () {
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
