pub enum CommandData {
    VSS(String),
    Num(usize),
    Run(String),
    Exit,
    Invalid(String),
}

pub fn parse_command(command: &String) -> Option<CommandData> {
    let trimmed = command.trim();

    if trimmed.is_empty() {
        return None;
    };

    let mut parts = trimmed.split_whitespace();
    let command = parts.next().unwrap();

    match command {
        "vss" => {
            if let Some(vss_file) = parts.next() {
                return Some(CommandData::VSS(vss_file.to_owned()));
            }
            return Some(CommandData::Invalid(
                "VSS-file path is required".to_string(),
            ));
        }
        "num" => {
            if let Some(num) = parts.next() {
                if let Ok(num) = num.parse::<usize>() {
                    return Some(CommandData::Num(num));
                }
                return Some(CommandData::Invalid("Can not parse into usize".to_string()));
            }
            return Some(CommandData::Invalid("num (usize) is required".to_string()));
        }
        "run" => {
            if let Some(signals_file) = parts.next() {
                return Some(CommandData::Run(signals_file.to_owned()));
            }
            return Some(CommandData::Invalid(
                "signals-file path is required".to_string(),
            ));
        }
        "exit" => Some(CommandData::Exit),
        _ => Some(CommandData::Invalid("Command not found".to_owned())),
    }
}

pub fn show_help() {
    println!("------ H E L P ------");
    println!("vss <path-2-VSS-file>: set vss-file");
    println!("num <usize value>: set number of results/signal");
    println!("run <path-2-signals-file>: find top related signals in VSS file of each signal in signal file");
    println!("exit: quit cli");
    println!("---------------------");
}
