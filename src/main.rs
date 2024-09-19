#![allow(warnings)]

mod constants;
mod signals_parser;
mod structs;
mod vss_matcher;
mod vss_parser;

use clap::{Arg, Command};
use constants::*;

fn main() {
    let matches = Command::new(CLI_NAME)
        .version(VERSION)
        .author(AUTHOR)
        .about(ABOUT)
        .arg(
            Arg::new(ARG_SIGNALS)
                .short(ARG_SIGNALS_SHORT)
                .long(ARG_SIGNALS_LONG)
                .value_name(ARG_SIGNALS_VALUE_NAME)
                .help(ARG_SIGNALS_HELP)
                .default_value(ARG_SIGNALS_DEFAULT),
        )
        .arg(
            Arg::new(ARG_VSS)
                .short(ARG_VSS_SHORT)
                .long(ARG_VSS_LONG)
                .value_name(ARG_VSS_VALUE_NAME)
                .help(ARG_VSS_HELP)
                .default_value(ARG_VSS_DEFAULT),
        )
        .arg(
            Arg::new(ARG_NUM)
                .short(ARG_NUM_SHORT)
                .long(ARG_NUM_LONG)
                .value_name(ARG_NUM_VALUE_NAME)
                .help(ARG_NUM_HELP)
                .default_value(ARG_NUM_DEFAULT),
        )
        .get_matches();

    let signals_file = matches
        .get_one::<String>(ARG_SIGNALS)
        .expect("Can not parse signal file");
    let vss_file = matches
        .get_one::<String>(ARG_VSS)
        .expect("Can not parse vss file");
    let num: usize = matches
        .get_one::<String>(ARG_NUM)
        .unwrap()
        .parse()
        .expect("Can not parse num");

    println!("source signals: {}", signals_file);
    println!("vss json: {}", vss_file);
    println!("number of results: {}/signal", num);

    let signals = signals_parser::read_signals(&signals_file);
    let collection = vss_parser::read_signals(&vss_file);

    let results = vss_matcher::vss_match(signals, collection);

    for (signal, result) in results {
        println!("\n-------------");
        println!("signal: {} (length: {})\n", signal, signal.len());

        for i in 0..num {
            println!("{:?}", result.get(i));
        }
    }
}
