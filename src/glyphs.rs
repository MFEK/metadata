use clap;
use glifparser::{read_from_filename, Glif};
use itertools::Itertools as _;
use std::fs::read_dir;
use std::path::PathBuf;
use unic_ucd::category::GeneralCategory;
use unic_ucd::name::Name;

use std::cmp::Ordering;
use std::str::FromStr as _;

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

fn add_headers(_args: &clap::ArgMatches) {
    print!("glifname\tcodepoints\tuniname\tunicat\tfilename\n");
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

fn glyph_row(g: &Glif<()>) -> String {
    let codepoints = g.unicode.clone().into_iter().collect();
    let mut ret = format!("{}\t{}\t", &g.name, codepoints_to_string(&codepoints));
    if g.unicode.len() > 0 {
        ret.push_str(&format!(
            "{}\t",
            (g.unicode
                .iter()
                .map(|cp| Name::of(*cp).map(|n| name_to_string(&n)).unwrap_or(unnamed_name(*cp).to_string()))
                .collect::<Vec<String>>())
            .join(",")
        ));
        ret.push_str(&format!(
            "{}\t",
            (g.unicode.iter().map(|cp| format!("{:?}", GeneralCategory::of(*cp))))
                .collect::<Vec<String>>()
                .join(",")
        ));
    } else {
        ret.push_str("\t\t");
    }
    ret.push_str(
        (g.filename.as_ref())
            .map(|pb| pb.as_path())
            .unwrap_or(PathBuf::from_str("<NULL>").unwrap().as_path())
            .to_str()
            .unwrap(),
    );
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
    let mut path = PathBuf::from(path);
    path.push("glyphs");
    let ufo = read_dir(path).unwrap().map(|dr| dr.unwrap()).collect_vec();
    let do_sort = args.is_present("sort");
    let unencoded_at_top = args.is_present("unencoded-at-top");
    let hide_unencoded = args.is_present("hide-unencoded");
    let mut glyph_rows: Vec<_> = ufo
        .iter()
        .map(|g| {
            let mut ret = vec![];
            let g = if let Ok(g) = read_from_filename(g.path()) { g } else { return None };
            if g.unicode.len() == 0 && hide_unencoded {
                return Some(String::new());
            }
            if do_sort {
                ret.push(glyph_row(&g));
            }
            // Unencoded glyph or no sorting by Unicode value, will join encoding values on same
            // line where they exist.
            if ret.len() == 0 || !do_sort {
                ret.push(glyph_row(&g));
            }
            Some(ret.join(""))
        })
        .filter(|g| g.is_some())
        .map(Option::unwrap)
        .collect();
    add_headers(args);
    if args.is_present("sort") {
        glyph_rows.sort_by(|a, b| sort_rows_callback(a, b, unencoded_at_top));
    }
    for row in glyph_rows {
        print!("{}", row);
    }
}

pub fn glyph(path: &std::ffi::OsStr, _args: &clap::ArgMatches) {
    let g = read_from_filename(path).unwrap();
    print!("{}", glyph_row(&g));
}
