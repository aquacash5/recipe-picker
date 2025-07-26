use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::query::Query;

/// Recipe Picker
///
/// Pick recipes from a CSV of recipes and tags
#[derive(Parser, Debug)]
#[command(version, about, long_about)]
pub struct Cli {
    /// File with all of the recipes
    #[arg(short = 'i', long = "input", env = "RECIPE_PICKER_INPUT")]
    pub input: PathBuf,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Print out tags
    Tags,

    /// Print out the total number of recipes
    Total,

    /// Get recipe samples
    Sample {
        /// How many results do you want to get from the recipes
        #[arg(
            short = 'r',
            long = "results",
            env = "RECIPE_PICKER_RESULTS",
            default_value_t = 10
        )]
        results: usize,

        /// Tags to include or exclude
        #[arg(last = true)]
        tags: Vec<Query>,
    },
}
