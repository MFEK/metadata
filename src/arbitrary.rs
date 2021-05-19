use enum_for_matches;
use norad::Font;
use serde_value::Value as SerdeValue;

pub fn arbitrary(ufo: &Font, keys: Vec<&str>) {
    let md = &ufo.meta;
    assert_eq!(
        md.format_version,
        norad::FormatVersion::V3,
        "UFO versions other than 3 unsupported"
    );
    let fi = ufo
        .font_info
        .as_ref()
        .expect("Norad failed to parse font metainfo");
    let map = serde_value::to_value(fi).expect("Failed to serialize fontinfo - not a serde Value?");

    for key in keys {
        match map {
            SerdeValue::Map(ref m) => {
                let arg = &SerdeValue::String(key.to_string());
                match m.get(arg) {
                    Some(SerdeValue::Option(ref o)) => enum_for_matches::run!(
                        **(o.as_ref().unwrap()),
                        {
                              SerdeValue::U8(ref oo)
                            | SerdeValue::U16(ref oo)
                            | SerdeValue::U32(ref oo)
                            | SerdeValue::U64(ref oo)
                            | SerdeValue::I8(ref oo)
                            | SerdeValue::I16(ref oo)
                            | SerdeValue::I32(ref oo)
                            | SerdeValue::I64(ref oo)
                            | SerdeValue::F32(ref oo)
                            | SerdeValue::F64(ref oo)
                            | SerdeValue::Char(ref oo)
                            | SerdeValue::String(ref oo)
                        },
                        {println!("{}", &oo);},
                        {panic!("Unimplemented request for array, option or dict");}
                    ),
                    _ => println!(""),
                }
            }
            _ => panic!("FontInfo not a BTreeMap"),
        }
    }
}
