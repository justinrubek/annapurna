use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Ingredient {
    pub name: String,
}

impl std::string::ToString for Ingredient {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}

impl Ingredient {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl From<String> for Ingredient {
    fn from(name: String) -> Self {
        Self { name }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Recipe {
    pub name: String,
    pub ingredients: Vec<Ingredient>,
}

impl Recipe {
    pub fn new(name: String, ingredients: Vec<Ingredient>) -> Self {
        Self { name, ingredients }
    }

    pub fn from_hashmap(items: HashMap<String, Vec<String>>) -> HashMap<Recipe, Vec<Ingredient>> {
        items
            .into_iter()
            .map(|(name, ingredients)| {
                let ingredients = ingredients
                    .into_iter()
                    .map(Ingredient::new)
                    .collect::<Vec<Ingredient>>();
                (Recipe::new(name, ingredients.clone()), ingredients)
            })
            .collect::<HashMap<Recipe, Vec<Ingredient>>>()
    }

    pub fn flatten(items: HashMap<Recipe, Vec<Ingredient>>) -> Vec<(String, String)> {
        items
            .into_iter()
            .flat_map(|(recipe, ingredients)| {
                ingredients
                    .into_iter()
                    .map(move |ingredient| (recipe.to_string(), ingredient.to_string()))
            })
            .collect()
    }
}

impl std::string::ToString for Recipe {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}

impl std::fmt::Debug for Recipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)?;
        write!(f, " - ")?;
        self.ingredients
            .iter()
            .for_each(|i| write!(f, "{i:?}, ").unwrap());

        Ok(())
    }
}

impl std::hash::Hash for Recipe {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for Recipe {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl Eq for Recipe {}
