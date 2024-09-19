use crate::structs::{MapSignal, SignalData};

use std::{cmp::Reverse, collections::HashMap};

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

pub fn vss_match(
    signals_v3: &Vec<String>,
    signals_v4: &Vec<SignalData>,
) -> HashMap<String, Vec<MapSignal>> {
    let mut result = HashMap::new();

    for signal_v3 in signals_v3 {
        let mut map_signal = vec![];

        for signal_v4 in signals_v4 {
            let counter = calulate_match_pattern(&signal_v3, &signal_v4.signal);

            map_signal.push(MapSignal {
                signal: signal_v4.signal.clone(),
                counter,
            })
        }

        map_signal.sort_by_key(|m| Reverse(m.counter));

        result.insert(signal_v3.to_owned(), map_signal);
    }

    result
}
