mod by;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = by::add(2, 2);
        assert_eq!(result, 4);
    }
}
