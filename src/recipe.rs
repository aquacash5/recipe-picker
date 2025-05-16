use calamine::{Data, DataType};
use unicase::UniCase;

#[derive(Debug, Clone, Default)]
pub struct Recipe {
    pub name: UniCase<String>,
    pub book: UniCase<String>,
    pub tags: Vec<UniCase<String>>,
}

impl From<(&[String], &[Data])> for Recipe {
    fn from((columns, data): (&[String], &[Data])) -> Self {
        assert!(columns.len() == data.len());
        let mut recipe = Recipe::default();
        let data = data
            .iter()
            .map(Data::as_string)
            .map(|d| d.map(UniCase::new));
        let columns = columns.iter().map(Option::Some);
        let pairs = columns.zip(data).flat_map(|(k, v)| k.zip(v));
        for (k, v) in pairs {
            match k.as_str() {
                "Recipe" => recipe.name = v,
                "Book" => recipe.book = v,
                "Category" => recipe.tags.push(v),
                "Type" => recipe.tags.push(v),
                "Tags" => {
                    recipe.tags.extend(
                        v.split(",")
                            .map(str::trim)
                            .map(String::from)
                            .map(UniCase::new),
                    );
                }
                _ => {}
            }
        }
        recipe
    }
}

impl PartialEq for Recipe {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}

impl Eq for Recipe {}

impl PartialOrd for Recipe {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Recipe {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.book == other.book {
            self.name.cmp(&other.name)
        } else {
            self.book.cmp(&other.book)
        }
    }
}
