#[cfg(test)]
mod tests {
    use webassembly_test::webassembly_test;

    #[webassembly_test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[webassembly_test]
    fn it_does_not_work() {
        assert_eq!(2 + 2, 5);
    }

    #[webassembly_test]
    #[ignore]
    fn it_is_ignored() {
        assert_eq!(2 + 2, 5);
    }
}
