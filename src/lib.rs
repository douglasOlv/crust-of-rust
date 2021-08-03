pub mod events;
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
}
