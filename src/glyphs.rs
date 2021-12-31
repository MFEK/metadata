use clap;
use norad::{DataRequest, Font, Glyph};
use unic_ucd::category::GeneralCategory;
use unic_ucd::name::Name;

use std::cmp::Ordering;
use std::sync::Arc;

pub fn clap_subcommands() -> [clap::App<'static, 'static>; 2] {
    [
        clap::SubCommand::with_name("glyphs")
            .about("Dumps the font's glyphs")
            .setting(clap::AppSettings::DeriveDisplayOrder)
            .arg(
                clap::Arg::with_name("sort")
                    .takes_value(false)
                    .short("s")
                    .long("sort")
                    .help("Sort by Unicode char"),
            )
            .arg(
                clap::Arg::with_name("hide-unencoded")
                    .takes_value(false)
                    .short("H")
                    .long("hide-unencoded")
                    .help("Don't show unencoded glyphs in listing"),
            )
            .arg(
                clap::Arg::with_name("unencoded-at-top")
                    .takes_value(false)
                    .short("u")
                    .long("unencoded-at-top")
                    .help("Glyphs without encodings go to the top"),
            ),
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

fn glyph_row(g: &Glyph, codepoint_first: bool) -> String {
    let mut ret = if codepoint_first {
        format!("{}\t{}\t", codepoints_to_string(&g.codepoints), &g.name)
    } else {
        format!("{}\t{}\t", &g.name, codepoints_to_string(&g.codepoints))
    };
    if g.codepoints.len() > 0 {
        ret.push_str(&format!(
            "{}\t",
            (g.codepoints
                .iter()
                .map(|cp| Name::of(*cp).map(|n| name_to_string(&n)).unwrap_or(unnamed_name(*cp).to_string()))
                .collect::<Vec<String>>())
            .join(",")
        ));
        ret.push_str(&format!(
            "{}\t",
            (g.codepoints.iter().map(|cp| format!("{:?}", GeneralCategory::of(*cp))))
                .collect::<Vec<String>>()
                .join(",")
        ));
    }
    ret.push_str("\n");
    ret
}

fn sort_rows_callback(a: &String, b: &String, unencoded_at_top: bool) -> Ordering {
    let ab: Vec<_> = [a, b]
        .iter()
        .map(|s| {
            let hex_vec = s.chars().take_while(|c| c.is_ascii_hexdigit()).collect::<String>();
            let hex = isize::from_str_radix(&hex_vec, 16).unwrap_or(if unencoded_at_top { -1 } else { isize::MAX });
            hex
        })
        .collect();
    let (a, b): (isize, isize) = (ab[0], ab[1]);
    a.cmp(&b)
}

pub fn glyphs(path: &std::ffi::OsStr, args: &clap::ArgMatches) {
    let dr = DataRequest::none().layers(true).data(true);
    let ufo = Font::load_requested_data(path, &dr).expect("Failed to load UFO w/norad");
    let do_sort = args.is_present("sort");
    let unencoded_at_top = args.is_present("unencoded-at-top");
    let hide_unencoded = args.is_present("hide-unencoded");
    let mut glyph_rows: Vec<_> = ufo
        .default_layer()
        .iter()
        .map(|g: &Arc<Glyph>| {
            let mut ret = vec![];
            // If we're sorting by codepoint, we want to list codepoints multiple times.
            // So break the norad Glyph out of its Arc, then just reset its "codepoints".
            let mut g = Clone::clone(&**g);
            if g.codepoints.len() == 0 && hide_unencoded {
                return String::new();
            }
            if do_sort {
                let codepoints = g.codepoints.to_owned();
                for cp in codepoints {
                    g.codepoints = vec![cp];
                    ret.push(glyph_row(&g, true));
                }
            }
            // Unencoded glyph or no sorting by Unicode value, will join encoding values on same
            // line where they exist.
            if ret.len() == 0 || !do_sort {
                ret.push(glyph_row(&g, do_sort));
            }
            ret.join("")
        })
        .collect();
    if args.is_present("sort") {
        glyph_rows.sort_by(|a, b| sort_rows_callback(a, b, unencoded_at_top));
    }
    for row in glyph_rows {
        print!("{}", row);
    }
}

pub fn glyph(path: &std::ffi::OsStr, _args: &clap::ArgMatches) {
    let g = Glyph::load(path).unwrap();
    print!("{}", glyph_row(&g, false));
}
