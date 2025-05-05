use calamine::{Data, DataType};

#[derive(Debug, Clone, Default)]
pub struct Recipe {
    pub name: String,
    pub book: String,
    pub tags: Vec<String>,
}

pub enum RecipeError {
    CannotGetValue,
}

impl TryFrom<(&[String], &[Data])> for Recipe {
    type Error = RecipeError;

    fn try_from((columns, data): (&[String], &[Data])) -> Result<Self, Self::Error> {
        let mut recipe = Recipe::default();
        for (k, v) in columns.iter().zip(data) {
            match k.as_str() {
                "Recipe" => recipe.name = v.as_string().ok_or(RecipeError::CannotGetValue)?,
                "Book" => recipe.book = v.as_string().ok_or(RecipeError::CannotGetValue)?,
                "Tags" => {
                    recipe.tags = v
                        .as_string()
                        .ok_or(RecipeError::CannotGetValue)?
                        .split(",")
                        .map(str::trim)
                        .map(String::from)
                        .collect()
                }
                _ => {}
            }
        }
        Ok(recipe)
    }
}
