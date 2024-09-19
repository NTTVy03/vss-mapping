#![allow(warnings)]

mod cli;
mod command_parser;
mod constants;
mod signals_parser;
mod structs;
mod vss_matcher;
mod vss_parser;

use command_parser::*;
use std::io::{stdin, stdout, Write};
use structs::*;

fn main() {
    let mut params = cli::get_params();
    let mut database = Database {
        vss: vss_parser::read_signals(&params.vss_file),
    };

    let stdin = stdin();
    let mut stdout = stdout();

    show_help();

    println!("{:?}\n", params);

    loop {
        print!("> ");
        stdout.flush().unwrap();

        let mut input = String::new();
        stdin.read_line(&mut input).expect("Failed to read input");

        match parse_command(&input) {
            None => {
                // ignore empty command
            }
            Some(CommandData::Exit) => {
                break;
            }
            Some(CommandData::Invalid(message)) => {
                println!("{}\n", message);
                show_help();
            }
            Some(CommandData::Num(num)) => {
                params.num = num;
            }
            Some(CommandData::VSS(vss_file)) => {
                params.vss_file = vss_file;

                database.vss = vss_parser::read_signals(&params.vss_file);
            }
            Some(CommandData::Run(signal_file)) => {
                let input = signals_parser::read_signals(&signal_file);

                let results = vss_matcher::vss_match(&input, &database.vss);

                for (signal, result) in results {
                    println!("\n-------------");
                    println!("signal: {} (length: {})\n", signal, signal.len());

                    for i in 0..params.num {
                        println!("{:?}", result.get(i));
                    }
                }
            }
        }
    }
}
