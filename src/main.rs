//! MFEKmetadata - Basic metadata fetcher for the MFEK project.
//! (c) 2020–2021 Fredrick R. Brennan & MFEK Authors. Apache 2.0 licensed.
#![allow(non_snake_case)] // for our name MFEKmetadata

use clap;
use glifparser::Glif;
use norad::{DataRequest, Font, Glyph};

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
    let mut app = clap::App::new(clap::crate_name!())
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
        .subcommand(glyphslen::clap_subcommand())
        .subcommand(glyphpathlen::clap_subcommand())
        .subcommand(arbitrary::clap_subcommand());

    for sc in glyphs::clap_subcommands() {
        // `glyph`, `glyphs`
        app = app.subcommand(sc);
    }

    app.get_matches()
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
