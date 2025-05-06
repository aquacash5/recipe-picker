use calamine::{Xlsx, open_workbook};
use clap::Parser;
use cli::Cli;
use itertools::Itertools;
use rand::seq::IteratorRandom;
use recipe::Recipe;

mod cli;
mod query;
mod recipe;

fn main() {
    let args = Cli::parse();

    let mut workbook: Xlsx<_> = open_workbook(&args.input).expect("Cannot open file");
    workbook.load_tables().expect("Cannot find sheet");
    let table_name = workbook
        .table_names_in_sheet("Recipes")
        .first()
        .unwrap()
        .to_string();
    let recipes = workbook
        .table_by_name(&table_name)
        .expect("Cannot read Recipes table");

    let recipes: Vec<Recipe> = recipes
        .data()
        .rows()
        .map(|row| Recipe::from((recipes.columns(), row)))
        .collect();

    println!();

    match args.command {
        cli::Commands::Tags => recipes
            .iter()
            .flat_map(|r| r.tags.iter().map(ToOwned::to_owned))
            .sorted()
            .unique()
            .for_each(|tag| println!("{tag}")),
        cli::Commands::Sample { results, tags } => {
            let mut rng = rand::rng();
            let mut recipes = recipes
                .iter()
                .filter(|recipe| tags.iter().all(|q| q.matches(&recipe.tags)))
                .choose_multiple(&mut rng, results);
            recipes.sort();
            let max_book_len = recipes
                .iter()
                .map(|r| r.book.len())
                .max()
                .expect("No recipes");
            // TODO: Show how many recipes were filtered out.
            for r in recipes {
                println!("{:width$}: {}", r.book, r.name, width = max_book_len);
            }
        }
    }
}
