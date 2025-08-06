use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(about = "A Git implementation in Rust")]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    ///initialize a new repository
    Init(InitCommand),
    Add(AddCommand),
    ///simply prints the raw contents of an object
    CatFile(CatFileCommand),
    HashObject(HashObjectCommand),
    CheckIgnore,
    CheckOut,
    Commit,
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
pub struct CatFileCommand {
    /// Type of object to print
    pub object_type: ObjectType,
    /// Object hash or reference
    pub object_hash_or_ref: String,
}

#[derive(Parser, Debug)]
pub struct HashObjectCommand {
    /// Type of object to hash
    pub object_type: ObjectType,
    /// Actually write the object into the object database
    #[arg(short, short('w'), long)]
    pub write: bool,
    /// Object contents
    pub path: String,
}

#[derive(Parser, Debug)]
pub struct AddCommand {
    #[command(subcommand)]
    action: AddAction,
}

#[derive(Debug, ValueEnum, Clone)]
pub enum ObjectType {
    Blob,
    Commit,
    Tag,
    Tree,
}

#[derive(Subcommand, Debug)]
enum AddAction {
    All,
    File,
}
