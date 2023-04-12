use std::path::Path;
use std::path::Component;
use std::ffi::OsString;

fn simplify_path(path: String) -> String {
    let mut stack = vec![];
    let components = Path::new(path.as_str()).components();
    for c in components {
        match c {
            Component::RootDir => {},
            Component::ParentDir => {
                match stack.pop() {
                    Some(_tok) => {},
                    None => {}
                }
            },
            Component::CurDir => {},
            Component::Normal(_tok) => {
                let os_string = OsString::from(_tok);
                stack.push(os_string.into_string().unwrap());
            },
            _ => {
                unreachable!()
            }
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