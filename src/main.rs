//! MFEKmetadata - Basic metadata fetcher for the MFEK project.
//! (c) 2020–2021 Fredrick R. Brennan & MFEK Authors. Apache 2.0 licensed.
#![allow(non_snake_case)] // for our name MFEKmetadata

use clap;
use mfek_ipc;

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
                    if p.is_file() || p.is_dir() {
                        Ok(())
                    } else {
                        Err(format!("File {} does not exist", p.display()))
                    }
                }),
        )
        .subcommand(glyphslen::clap_subcommand())
        .subcommand(glyphpathlen::clap_subcommand())
        .subcommand(arbitrary::clap_subcommand())
        .subcommand(write_metainfo::clap_subcommand());

    for sc in glyphs::clap_subcommands() {
        // `glyph`, `glyphs`
        app = app.subcommand(sc);
    }

    app.get_matches()
}

#[rustfmt::skip]
fn main() {
    util::init_env_logger();
    mfek_ipc::display_header("metadata");
    let matches = parse_args();
    let (program, args) = matches.subcommand();

    let path = matches.value_of_os("PATH").unwrap();

    let args = args.expect("Failed to parse args?");

    match program {
        "arbitrary" => arbitrary(path, &args),
        "glyphs" => glyphs(path, &args),
        "glyphslen" => glyphslen(path, &args),
        "glyph" => glyph(path, &args),
        "glyphpathlen" => glyphpathlen(path, &args),
        "write_metainfo" => write_metainfo(path, &args),
        _ => {}
    }
}
