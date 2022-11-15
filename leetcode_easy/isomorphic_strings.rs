use std::collections::HashMap;

fn is_isomorphic(s: String, t: String) -> bool {
    let (mut st, mut ts) = (HashMap::new(), HashMap::new());
    for (x, y) in s.chars().zip(t.chars()) {
        if (st.contains_key(&x) && st.get(&x).unwrap() != &y)
            || (ts.contains_key(&y) && ts.get(&y).unwrap() != &x)
        {
            return false;
        }
        st.insert(x, y);
        ts.insert(y, x);
    }
    true
}

#[cfg(test)]
pub mod tests {
    use crate::is_isomorphic;
    #[test]
    fn run_tc1() {
        let s = String::from("egg");
        let t = String::from("add");
        assert_eq!(is_isomorphic(s, t), true);
    }
    #[test]
    fn run_tc2() {
        let s = String::from("foo");
        let t = String::from("bar");
        assert_eq!(is_isomorphic(s, t), false);
    }
    #[test]
    fn run_tc3() {
        let s = String::from("paper");
        let t = String::from("title");
        assert_eq!(is_isomorphic(s, t), true);
    }
}

fn main() {
    println!("Hello, world!");
    let s = String::from("paper");
    let t = String::from("title");
    assert_eq!(is_isomorphic(s, t), true);
}
