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