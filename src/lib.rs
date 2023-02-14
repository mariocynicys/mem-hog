use rand::{distributions::Alphanumeric, Rng};
pub use std::collections::{HashMap, HashSet};

type Key = [u8; 16];
type Value = [u8; 20];

/// The amount of dummy data bytes in `BigStruct`.
const DUMMY_DATA_COUNT: usize = 512;

/// The type of the dummy data field in `BigStruct`.
/// If you set dummy data to a static size array: `type DummyData = [u8; DUMMY_DATA_COUNT];`
/// there is no difference between iter and non-iter fills, not sure why.
type DummyData = [u8; DUMMY_DATA_COUNT];//`Vec<u8>;

/// A struct to hold the `Value` for `Key` plus some dummy data to bloat the memory.
pub struct BigStruct {
    pub uuid: Value,
    pub dum: DummyData,
}

/// The type of the collection to collect the `(Key, BigStruct)` pairs into in `fill_map_iter` & `fill_map`.
/// Try to set it to `Vec<(Key, BigStruct)>`, yields different results.
type CollectionType = HashMap<Key, BigStruct>;

/// Returns an iterator of random `(Key, BigStruct)` pairs.
pub fn generate_random_pairs(amount: u32) -> impl Iterator<Item = (Key, BigStruct)> {
    let mut rng = rand::thread_rng();
    let dum: DummyData = (&mut rng)
        .sample_iter(Alphanumeric)
        .take(DUMMY_DATA_COUNT)
        .collect::<Vec<u8>>()
        .try_into()
        .unwrap();
    (0..amount).map(move |_| {
        (
            rng.gen(), // A random key.
            BigStruct {
                uuid: rng.gen(),  // This is the value we will use for inverse map.
                dum: dum.clone(), // Extra dummy data to bloat the memory.
            },
        )
    })
}

/// Fills the inverse map without storing/collecting any data beforehand.
/// Just loops over an iterator of random data without collecting this iterator in-memory.
pub fn fill_map_light(inverse_map: &mut HashMap<Value, HashSet<Key>>, amount: u32) {
    let iter = generate_random_pairs(amount);
    for (key, val) in iter {
        let (key, val) = (key, val.uuid);
        if let Some(set) = inverse_map.get_mut(&val) {
            set.insert(key);
        } else {
            inverse_map.insert(val, HashSet::from_iter(vec![key]));
        }
    }
}

/// Fills the inverse map by collecting the random data iterator then iterating over it
/// using `iter()`.
/// This takes the most memory but is trimmable to the same size as `fill_map_light`.
pub fn fill_map_iter(inverse_map: &mut HashMap<Value, HashSet<Key>>, amount: u32) {
    let map: CollectionType = generate_random_pairs(amount).collect();
    for (key, val) in map.iter() {
        let (key, val) = (*key, val.uuid);
        if let Some(set) = inverse_map.get_mut(&val) {
            set.insert(key);
        } else {
            inverse_map.insert(val, HashSet::from_iter(vec![key]));
        }
    }
}

/// Fills the inverse map by collecting the random data iterator then consuming it
/// and filling it's content directly in the inverse map.
/// This takes some memory slightly less than `fill_map_iter`, but the downside is that it's
/// not trimmable, i.e. It uses about 30% more memory compared to `fill_map_light` and this memory
/// isn't freed until the inverse map is freed, which can be a big problem if the inverse map is long-lived.
pub fn fill_map(inverse_map: &mut HashMap<Value, HashSet<Key>>, amount: u32) {
    let map: CollectionType = generate_random_pairs(amount).collect();
    for (key, val) in map {
        let (key, val) = (key, val.uuid);
        if let Some(set) = inverse_map.get_mut(&val) {
            set.insert(key);
        } else {
            inverse_map.insert(val, HashSet::from_iter(vec![key]));
        }
    }
}
