use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::query::Query;

/// Pick recipes
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// File with all of the recipes
    #[arg(short = 'i', long = "input", env = "RECIPE_PICKER_INPUT")]
    pub input: PathBuf,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Print out tags
    Tags,

    /// Print out the total number of recipes
    Total,

    /// Get recipe samples
    Sample {
        /// How many results do you want to get from the recipes
        #[arg(short = 'r', long = "results", default_value_t = 5)]
        results: usize,

        /// Tags to include or exclude
        #[arg(last = true)]
        tags: Vec<Query>,
    },
}
