use clap;
use csv::{self, Writer as CsvWriter};
use enum_for_matches;
use norad::Font;
use serde_value::Value as SerdeValue;

use std::io;

pub fn clap_subcommand() -> clap::App<'static, 'static> {
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
        )
}

pub fn arbitrary(ufo: &Font, keys: Vec<&str>) {
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
                        {
                            let mut wtr = CsvWriter::from_writer(io::stdout());
                            wtr.serialize(oo).unwrap_or(());
                        },
                        {panic!("Unimplemented request for array, option or dict");}
                    ),
                    _ => println!(""),
                }
            }
            _ => panic!("FontInfo not a BTreeMap"),
        }
    }
}
