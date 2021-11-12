use clap;
use norad::Font;

pub fn clap_subcommand() -> clap::App<'static, 'static> {
    clap::SubCommand::with_name("glyphslen").about("Show number of glyphs in font")
}

pub fn glyphslen(ufo: &Font) {
    println!("{}", ufo.default_layer().len())
}
