use clap::Parser;

use crate::preprocesser::transform::Transformation;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// List of preprocessers in order of operation
    #[arg(value_enum)]
    pub filter: Option<Vec<Transformation>>,
}
