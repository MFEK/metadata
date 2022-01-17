use clap;

use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::{self, Read as _, Write as _};
use std::mem;
use std::path;
use std::time::Instant;

use crate::util::exit;

static METAINFO: &[u8] = br#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
    <dict>
        <key>creator</key>
        <string>org.MFEK</string>
        <key>formatVersion</key>
        <integer>3</integer>
    </dict>
</plist>"#;

pub fn clap_subcommand() -> clap::App<'static, 'static> {
    clap::SubCommand::with_name("write_metainfo")
}

pub(crate) fn write_metainfo_impl(ufo: &OsStr) -> Result<path::PathBuf, io::Error> {
    let ufo = path::Path::new(ufo);
    if !ufo.is_dir() {
        exit!("{:?} not a directory", ufo);
    }
    let ufo = ufo.to_path_buf();
    let mut metainfo_f = ufo.clone();
    metainfo_f.push("metainfo.plist");
    fslock::lockfile_truncate(false);
    let filelock = match fslock::LockFile::open(&metainfo_f).map(|mut l| l.lock().map(|_| l)) {
        Ok(Ok(f)) => f,
        Ok(Err(e)) => Err(e)?,
        Err(e) => Err(e)?,
    };
    let mut fsfile = unsafe {
        use std::os::unix::io::FromRawFd;
        fs::File::from_raw_fd(filelock.raw())
    };
    match fsfile.write(METAINFO) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }?;
    mem::forget(fsfile);
    Ok(metainfo_f)
}

pub fn write_metainfo(path: &OsStr, _args: &clap::ArgMatches) {
    let now = Instant::now();
    let pb = write_metainfo_impl(path).unwrap_or_else(|e| panic!("Failed to write metainfo.plist! {:?}", e));
    let elapsed = now.elapsed().as_micros();
    log::info!(
        "writing {}/metainfo.plist took {}µs",
        path.to_owned().into_string().unwrap_or_else(|o| format!("<??PATH{:?}>", o)),
        elapsed
    );
    let now = Instant::now();
    let mut s = Vec::with_capacity(METAINFO.len());
    match fslock::LockFile::open(&pb).map(|mut fl| fl.lock().map(|_| Into::<File>::into(&mut fl).read_to_end(&mut s).map(|_| fl.unlock()))) {
        Ok(Ok(Ok(Ok(())))) => {
            debug_assert_eq!(s, METAINFO);
            log::info!("confirming metainfo.plist contents took {}µs", now.elapsed().as_micros())
        }
        Err(e) | Ok(Err(e)) | Ok(Ok(Err(e))) | Ok(Ok(Ok(Err(e)))) => exit!("Failed readback! {:?}", e),
    }
}
