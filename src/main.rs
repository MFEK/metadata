//! MFEKmetadata - Basic metadata fetcher for the MFEK project.
//! (c) 2020–2021 Fredrick R. Brennan & MFEK Authors. Apache 2.0 licensed.
#![allow(non_snake_case)] // for our name MFEKmetadata

use clap;
use env_logger;
use glifparser::Glif;
use mfek_ipc;
use norad::{DataRequest, Font, Glyph};

mod glyphs;
use glyphs::{glyph, glyphs};
mod glyphslen;
use glyphslen::glyphslen;
mod glyphpathlen;
use glyphpathlen::glyphpathlen;
mod arbitrary;
use arbitrary::arbitrary;
mod write_metainfo;
use write_metainfo::write_metainfo;

#[macro_use]
pub mod util;

use std::path;
use std::sync::Arc;

fn parse_args() -> clap::ArgMatches<'static> {
    let mut app = clap::App::new(clap::crate_name!())
        .version(env!("CARGO_PKG_VERSION"))
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .setting(clap::AppSettings::SubcommandRequiredElseHelp)
        .arg(
            clap::Arg::with_name("PATH")
                .help("Sets the input file (glif/UFO/rarely plist) to use")
                .required(true)
                .index(1)
                .validator(|p| {
                    let p = path::Path::new(&p);
                    if p.is_file() || p.is_dir() { Ok(()) } else { Err(format!("File {} does not exist", p.display())) }
                })
        )
        .subcommand(glyphslen::clap_subcommand())
        .subcommand(glyphpathlen::clap_subcommand())
        .subcommand(arbitrary::clap_subcommand())
        .subcommand(write_metainfo::clap_subcommand());

    for sc in glyphs::clap_subcommands() {
        // `glyph`, `glyphs`
        app = app.subcommand(sc);
    }

    mfek_ipc::display_header("metadata");
    app.get_matches()
}

#[rustfmt::skip]
fn main() {
    env_logger::init();
    let matches = parse_args();
    let (program, args) = matches.subcommand();

    let path = matches.value_of_os("PATH").unwrap();

    let dr = match program {
        "arbitrary" | "glyph" | "glyphpathlen" | "write_metainfo" => DataRequest::none(),
        "glyphs" | "glyphslen" => *DataRequest::none().layers(true),
        _ => unimplemented!(),
    };

    let ufo = match program {
        "arbitrary" | "glyph" | "glyphpathlen" | "write_metainfo" => None,
        _ => Some(Font::load_requested_data(path, dr).unwrap()),
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
        "arbitrary" => arbitrary(path, args.unwrap()),
        "glyphs" => glyphs(&ufo.unwrap()),
        "glyphslen" => glyphslen(&ufo.unwrap()),
        "glyph" => glyph(Arc::new(glypho.unwrap())),
        "glyphpathlen" => glyphpathlen(glif.unwrap(), args.unwrap()),
        "write_metainfo" => write_metainfo(path).unwrap(),
        _ => {}
    }
}
