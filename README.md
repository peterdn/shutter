# shutter [![Build Status](https://travis-ci.org/peterdn/shutter.svg?branch=master)](https://travis-ci.org/peterdn/shutter)

An Instagram scraping library and CLI app written in Rust.

## Usage

To run the CLI app:

    cargo run --bin instagram-scraper -- [FLAGS] [OPTIONS] <USERNAME>

The app currently supports the following:

 - Print user's profile information with `-p`.
 - Download latest images with `-i`. Use `-o <DIRECTORY>` to set output directory.

## License

Distributed under the BSD-2-Clause license. See LICENSE for details.
