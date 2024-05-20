use chrono::{DateTime, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, time::Duration};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Ingredient {
    pub name: String,
}

impl std::fmt::Display for Ingredient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
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

impl std::fmt::Display for Recipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Task {
    pub description: String,
    pub completed: bool,
    pub duration: Option<Duration>,
    pub start_time: Option<NaiveTime>,
    pub completion_time: Option<DateTime<Utc>>,
}
