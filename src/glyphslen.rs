use clap;
use norad::{DataRequest, Font};

pub fn clap_subcommand() -> clap::App<'static, 'static> {
    clap::SubCommand::with_name("glyphslen").about("Show number of glyphs in font")
}

pub fn glyphslen(path: &std::ffi::OsStr, _args: &clap::ArgMatches) {
    let dr = DataRequest::none().layers(true).data(true);
    let ufo = Font::load_requested_data(path, &dr).expect("Failed to load UFO w/norad");
    println!("{}", ufo.default_layer().len())
}
