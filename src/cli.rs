use std::path::PathBuf;
use structopt::StructOpt;

/// Generates documentation from handlebars templates.
#[derive(StructOpt)]
pub struct Cli {
    /// The input folder with templates
    pub input: PathBuf,
    /// The output folder
    pub output: PathBuf,
}
