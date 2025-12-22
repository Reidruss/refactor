use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct RefactorArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Renames a variable in the specified file
    RenameVariable(RenameVariableCommand),
}

#[derive(Debug, Args)]
pub struct RenameVariableCommand {
    /// File path of target file
    pub file_path: String,

    /// Old variable name
    pub old_name: String,

    /// New variable name
    pub new_name: String,
}
