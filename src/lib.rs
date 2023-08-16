use prng::Prng;

pub type SearchResult = (Vec<usize>, usize, u64);

#[derive(Debug, Clone)]
pub struct Item {
    pub value: u64,
    pub weight: usize,
}

pub sort_by_density(items: &[Item]) -> Vec<Item> {
    items.clone() // Necessary to clone?
        .map(|x: Item| (f64(x.value) / f64(x.weight)), x) // Make tuples of float64 and Item
        .sort(|t1, t2| t2.0.partialcmp(t1.0)) // Sort by float64 value descending
        .map(|t: (f64, Item)| t.1) // Drop the float64
        .collect::<Vec::<Item>>() // Collect the ordered Items
}

pub fn sum_values(items: &[Item]) -> u64 {
    items.iter().map(|x| x.value).sum()
}

pub fn sum_weights(items: &[Item]) -> usize {
    items.iter().map(|x| x.weight).sum()
}

pub fn make_rand_item(
    prng: &mut Prng,
    min_value: u64, max_value: u64,
    min_weight: u64, max_weight: u64,
) -> Item {
    Item {
        value: prng.next_u64(min_value, max_value),
        weight: prng.next_u64(min_weight, max_weight) as usize,
    }
}

pub fn make_items(
    prng: &mut Prng,
    num_items: usize,
    min_value: u64, max_value: u64,
    min_weight: u64, max_weight: u64,
) -> Vec<Item> {
    let mut items: Vec<Item> = vec![];
    for _ in 0..num_items {
        items.push(make_rand_item(
            prng,
            min_value, max_value,
            min_weight, max_weight,
        ));
    }
    items
}

pub fn select_items(items: &[Item], selected: &[usize]) -> Vec<Item> {
    items.iter()
        .enumerate()
        .filter(|x: &(usize, &Item)| selected.contains(&x.0))
        .map(|x: (usize, &Item)| x.1.clone())
        .collect::<Vec::<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
