use csv::StringRecord;

#[derive(Debug, Clone, Default)]
pub struct Recipe {
    pub name: String,
    pub book: String,
    pub tags: Vec<String>,
}

impl From<(StringRecord, StringRecord)> for Recipe {
    fn from((columns, data): (StringRecord, StringRecord)) -> Self {
        assert!(columns.len() == data.len());
        let mut recipe = Recipe::default();
        let data = data.iter().map(String::from);
        let pairs = columns.iter().zip(data);
        for (k, v) in pairs {
            match k {
                "Recipe" => recipe.name = v,
                "Book" => recipe.book = v,
                "Category" => recipe.tags.push(v),
                "Type" => recipe.tags.push(v),
                "Tags" => {
                    recipe
                        .tags
                        .extend(v.split(",").map(str::trim).map(String::from));
                }
                _ => {}
            }
        }
        recipe
    }
}
