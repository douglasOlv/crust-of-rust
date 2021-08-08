use std::collections::hash_map::{DefaultHasher, RandomState};
use std::collections::HashMap;
use std::hash::{BuildHasher, Hash, Hasher};
pub struct Bloom {
    bitmap: HashMap<u64, bool>,
    hashers: [DefaultHasher; 2],
}

impl Bloom {
    pub fn new() -> Self {
        let bitmap = HashMap::new();
        let hashers = [
            RandomState::new().build_hasher(),
            RandomState::new().build_hasher(),
        ];
        Bloom { bitmap, hashers }
    }
    pub fn push(&mut self, item: &impl Hash) {
        let (hash1, hash2) = self.get_hash(item);
        self.bitmap.insert(hash1, true);
        self.bitmap.insert(hash2, true);
    }

    pub fn contains(&mut self, item: &impl Hash) -> bool {
        let (hash1, hash2) = self.get_hash(item);

        if let Some(value) = self.bitmap.get(&hash1) {
            return *value;
        }

        if let Some(value) = self.bitmap.get(&hash2) {
            return *value;
        }

        false
    }

    fn get_hash(&mut self, item: &impl Hash) -> (u64, u64) {
        let mut hash1 = self.hashers[0].clone();
        let mut hash2 = self.hashers[1].clone();

        item.hash(&mut hash1);
        item.hash(&mut hash2);

        (hash1.finish(), hash2.finish())
    }
}
