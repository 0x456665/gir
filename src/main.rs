use std::path::PathBuf;

use clap::Parser;
// use configparser::ini::Ini;
use gir::cat_file::cat_file;
use gir::cli_parser::{CatFileCommand, Cli, Commands, InitCommand, ObjectType};
use gir::init::init;

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Init(InitCommand { path }) => {
            let path_buf = PathBuf::from(path[0].clone());
            init::init_repo(path_buf).unwrap();
        }
        Commands::CatFile(CatFileCommand {
            object_type,
            object_hash_or_ref,
        }) => match object_type {
            ObjectType::Blob => {
                cat_file(object_hash_or_ref, "blob");
            }
            ObjectType::Commit => {
                cat_file(object_hash_or_ref, "commit");
            }
            ObjectType::Tag => {
                cat_file(object_hash_or_ref, "tag");
            }
            ObjectType::Tree => {
                cat_file(object_hash_or_ref, "tree");
            }
        },
        _ => {
            println!("Invalid Command, Check help")
        }
    }
    println!("args are: {cli:?}");
}
