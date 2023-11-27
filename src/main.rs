//! Print the AST for a given Python file.
#![allow(clippy::print_stdout, clippy::print_stderr)]

use clap::Parser;
use indicatif::{ProgressBar, ProgressIterator, ProgressState, ProgressStyle};
use std::{fmt::Write, path::PathBuf};

use anyhow::Result;
use pytest_typechecker::{check_and_parse_file, files::get_files_list};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Python pytest file or directory for which to check fixture types.
    #[arg(required = true, num_args(1..))]
    file: Vec<PathBuf>,

    /// Check files recursively.
    #[arg(required = false, short, long, default_value_t = false)]
    recursive: bool,
}

fn get_progress_bar(total_len: u64) -> ProgressBar {
    let pb = ProgressBar::new(total_len);

    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})",
        )
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
            write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
        })
        .progress_chars("#>-"),
    );

    pb
}

fn main() -> Result<()> {
    let args = Args::parse();

    let files = get_files_list(args.file, args.recursive);

    let pb = get_progress_bar(files.len() as u64);
    for file in check_and_parse_file(files.iter().progress_with(pb)) {
        print!("{}", file);
    }

    Ok(())
}
