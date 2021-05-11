use norad::Font;

pub fn glyphslen(ufo: &Font) {
    eprintln!("{}", ufo.default_layer().len())
}
