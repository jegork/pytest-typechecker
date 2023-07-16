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
            .filter(|v| v.state != ArgTypeState::CorrectType)
            .map(|v| {
                return if v.state == ArgTypeState::IncorrectType {
                    format!("{}: incorrect type", v.name)
                } else {
                    format!("{}: missing type", v.name)
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
