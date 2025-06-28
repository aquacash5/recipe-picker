use clap::Parser;
use cli::Cli;
use itertools::Itertools;
use rand::seq::IteratorRandom;
use recipe::Recipe;
use std::{fs::File, iter};
use unicase::UniCase;

mod cli;
mod query;
mod recipe;

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let csv_file = File::open(&args.input)?;
    let mut csv_reader = csv::Reader::from_reader(csv_file);

    let headers = csv_reader.headers()?.clone();

    let headers = iter::repeat(Some(headers));
    let records = csv_reader.records().map(Result::ok);

    let recipes = headers
        .zip(records)
        .flat_map(|(k, v)| k.zip(v))
        .map(Recipe::from);

    println!();

    match args.command {
        cli::Command::Tags => recipes
            .flat_map(|r| r.tags.into_iter())
            .sorted()
            .unique()
            .for_each(|tag| println!("{tag}")),
        cli::Command::Total => println!("{}", recipes.count()),
        cli::Command::Sample { results, tags } => {
            let mut rng = rand::rng();
            let mut recipes = recipes
                .filter(|recipe| tags.iter().all(|q| q.matches(&recipe.tags)))
                .choose_multiple(&mut rng, results);
            recipes.sort_by_key(|r| UniCase::unicode(format!("{} {}", r.book, r.name)));
            let max_book_len = recipes
                .iter()
                .map(|r| r.book.len())
                .max()
                .expect("No recipes");
            for r in recipes {
                println!("{:width$}: {}", r.book, r.name, width = max_book_len);
            }
        }
    }

    Ok(())
}
