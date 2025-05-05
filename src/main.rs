use calamine::{Xlsx, open_workbook};
use clap::Parser;
use cli::Cli;
use rand::seq::IteratorRandom;
use recipe::Recipe;

mod cli;
mod query;
mod recipe;

fn main() {
    let args = Cli::parse();
    let mut rng = rand::rng();

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
        .flat_map(|row| Recipe::try_from((recipes.columns(), row)))
        .filter(|recipe| args.tag.iter().all(|q| q.matches(&recipe.tags)))
        .choose_multiple(&mut rng, args.results);

    let max_book_len = recipes
        .iter()
        .map(|r| r.book.len())
        .max()
        .expect("No recipes");

    for r in recipes {
        println!("{:width$}: {}", r.book, r.name, width = max_book_len);
    }
}
