use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut res = BTreeMap::new();

    for (score, leters) in h {
        for &leter in leters {
            res.insert(leter.to_ascii_lowercase(), *score);
        }}res
}
