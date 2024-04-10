use std::path::PathBuf;

use clap::Parser;

/// Generates documentation from handlebars templates.
#[derive(Parser)]
#[command(version)]
pub struct Cli {
    /// The input folder with templates
    pub input: PathBuf,
    /// The output folder
    pub output: PathBuf,
}
