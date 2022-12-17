fn is_operator(t: &str) -> bool {
    match t {
        "+" => true,
        "-" => true,
        "*" => true,
        "/" => true,
        _ => false,
    }
}

fn get_result(op1: &String, operator: &str, op2: &String) -> String {
    let op1 = op1.parse::<i32>().unwrap();
    let op2 = op2.parse::<i32>().unwrap();
    let result;
    match operator {
        "+" => result = op1 + op2,
        "-" => result = op1 - op2,
        "*" => result = op1 * op2,
        "/" => result = op1 / op2,
        _ => panic!("Invalid operation"),
    };
    format!("{}", result)
}

fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut eval_stack = vec![];
    for t in tokens {
        if is_operator(&t) {
            // eval
            println!("{:?}", eval_stack);
            let op2;
            match eval_stack.pop() {
                Some(val) => {
                    op2 = val;
                }
                None => {
                    panic!("Invalid operation sequence");
                }
            }
            let op1;
            match eval_stack.pop() {
                Some(val) => {
                    op1 = val;
                }
                None => {
                    panic!("Invalid operation sequence");
                }
            }
            let result = get_result(&op1, &t, &op2);
            eval_stack.push(result);
        } else {
            eval_stack.push(t);
        }
    }
    eval_stack.last().unwrap().parse::<i32>().unwrap()
}

#[cfg(test)]
pub mod tests {
    use crate::eval_rpn;
    #[test]
    fn run_tc1() {
        let tokens = ["2", "1", "+", "3", "*"];
        let mut t = vec![];
        for _t in tokens {
            t.push(String::from(_t));
        }
        assert_eq!(eval_rpn(t), 9);
    }

    #[test]
    fn run_tc2() {
        let tokens = ["4", "13", "5", "/", "+"];
        let mut t = vec![];
        for _t in tokens {
            t.push(String::from(_t));
        }
        assert_eq!(eval_rpn(t), 6);
    }

    #[test]
    fn run_tc3() {
        let tokens = [
            "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+",
        ];
        let mut t = vec![];
        for _t in tokens {
            t.push(String::from(_t));
        }
        assert_eq!(eval_rpn(t), 22);
    }
}

fn main() {
    let tokens = ["2", "1", "+", "3", "*"];
    let mut t = vec![];
    for _t in tokens {
        t.push(String::from(_t));
    }
    assert_eq!(eval_rpn(t), 9);
}
