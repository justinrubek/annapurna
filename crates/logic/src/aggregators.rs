/// computes the missing ingredients for each recipe and returns the ingredients needed to make it
pub fn vec_missing<'a, TInputIter>(
    ingredients: Vec<String>,
) -> impl Fn(TInputIter) -> std::vec::IntoIter<Vec<String>>
where
    TInputIter: Iterator<Item = (&'a String,)>,
{
    move |available| {
        let ingredients = ingredients.clone();
        let available: Vec<String> = available.map(|(i,)| i.to_string()).collect();
        let missing = ingredients
            .into_iter()
            .filter(|ingredient| !available.contains(ingredient))
            .collect::<Vec<String>>();

        vec![missing].into_iter()
    }
}
