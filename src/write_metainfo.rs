use clap;
use plist;

use std::ffi::OsStr;
use std::fs;
use std::path;

use crate::exit;

pub fn clap_subcommand() -> clap::App<'static, 'static> {
    clap::SubCommand::with_name("write_metainfo")
}

pub fn write_metainfo(ufo: &OsStr) -> Result<(), String> {
    let ufo = path::Path::new(ufo);
    if !ufo.is_dir() {
        exit!("{:?} not a directory", ufo);
    }
    let ufo = ufo.to_path_buf();
    let mut metainfo_f = ufo.clone();
    metainfo_f.push("metainfo.plist");
    let metainfo = match plist::Value::from_file(&metainfo_f) {
        Ok(mf) => mf,
        Err(e) => {
            return Err(format!("{:?}", e));
        }
    };
    let mut metainfo = match metainfo.into_dictionary() {
        Some(dict) => dict,
        _ => {
            return Err(format!("Metainfo not dict"));
        }
    };
    log::trace!("metainfo: {:?}", &metainfo);
    metainfo.insert(
        "creator".to_string(),
        plist::Value::String("org.MFEK".to_string()),
    );
    let fsfile = match fs::File::create(&metainfo_f) {
        Ok(f) => f,
        Err(e) => Err(format!("Failed to create file {:?}: {:?}", &metainfo_f, e))?,
    };
    match plist::to_writer_xml_with_options(
        fsfile,
        &metainfo,
        &plist::XmlWriteOptions::default().indent_string("    "),
    ) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to serialize XML due to: {:?}", e)),
    }
}
