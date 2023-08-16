use prng::Prng;

pub type SearchResult = (Vec<usize>, usize, u64);

#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    pub value: u64,
    pub weight: usize,
}

pub fn sort_by_density(items: &[Item]) -> Vec<Item> {
    let mut tmp_items: Vec<(f64, Item)> = vec![];
    for item in items.iter() {
        let density: f64 = (item.value as f64) / (item.weight as f64);
        tmp_items.push((density, item.clone())); // Make tuples of float64 and Item
    }

    tmp_items.sort_by(|t1, t2| t2.0.partial_cmp(&t1.0).unwrap()); // Sort by float64 value descending
    // println!("{:?}", tmp_items);

    tmp_items.into_iter()
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
    fn density_1() {
        let items: Vec<Item> = vec![
            Item {weight:1, value:3}, // density=3
            Item {weight:1, value:1}, // density=1
            Item {weight:6, value:1}, // density=0.16
            Item {weight:1, value:6}, // density=6
        ];
        let sorted = sort_by_density(&items);
        let correct: Vec<usize> = vec![3, 0, 1, 2];
        for (i, ix) in correct.iter().enumerate() {
            assert!(sorted[i] == items[*ix]);
        }
    }
}
