use clap::{arg, Command};
use std::path::PathBuf;

pub fn cli() -> Command {
    Command::new("kamura")
        .about("Kamura Engine")
        .arg_required_else_help(true)
        .arg(arg!(-p --perseus <PATH> "Perseus Model Path").value_parser(clap::value_parser!(PathBuf)))
}