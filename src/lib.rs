use prng::Prng;

pub type SearchResult = (Vec<usize>, usize, u64);

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Item {
    pub value: u64,
    pub weight: usize,
}

impl Item {
    pub fn density(&self) -> f64 {
        (self.value as f64) / (self.weight as f64)
    }
}

pub fn sort_by_density(items: &[Item]) -> Vec<Item> {
    let mut tmp_items: Vec<(f64, Item)> = items.iter()
        .map(|item: &Item| (item.density(), item.clone())) // Make tuples of float64 and Item
        .collect::<Vec<(f64, Item)>>();

    tmp_items.sort_by(|t1: &(f64, Item), t2: &(f64, Item)| {
        // Sort by float64 value descending
        let density_1: f64 = t1.0;
        let density_2: f64 = t2.0;
        density_2.partial_cmp(&density_1).unwrap()
    });

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
    min_weight: usize, max_weight: usize,
) -> Vec<Item> {
    let mut items: Vec<Item> = vec![];
    for _ in 0..num_items {
        items.push(make_rand_item(
            prng,
            min_value, max_value,
            min_weight as u64, max_weight as u64,
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
            Item {value:3, weight:1}, // density=3
            Item {value:1, weight:6}, // density=0.16
            Item {value:1, weight:10}, // density=0.10
            Item {value:1, weight:8}, // density=0.125
            Item {value:1, weight:1}, // density=1
            Item {value:6, weight:1}, // density=6
        ];
        let sorted = sort_by_density(&items);
        let correct: Vec<usize> = vec![5, 0, 4, 1, 3, 2];
        for (i, ix) in correct.iter().enumerate() {
            assert!(sorted[i] == items[*ix]);
        }
    }

    #[test]
    fn density_2() {
        let items: Vec<Item> = vec![
            Item {value:60, weight:60},
            Item {value:60, weight:30},
            Item {value:60, weight:20},
            Item {value:60, weight:15},
            Item {value:60, weight:12},
            Item {value:60, weight:10},
            Item {value:60, weight:6},
            Item {value:60, weight:5},
            Item {value:60, weight:4},
            Item {value:60, weight:3},
            Item {value:60, weight:2},
            Item {value:60, weight:1},
        ];
        let sorted = sort_by_density(&items);
        let correct: Vec<usize> = vec![11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        for (i, ix) in correct.iter().enumerate() {
            assert!(sorted[i] == items[*ix]);
        }
    }

    #[test]
    fn density_3() {
        let items: Vec<Item> = vec![
            Item {value:1, weight:60},
            Item {value:2, weight:60},
            Item {value:3, weight:60},
            Item {value:4, weight:60},
            Item {value:5, weight:60},
            Item {value:6, weight:60},
            Item {value:10, weight:60},
            Item {value:12, weight:62},
            Item {value:15, weight:65},
            Item {value:20, weight:60},
            Item {value:30, weight:60},
            Item {value:60, weight:60},
        ];
        let sorted = sort_by_density(&items);
        let correct: Vec<usize> = vec![11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        for (i, ix) in correct.iter().enumerate() {
            assert!(sorted[i] == items[*ix]);
        }
    }

}
