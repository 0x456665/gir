use std::path::PathBuf;

use clap::Parser;
// use configparser::ini::Ini;
use gir::cat_file::cat_file;
use gir::cli_parser::{CatFileCommand, Cli, Commands, HashObjectCommand, InitCommand, ObjectType};
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
        Commands::HashObject(HashObjectCommand{object_type, write, path}) => {
            let object_type = match object_type {
                ObjectType::Blob => "blob",
                ObjectType::Commit => "commit",
                ObjectType::Tag => "tag",
                ObjectType::Tree => "tree",
            };
            let object_type = object_type.to_string();
            let path = path.clone();
            println!("object_type: {object_type}");
            println!("write: {write}");
            println!("path: {path}");
        }
        _ => {
            println!("Invalid Command, Check help")
        }
    }
    println!("args are: {cli:?}");
}
