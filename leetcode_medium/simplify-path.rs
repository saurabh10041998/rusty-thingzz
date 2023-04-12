fn simplify_path(path: String) -> String {
    let mut stack = vec![];
    let path = path.split("/");
    for p in path {
        match p {
            "." | "" => {}
            ".." => match stack.pop() {
                Some(_tok) => {}
                None => {}
            },
            _ => stack.push(String::from(p)),
        }
    }
    let mut ans = String::from("/");
    ans.push_str(stack.join("/").as_str());
    ans
}

#[cfg(test)]
pub mod tests {
    use crate::simplify_path;
    #[test]
    fn run_tc1() {
        let path = String::from("/home/");
        assert_eq!(simplify_path(path), String::from("/home"));
    }
    #[test]
    fn run_tc2() {
        let path = String::from("/../");
        assert_eq!(simplify_path(path), String::from("/"));
    }
    #[test]
    fn run_tc3() {
        let path = String::from("/home//foo/");
        assert_eq!(simplify_path(path), String::from("/home/foo"));
    }
}

fn main() {
    let path = String::from("/../");
    assert_eq!(simplify_path(path), String::from("/"));
}
