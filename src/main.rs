#![allow(warnings)]

mod cli;
mod command_parser;
mod constants;
mod signals_parser;
mod structs;
mod vss_matcher;
mod vss_parser;

use std::{
    collections::HashMap,
    io::{stdout, Write},
};

use command_parser::*;
use constants::*;
use linefeed::{Completer, Completion, Interface, Prompter, Terminal};
use structs::*;

use std::sync::Arc;
#[derive(Default)]
struct MyCompleter {
    commands: HashMap<String, Vec<String>>,
}

impl<Term: Terminal> Completer<Term> for MyCompleter {
    fn complete(
        &self,
        word: &str,
        prompter: &Prompter<Term>,
        start: usize,
        end: usize,
    ) -> Option<Vec<Completion>> {
        let commands = vec!["exit", "run", "vss", "num"];
        let completions: Vec<Completion> = commands
            .iter()
            .filter(|cmd| cmd.starts_with(word))
            .map(|cmd| Completion::simple(cmd.to_string()))
            .collect();

        Some(completions)
    }
}

fn main() {
    let interface = Interface::new(CLI_NAME).expect("Failed to create CLI interface");

    interface.set_history_size(HISTORY_SIZE);
    interface.set_completer(Arc::new(MyCompleter::default()));

    let mut params = cli::get_params();
    let mut database = Database {
        vss: vss_parser::read_signals(&params.vss_file),
    };

    show_help();

    println!("{:?}\n", params);

    loop {
        print!("> ");
        stdout().flush().unwrap();

        let res = interface.read_line().expect("Failed to read user input");

        match res {
            linefeed::ReadResult::Input(input) => {
                interface.add_history_unique(input.clone());

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
            _ => {}
        }
    }
}
