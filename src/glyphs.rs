use norad::{Font, Glyph};
use unic_ucd::category::GeneralCategory;
use unic_ucd::name::Name;

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

use std::sync::Arc;
fn print_glyph(g: Arc<Glyph>) {
    print!("{}\t{}\t", &g.name, codepoints_to_string(&g.codepoints));
    if g.codepoints.len() > 0 {
        print!(
            "{}\t",
            (g.codepoints
                .iter()
                .map(|cp| Name::of(*cp)
                    .map(|n| name_to_string(&n))
                    .unwrap_or(if *cp <= '\x1f' { "<unnamed>".into() } else { "<unencoded>".into() }))
                .collect::<Vec<String>>())
            .join(",")
        );
        print!(
            "{}\t",
            (g.codepoints
                .iter()
                .map(|cp| format!("{:?}", GeneralCategory::of(*cp))))
            .collect::<Vec<String>>()
            .join(",")
        );
    }
    println!("")
}

pub fn glyphs(ufo: &Font) {
    for g in ufo.default_layer().iter_contents() {
        print_glyph(g);
    }
}

pub fn glyph(g: Arc<Glyph>) {
    print_glyph(g);
}
