use serde_json::Value;
use std::fs::File;
use std::io::BufReader;

use crate::structs::SignalData;

fn parse_uuid(sub_object: &serde_json::Map<String, Value>) -> Option<String> {
    sub_object
        .get("uuid")
        .and_then(|v| v.as_str())
        .map(String::from)
}

fn parse_children<'a>(sub_object: &'a serde_json::Map<String, Value>) -> Option<&'a Value> {
    sub_object.get("children")
}

fn json_to_signals(json: &Value, signals: &mut Vec<SignalData>, parent_signal: &str) {
    if let Value::Object(object) = json {
        for (key, sub_json) in object {
            if let Value::Object(sub_object) = sub_json {
                let signal = if parent_signal.is_empty() {
                    key.clone()
                } else {
                    format!("{}.{}", parent_signal, key)
                };

                if let Some(uuid) = parse_uuid(sub_object) {
                    signals.push(SignalData {
                        name: key.clone(),
                        signal: signal.clone(),
                        uuid,
                    });

                    if let Some(children) = parse_children(sub_object) {
                        json_to_signals(children, signals, &signal);
                    }
                }
            }
        }
    }
}

pub fn read_signals(path: &str) -> Vec<SignalData> {
    let file = File::open(path).expect("Failed to open file {path}");
    let reader = BufReader::new(file);

    let json: Value = serde_json::from_reader(reader).expect("Failed to parse file {path}");

    let mut signals = Vec::new();
    json_to_signals(&json, &mut signals, "");

    signals
}