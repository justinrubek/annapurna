use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path};

pub mod types;

/// A collection of all known recipes and ingredients.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Facts {
    pub inventory: Vec<types::Ingredient>,

    pub recipes: Vec<types::Recipe>,
}

impl Facts {
    pub fn read_from_directory<P: AsRef<Path>>(
        path: P,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let path = path.as_ref();
        let inventory_path = path.join("inventory.ron");
        let recipes_path = path.join("recipes.ron");

        let inventory_contents = std::fs::read_to_string(inventory_path)?;
        let inventory_data: Vec<String> = ron::from_str(&inventory_contents)?;
        let inventory: Vec<types::Ingredient> = inventory_data
            .into_iter()
            .map(types::Ingredient::new)
            .collect();

        let recipes_contents = std::fs::read_to_string(recipes_path)?;
        let recipes_data: HashMap<String, Vec<String>> = ron::from_str(&recipes_contents)?;
        let recipes = types::Recipe::from_hashmap(recipes_data)
            .into_keys()
            .collect();

        Ok(Self { inventory, recipes })
    }
}
