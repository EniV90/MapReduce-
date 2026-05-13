use std::hash::{DefaultHasher, Hash, Hasher};

pub fn ihash<K: Hash>(key: &K) -> usize {
    let mut hasher = DefaultHasher::new();

    key.hash(&mut hasher);

    let hash = hasher.finish();

    hash as usize
}
