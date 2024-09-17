use serde::Deserialize;
use serde_json::Value;
use std::cmp::Reverse;
use std::fs::File;
use std::io::BufReader;

const JSON_V3: &str = "input/vss_release_4.0.json";
const JSON_V4: &str = "input/vss_release_4.0.json";

#[derive(Deserialize, Debug)]
struct SignalData {
    name: String,
    signal: String,
    uuid: String,
}

#[derive(Debug)]
struct MapSignal {
    signal: String,
    counter: u8,
}

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

fn read_signals(path: &str) -> Vec<SignalData> {
    let file = File::open(path).expect("Failed to open file {path}");
    let reader = BufReader::new(file);

    let json: Value = serde_json::from_reader(reader).expect("Failed to parse file {path}");

    let mut signals = Vec::new();
    json_to_signals(&json, &mut signals, "");

    // signals.sort_by_key(|s| s.signal.clone());

    signals
}

fn calulate_match_pattern(s1: &String, s2: &String) -> u8 {
    let counter_left = s1
        .chars()
        .zip(s2.chars())
        .take_while(|(c1, c2)| c1 == c2)
        .count();
    let counter_right = s1
        .chars()
        .rev()
        .zip(s2.chars().rev())
        .take_while(|(c1, c2)| c1 == c2)
        .count();

    let mut counter = (counter_left + counter_right).try_into().unwrap();

    if counter > s1.len().try_into().unwrap() {
        counter = counter / 2;
    }

    counter
}

fn main() {
    let signals_v4 = read_signals(JSON_V4);
    let signals_v3 = vec!["Vehicle.Cabin.Door.Row1.Left.IsLocked".to_string()];
    let num_result = 5;

    for signal_v3 in signals_v3 {
        println!("\n-----------------");
        println!("Signal v3 = {}\n", signal_v3);

        let mut map_signal = vec![];
        
        for signal_v4 in &signals_v4 {
            let counter = calulate_match_pattern(&signal_v3, &signal_v4.signal);
            
            map_signal.push(MapSignal {
                signal: signal_v4.signal.clone(),
                counter,
        })
        }

        map_signal.sort_by_key(|m| Reverse(m.counter));

        for i in 0..num_result {
            println!("{:?} - {}", map_signal[i].signal, map_signal[i].counter);
        }
    }
}
