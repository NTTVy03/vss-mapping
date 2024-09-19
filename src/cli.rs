
use clap::{Arg, Command};

use crate::Params;
use crate::constants::*;

pub fn get_params() -> Params {
    let matches = Command::new(CLI_NAME)
        .version(VERSION)
        .author(AUTHOR)
        .about(ABOUT)
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

    let vss_file = matches
        .get_one::<String>(ARG_VSS)
        .expect("Can not parse vss file")
        .to_string();
    let num: usize = matches
        .get_one::<String>(ARG_NUM)
        .unwrap()
        .parse()
        .expect("Can not parse num");

    Params { vss_file, num }
}