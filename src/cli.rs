use std::path::PathBuf;

use clap::Parser;

use crate::query::Query;

/// Pick recipes
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// File with all of the recipes
    #[arg()]
    pub input: PathBuf,

    /// How many results do you want to get from the recipes
    #[arg()]
    pub results: usize,

    /// Tags to include or exclude
    #[arg(last = true)]
    pub tag: Vec<Query>,
}
