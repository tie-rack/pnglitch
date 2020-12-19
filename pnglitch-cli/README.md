# PNGlitch CLI

A command line interface for PNGlitch.

## Installing

To build and install, run: `cargo install --path .`

## Help

Run `pnglitch-cli --help`

## Examples

Using all defaults:

    pnglitch-cli input.png output.png

Setting some options:

    pnglitch-cli \
        --darken 0.0 \
        --channel-swap 0.8 \
        --min 3 \
        input.png output.png
