use clap::ArgMatches;
use glifparser::{Glif, PointData};
use kurbo::ParamCurveArclen;
use serde_json as sj;
use MFEKmath::{Piecewise, piecewise::SegmentIterator};

pub fn glyphpathlen(glif: Glif<()>, args: &ArgMatches) {
    let mut seglens = vec![];
    let accuracy = args.value_of("accuracy").unwrap().parse().unwrap();
    let outline = glif.outline.as_ref().expect("Glif contains no outline data");
    let pw = Piecewise::from(outline);

    for contour in pw.segs.iter() {
        let si = SegmentIterator::new(contour.clone());
        let mut path = kurbo::BezPath::new();
        path.move_to((contour.segs[0].w1).to_f64_tuple());
        for seg in si {
            path.curve_to((seg.0.w2).to_f64_tuple(), (seg.0.w3).to_f64_tuple(), (seg.0.w4).to_f64_tuple());
        }
        if contour.is_closed() {
            path.close_path();
        }
        let seglen: Vec<f64> = path.segments().map(|seg|seg.arclen(accuracy)).collect();
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
                println!("{}", sl.iter().map(|f|format!("{:.4}", f)).collect::<Vec<_>>().join(" "));
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
