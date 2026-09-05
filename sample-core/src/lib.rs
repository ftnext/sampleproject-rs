pub fn add_one(number: i32) -> i32 {
    number + 1
}

#[cfg(test)]
mod test_add_one {
    use super::add_one;

    #[test]
    fn 引数に1を加えた数を返す() {
        assert_eq!(add_one(1), 2);
        assert_eq!(add_one(-1), 0);
        assert_eq!(add_one(41), 42);
    }
}
