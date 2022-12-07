use std::collections::hash_map::{Entry, HashMap};
use std::hash::Hash;

pub fn bin_str_to_num(bin_str: &str) -> Result<usize, ()> {
    bin_str.chars().fold(Ok(0), |num, bit| {
        if bit == '1' {
            Ok(num? * 2 + 1)
        } else if bit == '0' {
            Ok(num? * 2)
        } else {
            Err(())
        }
    })
}

pub fn hash_increment<K>(map: &mut HashMap<K, usize>, key: K, count: usize)
where
    K: Eq + Hash,
{
    match map.entry(key) {
        Entry::Occupied(o) => *o.into_mut() += count,
        Entry::Vacant(v) => *v.insert(0) += count,
    };
}
