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

    // Extracts a variable and uses a new temp variable in its place
    ExtractVariable(ExtractVariableCommand),
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

#[derive(Debug, Args)]
pub struct ExtractVariableCommand {
    /// File path of target file
    pub file_path: String,

    /// Variable name to extract
    pub extraction_name: String,

    /// New variable name
    pub new_name: String,
}
