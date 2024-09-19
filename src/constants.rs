pub const CLI_NAME: &str = "VSS mapping";
pub const VERSION: &str = "1.0";
pub const AUTHOR: &str = "NTTVy03";
pub const ABOUT: &str = "A simple CLI to find top matched signals in VSS JSON file";
pub const HISTORY_SIZE: usize = 100;

pub const ARG_VSS: &str = "vss";
pub const ARG_VSS_SHORT: char = 'v';
pub const ARG_VSS_LONG: &str = "vss";
pub const ARG_VSS_VALUE_NAME: &str = "JSON-FILE";
pub const ARG_VSS_DEFAULT: &str = "vss-core/vss_release_4.0.json";
pub const ARG_VSS_HELP: &str = "The VSS Json file, where we will search in";

pub const ARG_NUM: &str = "num";
pub const ARG_NUM_SHORT: char = 'n';
pub const ARG_NUM_LONG: &str = "num";
pub const ARG_NUM_VALUE_NAME: &str = "integer";
pub const ARG_NUM_DEFAULT: &str = "5";
pub const ARG_NUM_HELP: &str = "Number of top related signals for each signal in signals file";