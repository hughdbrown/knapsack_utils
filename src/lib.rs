use prng::Prng;

#[derive(Debug, Clone)]
pub struct Item {
    pub value: u64,
    pub weight: usize,
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
