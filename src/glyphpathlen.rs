use clap::{self, ArgMatches};
use kurbo::ParamCurveArclen;
use serde_json as sj;
use MFEKmath::{piecewise::SegmentIterator, Piecewise};

use crate::util;

pub fn clap_subcommand<'help>() -> clap::Command<'help> {
    clap::Command::new("glyphpathlen")
        .setting(clap::AppSettings::DeriveDisplayOrder)
        .about("Show length of contours in a glyph (.glif) on separate lines")
        .arg(
            clap::Arg::new("segmentwise")
                .short('s')
                .long("segmentwise")
                .help("Display length of each segment separated by spaces"),
        )
        .arg(
            clap::Arg::new("joined")
                .long("joined")
                .short('j')
                .help("Display one line: sum of joined path"),
        )
        .arg(clap::Arg::new("json").long("json").short('J').help("Output JSON instead"))
        .arg(
            clap::Arg::new("accuracy")
                .long("accuracy")
                .help("Precision of length calculation")
                .takes_value(true)
                .default_value("0.01")
                .forbid_empty_values(true)
                .number_of_values(1)
                .validator(util::arg_validator_positive_f64),
        )
}

pub fn glyphpathlen(path: &std::ffi::OsStr, args: &ArgMatches) {
    let glif = glifparser::read_from_filename::<_, ()>(path).expect("Failed to parse .glif file");
    let mut seglens = vec![];
    let accuracy = args.value_of("accuracy").unwrap().parse().unwrap();
    let outline = glif.outline.as_ref().expect("Glif contains no outline data");
    let pw = Piecewise::from(outline);

    for contour in pw.segs.iter() {
        let si = SegmentIterator::new(contour.clone());
        let mut path = kurbo::BezPath::new();
        path.move_to(Into::<(f64, f64)>::into(contour.segs[0].w1));
        for seg in si {
            path.curve_to(
                Into::<(f64, f64)>::into(seg.0.w2),
                Into::<(f64, f64)>::into(seg.0.w3),
                Into::<(f64, f64)>::into(seg.0.w4),
            );
        }
        if contour.is_closed() {
            path.close_path();
        }
        let seglen: Vec<f64> = path.segments().map(|seg| seg.arclen(accuracy)).collect();
        seglens.push(seglen);
    }

    fn make_pathlens(seglens: &Vec<Vec<f64>>) -> Vec<f64> {
        let mut pathlens = vec![];

        for sl in seglens.iter() {
            pathlens.push(sl.iter().sum::<f64>());
        }

        pathlens
    }

    if args.is_present("segmentwise") {
        if args.is_present("json") {
            println!("{}", sj::to_string(&seglens).unwrap());
        } else {
            for sl in seglens {
                println!("{}", sl.iter().map(|f| format!("{:.4}", f)).collect::<Vec<_>>().join(" "));
            }
        }
    } else if args.is_present("joined") {
        let joined = make_pathlens(&seglens).iter().sum::<f64>();
        if args.is_present("json") {
            println!("{}", sj::to_string(&[joined]).unwrap());
        } else {
            println!("{:.4}", joined);
        }
    } else {
        let pathlens = make_pathlens(&seglens);
        if args.is_present("json") {
            println!("{}", sj::to_string(&pathlens).unwrap());
        } else {
            for pl in pathlens {
                println!("{:.4}", pl);
            }
        }
    }
}
