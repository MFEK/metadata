use norad::Font;

pub fn glyphslen(ufo: &Font) {
    println!("{}", ufo.default_layer().len())
}
