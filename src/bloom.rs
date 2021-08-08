use bit_vec::BitVec;
use std::collections::hash_map::{DefaultHasher, RandomState};
use std::hash::{BuildHasher, Hash, Hasher};
pub struct Bloom {
    bitmap: BitVec,
    hashers: [DefaultHasher; 2],
}

impl Bloom {
    pub fn new(len: usize) -> Self {
        let nbits = Self::bitmap_size(len, 0.01);
        let bitmap = BitVec::from_elem(nbits, false);
        let hashers = [
            RandomState::new().build_hasher(),
            RandomState::new().build_hasher(),
        ];
        Bloom { bitmap, hashers }
    }
    pub fn push(&mut self, item: &impl Hash) {
        let (hash1, hash2) = self.get_hash(item);
        self.bitmap.set(hash1 as usize, true);
        self.bitmap.set(hash2 as usize, true);
    }

    pub fn contains(&mut self, item: &impl Hash) -> bool {
        let (hash1, hash2) = self.get_hash(item);

        if self.bitmap.get(hash1 as usize).unwrap() {
            return true;
        } else if self.bitmap.get(hash2 as usize).unwrap() {
            return true;
        }

        false
    }

    fn get_hash(&mut self, item: &impl Hash) -> (u64, u64) {
        let mut hash1 = self.hashers[0].clone();
        let mut hash2 = self.hashers[1].clone();

        item.hash(&mut hash1);
        item.hash(&mut hash2);

        (hash1.finish() % 0xf, hash2.finish() % 0xf)
    }

    fn bitmap_size(items_count: usize, fp_rate: f64) -> usize {
        let ln2_2 = core::f64::consts::LN_2 * core::f64::consts::LN_2;
        ((-1.0f64 * items_count as f64 * fp_rate.ln()) / ln2_2).ceil() as usize
    }
}
