use std::collections::BTreeMap;
use std::collections::HashMap;

fn frequency_sort(s: String) -> String {
    let mut hm = HashMap::new();
    for c in s.chars() {
        hm.entry(c).and_modify(|v| *v += 1).or_insert(1);
    }

    let mut bmap: BTreeMap<usize, String> = BTreeMap::new();
    for (c, v) in hm {
        bmap.entry(v)
            .and_modify(|val| val.push_str(&c.to_string().repeat(v)))
            .or_insert(c.to_string().repeat(v));
    }

    let mut res = String::new();

    for (_k, v) in bmap.iter().rev() {
        res.push_str(v);
    }
    res
}

#[cfg(test)]
pub mod tests {
    use crate::frequency_sort;
    #[test]
    fn run_tc1() {}

    #[test]
    fn run_tc2() {
        let s = String::from("cccaaa");
        assert_eq!(frequency_sort(s), String::from("aaaccc"));
    }

    #[test]
    fn run_tc3() {
        let s = String::from("Aabb");
        assert_eq!(frequency_sort(s), String::from("Aabb"));
    }
}

fn main() {
    let s = String::from("tree");
    assert_eq!(frequency_sort(s), String::from("eert"));
}
