use serde_json::Value;
use std::fs::File;
use std::io::BufReader;

pub fn read_signals(path: &str) -> Vec<String> {
    let file = File::open(path).expect("Failed to open file {path}");
    let reader = BufReader::new(file);

    let json: Value = serde_json::from_reader(reader).expect("Failed to parse file {path}");
    let mut signals = vec![];

    if let Value::Array(array) = json {
        for element in array {
            if let Value::String(value) = element {
                signals.push(value);
            }
        }
    }   

    signals
}