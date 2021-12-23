use clap;
use itertools::Itertools;
use plist;

use std::ffi;
use std::fs;
use std::io::Write as _;
use std::path as fspath;
use std::time::Instant;

use crate::util;
use crate::write_metainfo::write_metainfo_impl;

pub fn clap_subcommand() -> clap::App<'static, 'static> {
    clap::SubCommand::with_name("arbitrary")
        .setting(clap::AppSettings::DeriveDisplayOrder)
        .about("Performs arbitrary operations on a plist file, by default a font's fontinfo.plist.\n\nNote: The arguments `-k`, `-v`, and `-d` must be provided multiple times for multiple values, not delimited.")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            clap::Arg::with_name("keys")
                .required_unless("delete-keys")
                .multiple(true)
                .takes_value(true)
                .allow_hyphen_values(true)
                .number_of_values(1)
                .short("k")
                .long("key")
                .value_name("key")
                .help("List of key values to display, one per line, in order requested"),
        )
        .arg(
            clap::Arg::with_name("values")
                .multiple(true)
                .takes_value(true)
                .allow_hyphen_values(true)
                .number_of_values(1)
                .short("v")
                .long("value")
                .value_name("value")
                .help("List of values to write, in order requested"),
        )
        .arg(
            clap::Arg::with_name("delete-keys")
                .multiple(true)
                .takes_value(true)
                .allow_hyphen_values(true)
                .number_of_values(1)
                .short("d")
                .long("delete")
                .value_name("key")
                .help("List of keys to delete from the plist"),
        )
        .arg(
            clap::Arg::with_name("xml-redirect")
                .takes_value(true)
                .short("X")
                .long("xml-redirect")
                .value_name("FILE")
                .help("Redirect XML to this path instead. Use /dev/stdout or /dev/stderr if that's what you want, `-` not recognized.")
        )
}

pub fn arbitrary(path: &ffi::OsStr, args: &clap::ArgMatches) {
    let now = Instant::now();
    drop(write_metainfo_impl(path));
    let mut path = fspath::PathBuf::from(path);
    let keys: Vec<String> = args.values_of("keys").map(|k| k.map(|s| s.to_owned()).collect()).unwrap_or(vec![]);
    let values: Vec<String> = args.values_of("values").map(|v| v.map(|s| s.to_owned()).collect()).unwrap_or(vec![]);
    let to_delete: Vec<&str> = args.values_of("delete-keys").map(|dk| dk.collect()).unwrap_or(vec![]);
    let xml_redirect: Option<_> = args.value_of("xml-redirect");
    let values_len = values.len();
    let delete_len = to_delete.len();
    let is_plist = path.extension() != Some(&ffi::OsString::from("plist"));
    if is_plist {
        path.push("fontinfo.plist");
    }
    let mut plistv = plist::Value::from_file(&path).expect("fontinfo not plist");
    let map: &mut plist::Dictionary = plistv.as_dictionary_mut().unwrap();

    for keyvalue in keys.into_iter().zip_longest(values) {
        let (key, value) = match (keyvalue.clone().left(), keyvalue.right()) {
            (Some(key), Some(value)) => (key, Some(value)),
            (Some(key), None) => (key, None),
            (None, None) | (None, Some(_)) => continue,
        };

        let argval = map.get(&key).to_owned();

        let value: String = match value {
            None => {
                if let None = argval {
                    log::warn!("No value for {}", &key);
                }
                println!("{}", serde_json::to_string(&argval).unwrap());
                continue;
            }
            Some(value) => value.to_string(),
        };

        map.insert(key, plist::from_bytes::<plist::Value>(value.as_bytes()).unwrap());
    }

    for dk in to_delete {
        match map.remove(dk) {
            Some(_) => {
                log::debug!("Removed {}", dk);
            }
            None => {
                log::warn!("Tried to remove non-existent key {}", dk);
            }
        }
    }

    if values_len != 0 || delete_len != 0 {
        if let Some(f) = xml_redirect {
            path = fspath::PathBuf::from(f);
        }
        let mut file = match fs::File::create(&path) {
            Ok(f) => f,
            Err(e) => util::exit!("Failed to create file {:?}! I/O error: {:?}", &path, e),
        };
        if let Err(e) = plist::to_writer_xml(&file, &map) {
            util::exit!("Failed to write XML to {:?}! plist.rlib error: {:?}", &path, e);
        }
        if let Err(e) = file.write(b"\n") {
            util::exit!("Failed to write final newline to plist {:?}! I/O error: {:?}", &path, e);
        }
    }

    log::trace!("Completed in {}µs", now.elapsed().as_micros());
}
