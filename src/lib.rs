use prng::Prng;

#[derive(Debug, Clone)]
pub struct Item {
    pub value: u64,
    pub weight: usize,
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
    let mut items: Vec<Item> = Vec::with_capacity(num_items);
    for _ in 0..num_items {
        items.push(make_rand_item(
            prng,
            min_value, max_value,
            min_weight, max_weight,
        ));
    }
    items
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
