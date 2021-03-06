//! MFEKmetadata - Basic metadata fetcher for the MFEK project.
//! Main author is Fredrick Brennan (@ctrlcctrlv); see AUTHORS.
//! (c) 2020. Apache 2.0 licensed.
#![allow(non_snake_case)] // for our name MFEKmetadata

extern crate clap;
extern crate justify;
extern crate norad;
extern crate serde_value;
extern crate enum_for_matches;

use norad::{Ufo, DataRequest};

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

use serde_value::Value as SerdeValue;
fn arbitrary(ufo: &Ufo, keys: Vec<&str>) {
    let md = &ufo.meta;
    assert_eq!(
        md.format_version,
        norad::FormatVersion::V3,
        "UFO versions other than 3 unsupported"
    );
    let fi = ufo
        .font_info
        .as_ref()
        .expect("Norad failed to parse font metainfo");
    let map = serde_value::to_value(fi).expect("Failed to serialize fontinfo - not a serde Value?");

    for key in keys {
        match map {
            SerdeValue::Map(ref m) => {
                let arg = &SerdeValue::String(key.to_string());
                match m.get(arg) {
                    Some(SerdeValue::Option(ref o)) => enum_for_matches::run!(
                        **(o.as_ref().unwrap()),
                        { 
                              SerdeValue::U8(ref oo)
                            | SerdeValue::U16(ref oo)
                            | SerdeValue::U32(ref oo)
                            | SerdeValue::U64(ref oo)
                            | SerdeValue::I8(ref oo)
                            | SerdeValue::I16(ref oo)
                            | SerdeValue::I32(ref oo)
                            | SerdeValue::I64(ref oo)
                            | SerdeValue::F32(ref oo)
                            | SerdeValue::F64(ref oo)
                            | SerdeValue::Char(ref oo)
                            | SerdeValue::String(ref oo)
                        }, 
                        {println!("{}", &oo);}, 
                        {panic!("Unimplemented request for array, option or dict");}
                    ),
                    _ => println!(""),
                }
            }
            _ => panic!("FontInfo not a BTreeMap"),
        }
    }
}

fn main() {
    let matches = parse_args();
    let (program, args) = matches.subcommand();

    let ufo = Ufo::with_fields(DataRequest::none())
        .load_ufo(matches.value_of("UFO").unwrap())
        .unwrap();

    match program {
        "arbitrary" => arbitrary(&ufo, args.unwrap().values_of("keys").unwrap().collect()),
        _ => {}
    }
}
