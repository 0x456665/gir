use std::path::PathBuf;

use clap::Parser;
// use configparser::ini::Ini;
use gir::cli_parser::{Cli, Commands};
use gir::init::init;
fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Init { path } => {
            let path_buf = PathBuf::from(path[0].clone());
            init::init_repo(path_buf).unwrap();
        }
        _ => {}
    }
    println!("args are: {cli:?}");
}
