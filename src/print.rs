use crate::function::{ArgTypeState, CheckedFunction};
use prettytable::{row, Table};
use std::process::exit;

pub fn pretty_print(funcs: Vec<CheckedFunction>) {
    let mut table = Table::new();

    table.set_titles(row!["Name", "Invalid args"]);

    for f in funcs {
        let s: Vec<String> = f
            .args
            .iter()
            .filter(|v| match v {
                ArgTypeState::CorrectType { .. } => false,
                _ => true,
            })
            .map(|v| {
                return match v {
                    ArgTypeState::IncorrectType {
                        name,
                        provided,
                        expected,
                    } => format!("{}: expected {}, provided: {}", name, expected, provided),
                    ArgTypeState::MissingType { name } => format!("{}: missing type", name),
                    ArgTypeState::Error { name, msg } => format!("{}: {}", name, msg.clone()),
                    _ => String::from(""),
                };
            })
            .collect();
        table.add_row(row![f.name, s.join("\n")]);
    }

    if table.len() > 0 {
        table.printstd();
        exit(1);
    } else {
        println!("All types are correct!");
    }
}
