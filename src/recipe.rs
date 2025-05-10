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
        let mut recipe = Recipe::default();
        for (k, v) in columns.iter().zip(data) {
            if let Some(s_v) = v.as_string() {
                let s_v = UniCase::new(s_v);
                match k.as_str() {
                    "Recipe" => recipe.name = s_v,
                    "Book" => recipe.book = s_v,
                    "Category" => recipe.tags.push(s_v),
                    "Type" => recipe.tags.push(s_v),
                    "Tags" => {
                        recipe.tags.extend(
                            s_v.split(",")
                                .map(str::trim)
                                .map(String::from)
                                .map(UniCase::new),
                        );
                    }
                    _ => {}
                }
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
