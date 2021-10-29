//! MFEKmetadata - Basic metadata fetcher for the MFEK project.
//! Main author is Fredrick Brennan (@ctrlcctrlv); see AUTHORS.
//! (c) 2020. Apache 2.0 licensed.
#![allow(non_snake_case)] // for our name MFEKmetadata

use clap;
use norad::{DataRequest, Font, Glyph};
use glifparser::Glif;

mod glyphs;
use glyphs::{glyph, glyphs};
mod glyphslen;
use glyphslen::glyphslen;
mod glyphpathlen;
use glyphpathlen::glyphpathlen;
mod arbitrary;
use arbitrary::arbitrary;

mod util;

use std::sync::Arc;

fn parse_args() -> clap::ArgMatches<'static> {
    clap::App::new(clap::crate_name!())
        .version("0.0")
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .setting(clap::AppSettings::SubcommandRequiredElseHelp)
        .arg(
            clap::Arg::with_name("UFO_OR_GLIF")
                .help("Sets the input file (glif/UFO) to use")
                .required(true)
                .index(1),
        )
        .subcommand(clap::SubCommand::with_name("glyphslen").about("Show number of glyphs in font"))
        .subcommand(
            clap::SubCommand::with_name("glyphpathlen")
                .setting(clap::AppSettings::DeriveDisplayOrder)
                .about("Show length of contours in a glyph (.glif) on separate lines")
                .arg(
                    clap::Arg::with_name("segmentwise")
                        .short("s")
                        .long("segmentwise")
                        .help("Display length of each segment separated by spaces"),
                )
                .arg(
                    clap::Arg::with_name("joined")
                        .long("joined")
                        .short("j")
                        .help("Display one line: sum of joined path"),
                )
                .arg(
                    clap::Arg::with_name("json")
                        .long("json")
                        .short("J")
                        .help("Output JSON instead"),
                )
                .arg(
                    clap::Arg::with_name("accuracy")
                        .long("accuracy")
                        .help("Precision of length calculation")
                        .takes_value(true)
                        .default_value("0.01")
                        .empty_values(false)
                        .number_of_values(1)
                        .validator(util::arg_validator_positive_f64)
                )
        )
        .subcommand(clap::SubCommand::with_name("glyphs").about("Dumps the font's glyphs"))
        .subcommand(clap::SubCommand::with_name("glyph").about("Dumps a single font glyph in the format of `MFEKmetadata glyphs`"))
        .subcommand(clap::SubCommand::with_name("metrics").about("Dumps the font's metrics"))
        .subcommand(
            clap::SubCommand::with_name("arbitrary")
                .about("Dumps key values")
                .arg(
                    clap::Arg::with_name("keys")
                        .required(true)
                        .multiple(true)
                        .takes_value(true)
                        .short("k")
                        .help("List of key values to display, one per line, in order requested"),
                )
                .arg(
                    clap::Arg::with_name("file")
                        .default_value("fontinfo.plist")
                        .multiple(false)
                        .short("f")
                        .help("File to search through for XPath's"),
                )
                .arg(
                    clap::Arg::with_name("with-keys")
                        .long("with-keys")
                        .takes_value(false)
                        .required(false)
                        .help("Whether to show keys in a tab-separated format"),
                ),
        )
        .get_matches()
}

#[rustfmt::skip]
fn main() {
    let matches = parse_args();
    let (program, args) = matches.subcommand();

    let path = matches.value_of("UFO_OR_GLIF").unwrap();

    let dr = match program {
        "arbitrary" => DataRequest::none(),
        "glyphs" | "glyphslen" => *DataRequest::none().layers(true),
        "glyph" | "glyphpathlen" => DataRequest::none(),
        _ => unimplemented!(),
    };

    let ufo = match program {
        "glyph" | "glyphpathlen" => None,
        _ => Some(Font::with_fields(dr).load_ufo(path).unwrap()),
    };

    let glypho = match program {
        "glyph" => Some(Glyph::load(path).unwrap()),
        _ => None,
    };

    let glif: Option<Glif<()>> = match program {
        "glyphpathlen" => Some(glifparser::read_from_filename(path).expect("Failed to parse .glif file")),
        _ => None,
    };

    match program {
        "arbitrary" => arbitrary(&ufo.unwrap(), args.unwrap().values_of("keys").unwrap().collect()),
        "glyphs" => glyphs(&ufo.unwrap()),
        "glyphslen" => glyphslen(&ufo.unwrap()),
        "glyph" => glyph(Arc::new(glypho.unwrap())),
        "glyphpathlen" => glyphpathlen(glif.unwrap(), args.unwrap()),
        _ => {}
    }
}
