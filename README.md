# PNGlitch

## About

PNGlitch is a library, command line tool, and website for applying
random effects to PNG images.

## The pieces

### `pnglitch-core`

This crate provides the function `pnglitch_core::glitch`, which
`pnglitch-cli` and `pnglitch-wasm` use to mess with PNG image
data.

### `pnglitch-cli`

This crate makes a command line tool for messing with PNGs.

### `pnglitch-wasm`

This one creates a Web Assembly interface for doing these
transformations right in the browser.

### `pnglitch-web`

Finally, a project that uses `pnglitch-wasm` and JavaScript to make a
webpage to interactively glitch out your PNGs.

## Effects

There are a number of transformations pnglitch can apply to image
data.

**Channel swap** switches the bytes of one channel for another. For
example, swapping the red and green channels.

**Darken** halves all the byte values in the chunk.

**Flip** reverses the bytes in the chunk. This also effectively swaps
channels around, so isn't quite as clean as making that part of the
image appear upside down.

**Lighten** brightens the image somewhat naively by mapping each byte
from 0-255 into 128-255.

**Line shift** rotates each line in a chunk. It honors the number of
channels in the image, so channels won't be effectively swapped
around.

**Off by one** rotates each line in a chunk, but each line by one more
pixel than the last.

**Quantize** applies a step function to byte values, limiting them to
192, 128, 64, or 0.

**Reverse** reverses each line. Like flip, effectively swaps channels,
so it similarly isn't quite as nice as just mirroring that part of the
image.

**Shift Channel** works like line shift, but only applies to a single
channel.

**XOR** generates a random byte and xors each byte in the chunk with
it.

## Further documentation

See each project's README for more.

## License

The code in pnglitch-cli, pnglitch-core, pnglitch-wasm, and
pnglitch-web are licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or
  http://opensource.org/licenses/MIT)

at your option.
