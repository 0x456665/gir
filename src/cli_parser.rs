use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    ///initialize a new repository
    Init {
        path: Vec<String>,
    },
    Add(AddCommand),
    CatFile,
    CheckIgnore,
    CheckOut,
    Commit,
    HashObject,
    Log,
    LsFiles,
    LsTree,
    RevParse,
    Rm,
    ShowRef,
    Status,
    Tag,
}

#[derive(Parser, Debug)]
pub struct InitCommand {
    ///path to initialize
    #[arg(default_value = ".")]
    pub path: Vec<String>,
}

#[derive(Parser, Debug)]
pub struct AddCommand {
    #[command(subcommand)]
    action: AddAction,
}

#[derive(Subcommand, Debug)]
enum AddAction {
    All,
    File,
}
