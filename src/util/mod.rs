#![allow(unused)]

use env_logger;
use log;

#[derive(Debug)]
enum ValidatorRange {
    All,
    PositiveNotZero,
    Positive,
}

fn arg_validator_f64_impl(v: &str, range: ValidatorRange) -> Result<(), String> {
    match v.parse::<f64>() {
        Ok(i) => {
            let err = || Err(String::from(&format!("Value outside allowed range ({:?})", range)));
            match range {
                ValidatorRange::All => Ok(()),
                ValidatorRange::PositiveNotZero => {
                    if i >= 0.0 {
                        Ok(())
                    } else {
                        err()
                    }
                }
                ValidatorRange::Positive => {
                    if i > 0.0 {
                        Ok(())
                    } else {
                        err()
                    }
                }
            }
        }
        Err(_) => Err(String::from("Value must be a float")),
    }
}

pub fn arg_validator_positive_f64(v: &str) -> Result<(), String> {
    arg_validator_f64_impl(v, ValidatorRange::Positive)
}

pub fn arg_validator_positive_or_zero_f64(v: &str) -> Result<(), String> {
    arg_validator_f64_impl(v, ValidatorRange::PositiveNotZero)
}

pub fn arg_validator_f64(v: &str) -> Result<(), String> {
    arg_validator_f64_impl(v, ValidatorRange::All)
}

pub fn arg_validator_isize(v: &str) -> Result<(), String> {
    match v.parse::<isize>() {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("Value must be an integer")),
    }
}

pub fn arg_validator_usize(v: &str) -> Result<(), String> {
    match v.parse::<isize>() {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("Value must be a positive integer")),
    }
}

pub fn arg_validator_suffix(f: &impl Fn(&str) -> Result<(), String>, suffix: char) -> impl Fn(String) -> Result<(), String> + '_ {
    move |mut v| {
        while v.ends_with(suffix) {
            assert_eq!(v.pop().unwrap(), suffix);
        }
        f(&v)
    }
}

macro_rules! exit {
    ($($args:tt)*) => {
    {
        log::error!($($args)*);
        std::process::exit(1);
    }
    }
}
pub(crate) use exit;

pub fn init_env_logger() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
}
