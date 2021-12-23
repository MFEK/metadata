use clap;
use norad::{DataRequest, Font, Glyph};
use unic_ucd::category::GeneralCategory;
use unic_ucd::name::Name;

use std::sync::Arc;

pub fn clap_subcommands() -> [clap::App<'static, 'static>; 2] {
    [
        clap::SubCommand::with_name("glyphs").about("Dumps the font's glyphs"),
        clap::SubCommand::with_name("glyph").about("Dumps a single font glyph in the format of `MFEKmetadata glyphs`"),
    ]
}

fn codepoints_to_string(cps: &Vec<char>) -> String {
    let mut ret = String::with_capacity((cps.len() * 4) + cps.len());
    for (i, cp) in cps.iter().enumerate() {
        ret.push_str(&format!("{:04x}", *cp as u32));
        if i != cps.len() - 1 {
            ret.push_str(",");
        }
    }
    ret
}

fn name_to_string(name: &Name) -> String {
    match name {
        Name::NR1(c) => format!("HANGUL SYLLABLE {}", c),
        Name::NR2(s, c) => format!("{} {:06x}", s, *c as u32),
        Name::NR3(sl) => format!("{}", sl.join(" ")),
    }
}

fn unnamed_name(cp: char) -> &'static str {
    match cp {
        '\x00'..='\x1f' => "<control>",
        '\u{E000}'..='\u{F8FF}' => "<PUA>",
        '\u{F0000}'..='\u{FFFFD}' => "<PUA-A>",
        '\u{100000}'..='\u{10FFFD}' => "<PUA-B>",
        _ => "<unencoded>",
    }
}

fn print_glyph(g: Arc<Glyph>) {
    print!("{}\t{}\t", &g.name, codepoints_to_string(&g.codepoints));
    if g.codepoints.len() > 0 {
        print!(
            "{}\t",
            (g.codepoints
                .iter()
                .map(|cp| Name::of(*cp).map(|n| name_to_string(&n)).unwrap_or(unnamed_name(*cp).to_string()))
                .collect::<Vec<String>>())
            .join(",")
        );
        print!(
            "{}\t",
            (g.codepoints.iter().map(|cp| format!("{:?}", GeneralCategory::of(*cp))))
                .collect::<Vec<String>>()
                .join(",")
        );
    }
    println!("")
}

pub fn glyphs(path: &std::ffi::OsStr, _args: &clap::ArgMatches) {
    let mut dr = DataRequest::none();
    dr.layers(true);
    let ufo = Font::load_requested_data(path, dr).expect("Failed to load UFO w/norad");
    for g in ufo.default_layer().iter() {
        print_glyph(g.to_owned());
    }
}

pub fn glyph(path: &std::ffi::OsStr, _args: &clap::ArgMatches) {
    let g = Glyph::load(path).unwrap();
    print_glyph(Arc::new(g));
}
