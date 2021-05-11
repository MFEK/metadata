//! MFEKmetadata - Basic metadata fetcher for the MFEK project.
//! Main author is Fredrick Brennan (@ctrlcctrlv); see AUTHORS.
//! (c) 2020. Apache 2.0 licensed.
#![allow(non_snake_case)] // for our name MFEKmetadata

use clap;
use norad::{Font, DataRequest};

mod glyphs;
use glyphs::glyphs;
mod glyphslen;
use glyphslen::glyphslen;
mod arbitrary;
use arbitrary::arbitrary;

fn parse_args() -> clap::ArgMatches<'static> {
    clap::App::new(clap::crate_name!())
        .version("0.0")
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .setting(clap::AppSettings::SubcommandRequiredElseHelp)
        .arg(
            clap::Arg::with_name("UFO")
                .help("Sets the input UFO font to use")
                .required(true)
                .index(1),
        )
        .subcommand(clap::SubCommand::with_name("glyphslen").about("Show number of glyphs in font"))
        .subcommand(clap::SubCommand::with_name("glyphs").about("Dumps the font's glyphs"))
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

fn main() {
    let matches = parse_args();
    let (program, args) = matches.subcommand();

    let ufopath = matches.value_of("UFO").unwrap();

    let dr = match program {
        "arbitrary" => DataRequest::none(),
        "glyphs" | "glyphslen" => *DataRequest::none().layers(true),
        _ => unimplemented!()
    };

    let ufo = Font::with_fields(dr)
        .load_ufo(ufopath)
        .unwrap();

    match program {
        "arbitrary" => arbitrary(&ufo, args.unwrap().values_of("keys").unwrap().collect()),
        "glyphs" => glyphs(&ufo),
        "glyphslen" => glyphslen(&ufo),
        _ => {}
    }
}
