use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FoundationFoods {
    #[serde(rename = "FoundationFoods")]
    pub foundation_foods: Vec<FoodDetails>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BrandedFoods {
    #[serde(rename = "BrandedFoods")]
    pub branded_foods: Vec<FoodDetails>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FoodDetails {
    #[serde(rename = "dataType")]
    pub data_type: String,
    pub description: String,
    #[serde(rename = "fdcId")]
    pub fdc_id: isize,
    #[serde(rename = "foodClass")]
    pub food_class: FoodClass,
    #[serde(rename = "foodNutrients")]
    pub food_nutrients: Vec<FoodNutrient>,
    #[serde(rename = "foodPortions")]
    pub food_portions: Option<Vec<FoodPortion>>,
}

#[derive(Clone, Debug)]
pub struct Macros {
    pub amount: f32,
    pub kcal: Option<FoodNutrient>,
    pub protein: Option<FoodNutrient>,
    pub fat: Option<FoodNutrient>,
    pub carbohydrate: Option<FoodNutrient>,
}

impl FoodDetails {
    pub fn macros(&self) -> Macros {
        let nutrients: HashMap<NutrientName, FoodNutrient> = self.food_nutrients.iter().cloned().filter_map(|nutrient| {
            match nutrient.nutrient.name {
                NutrientName::Energy => Some((nutrient.nutrient.name.clone(), nutrient.clone())),
                NutrientName::TotalLipid => Some((nutrient.nutrient.name.clone(), nutrient.clone())),
                NutrientName::CarbohydrateBySummation => Some((nutrient.nutrient.name.clone(), nutrient.clone())),
                NutrientName::Protein => Some((nutrient.nutrient.name.clone(), nutrient.clone())),
                _ => None,
            }
        }).collect();
        Macros {
            amount: 0.0,
            kcal: nutrients.get(&NutrientName::Energy).map(|n| n.to_owned()),
            protein: nutrients.get(&NutrientName::Protein).map(|n| n.to_owned()),
            fat: nutrients.get(&NutrientName::TotalLipid).map(|n| n.to_owned()),
            carbohydrate: nutrients.get(&NutrientName::CarbohydrateBySummation).map(|n| n.to_owned()),
        }

    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum FoodClass {
    Branded,
    Composite,
    FinalFood,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FoodPortion {
    pub id: isize,
    #[serde(rename = "measureUnit")]
    pub measure_unit: MeasureUnit,
    pub modifier: Option<String>,
    #[serde(rename = "gramWeight")]
    pub gram_weight: f32,
    #[serde(rename = "sequenceNumber")]
    pub sequence_number: isize,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MeasureUnit {
    pub id: isize,
    pub name: String,
    pub abbreviation: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FoodNutrient {
    pub amount: Option<f32>,
    pub id: isize,
    pub nutrient: Nutrient,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Nutrient {
    pub id: isize,
    pub number: String,
    pub name: NutrientName,
    pub rank: isize,
    #[serde(rename = "unitName")]
    pub unit_name: UnitName,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum UnitName {
    #[serde(rename = "g")]
    Gram,
    #[serde(rename = "mg")]
    Milligram,
    #[serde(rename = "µg")]
    Microgram,
    #[serde(rename = "kcal")]
    Kcal,
    #[serde(rename = "kJ")]
    Kilojoule,
    #[serde(rename = "IU")]
    InternationalUnit,
    #[serde(rename = "sp gr")]
    SpecificGravity,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub enum NutrientName {
    #[serde(rename = "Carbohydrate, by summation")]
    CarbohydrateBySummation,
    #[serde(rename = "Carbohydrate, by difference")]
    CarbohydrateByDifference,
    Cholesterol,
    Energy,
    #[serde(rename = "Energy (Atwater Specific Factors)")]
    EnergyAtwaterSpecific,
    #[serde(rename = "Energy (Atwater General Factors)")]
    EnergyAtwaterGeneral,
    #[serde(rename = "Fatty acids, total saturated")]
    FattyAcidsSaturated,
    Fructose,
    Glucose,
    #[serde(rename = "Fiber, insoluble")]
    InsolubleFiber,
    #[serde(rename = "Fiber, soluble")]
    SolubleFiber,
    Lactose,
    Protein,
    #[serde(rename = "Sodium, NA")]
    Sodium,
    Starch,
    #[serde(rename = "Fiber, total dietary")]
    TotalDietaryFiber,
    #[serde(rename = "Total fat (NLEA)")]
    TotalFat,
    #[serde(rename = "Total lipid (fat)")]
    TotalLipid,
    #[serde(rename = "Sugars, Total")]
    TotalSugars,
    #[serde(untagged)]
    Other(String),
}
