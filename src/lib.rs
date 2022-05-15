pub mod bloom;
pub mod events;
pub mod mpsc;
pub mod strsplit;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strsplit_works() {
        let haystack = "a b c d e";
        let leters: Vec<_> = strsplit::StrSplit::new(haystack, " ").collect();
        assert_eq!(leters, vec!["a", "b", "c", "d", "e"])
    }

    #[test]
    fn event_emiter_works() {
        let mut events = events::EventEmiter::new();

        events.on("hello", |v| assert_eq!(v, "world"));
        events.emit("hello", &"world".to_string());
        events.emit("hello", &String::from("world"));
    }

    #[test]
    fn mpsc_pig_pong() {
        let (mut tx, mut rx) = mpsc::channel();
        tx.send(42);

        assert_eq!(rx.recv(), Some(42))
    }

    #[test]
    fn bloom_filter() {
        let mut bloom = bloom::Bloom::new();
        bloom.push(&30);
        assert!(bloom.contains(&30));
        assert!(!bloom.contains(&42));
        bloom.push(&42);
        assert!(bloom.contains(&42));
    }

    #[test]
    fn sha2() {
        use sha2::Digest;

        let mut hash = sha2::Sha512::new();
        hash.update("123");
        let hashed = hash.finalize();
        let str_hash = hex::encode(hashed);

        assert_eq!(str_hash, "3c9909afec25354d551dae21590bb26e38d53f2173b8d3dc3eee4c047e7ab1c1eb8b85103e3be7ba613b31bb5c9c36214dc9f14a42fd7a2fdb84856bca5c44c2")
    }
}
