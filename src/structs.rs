use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SignalData {
    pub name: String,
    pub signal: String,
    pub uuid: String,
}

#[derive(Debug)]
pub struct MapSignal {
    pub signal: String,
    pub counter: u8,
}

#[derive(Debug)]
pub struct Database {
    pub vss: Vec<SignalData>,
}

#[derive(Debug)]
pub struct Params {
    pub vss_file: String,
    pub num: usize,
}