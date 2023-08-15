#[derive(Debug, Clone)]
pub struct Item {
    pub value: u64,
    pub weight: usize,
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
