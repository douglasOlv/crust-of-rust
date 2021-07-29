pub mod strsplit;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let haystack = "a b c d e";
        let leters: Vec<_> = strsplit::StrSplit::new(haystack, " ").collect();
        assert_eq!(leters, vec!["a", "b", "c", "d", "e"])
    }
}
