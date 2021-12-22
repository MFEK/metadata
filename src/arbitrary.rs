use clap;
use itertools::Itertools;
use plist;

use std::ffi;
use std::fs;
use std::path as fspath;

use crate::write_metainfo::write_metainfo_impl as write_metainfo;

pub fn clap_subcommand() -> clap::App<'static, 'static> {
    clap::SubCommand::with_name("arbitrary")
        .about("Dumps key values")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            clap::Arg::with_name("keys")
                .required(true)
                .multiple(true)
                .takes_value(true)
                .allow_hyphen_values(true)
                .number_of_values(1)
                .short("k")
                .long("key")
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
                .help("List of values to write, in order requested"),
        )
        .arg(
            clap::Arg::with_name("xml-redirect")
                .takes_value(true)
                .short("X")
                .long("xml-redirect")
                .help("Redirect XML to this path instead. Use /dev/stdout or /dev/stderr if that's what you want, `-` not recognized.")
        )
}

pub fn arbitrary(path: &ffi::OsStr, args: &clap::ArgMatches) {
    drop(write_metainfo(path));
    let mut path = fspath::PathBuf::from(path);
    let keys: Vec<String> = args
        .values_of("keys")
        .unwrap()
        .map(|s| s.to_owned())
        .collect();
    let values: Vec<String> = args
        .values_of("values")
        .map(|v| v.map(|s| s.to_owned()).collect())
        .unwrap_or(vec![]);
    let xml_redirect: Option<_> = args.value_of("xml-redirect");
    let values_len = values.len();
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
            (_, None) | (None, _) => continue,
        };

        let argval = map.get(&key).to_owned();

        let value: String = match value {
            None => {
                println!("{}", serde_json::to_string(&argval).unwrap());
                continue;
            }
            Some(value) => value.to_string(),
        };

        match argval {
            Some(_) => {
                map.insert(
                    key,
                    plist::from_bytes::<plist::Value>(value.as_bytes()).unwrap(),
                );
            }
            None => {}
        }
    }

    if values_len != 0 {
        if let Some(f) = xml_redirect {
            path = fspath::PathBuf::from(f);
        }
        let file = fs::File::create(&path).unwrap();
        plist::to_writer_xml(file, &map).unwrap();
        //plist::to_writer_xml(io::stdout(), &map).unwrap();
    }
}
