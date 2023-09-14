use std::collections::HashMap;

fn dfs(
    src: String,
    adj_lst: &mut HashMap<String, Vec<String>>,
    res: &mut Vec<String>,
    tick_len: usize,
) -> bool {
    if res.len() == tick_len + 1 {
        return true;
    }
    match adj_lst.get(&src) {
        Some(_x) => {}
        None => return false,
    }

    let temp = match adj_lst.get(&src) {
        Some(lst) => lst.clone(),
        None => unreachable!(),
    };

    for (idx, v) in temp.into_iter().enumerate() {
        adj_lst.get_mut(&src).unwrap().remove(idx);
        res.push(v.clone());
        if dfs(v.clone(), adj_lst, res, tick_len) {
            return true;
        }
        adj_lst.get_mut(&src).unwrap().insert(idx, v);
        res.pop().unwrap();
    }
    false
}

fn find_itinerary(mut tickets: Vec<Vec<String>>) -> Vec<String> {
    let tick_len = tickets.len();
    tickets.sort();
    let mut adj = HashMap::new();
    for e in tickets {
        let src = e[0].clone();
        let dst = e[1].clone();
        adj.entry(src)
            .and_modify(|v: &mut Vec<String>| v.push(dst.clone()))
            .or_insert(vec![dst]);
    }

    let mut res = vec![String::from("JFK")];
    dfs(String::from("JFK"), &mut adj, &mut res, tick_len);

    res
}

#[cfg(test)]
pub mod tests {
    use crate::find_itinerary;
    #[test]
    fn run_tc1() {
        let tickets = vec![
            vec![String::from("MUC"), String::from("LHR")],
            vec![String::from("JFK"), String::from("MUC")],
            vec![String::from("SFO"), String::from("SJC")],
            vec![String::from("LHR"), String::from("SFO")],
        ];
        assert_eq!(
            find_itinerary(tickets),
            vec![
                String::from("JFK"),
                String::from("MUC"),
                String::from("LHR"),
                String::from("SFO"),
                String::from("SJC")
            ]
        );
    }

    #[test]
    fn run_tc2() {
        let tickets = vec![
            vec![String::from("JFK"), String::from("SFO")],
            vec![String::from("JFK"), String::from("ATL")],
            vec![String::from("SFO"), String::from("ATL")],
            vec![String::from("ATL"), String::from("JFK")],
            vec![String::from("ATL"), String::from("SFO")],
        ];
        assert_eq!(
            find_itinerary(tickets),
            vec![
                String::from("JFK"),
                String::from("ATL"),
                String::from("JFK"),
                String::from("SFO"),
                String::from("ATL"),
                String::from("SFO")
            ]
        );
    }
}

fn main() {
    let tickets = vec![
        vec![String::from("JFK"), String::from("SFO")],
        vec![String::from("JFK"), String::from("ATL")],
        vec![String::from("SFO"), String::from("ATL")],
        vec![String::from("ATL"), String::from("JFK")],
        vec![String::from("ATL"), String::from("SFO")],
    ];
    assert_eq!(
        find_itinerary(tickets),
        vec![
            String::from("JFK"),
            String::from("ATL"),
            String::from("JFK"),
            String::from("SFO"),
            String::from("ATL"),
            String::from("SFO")
        ]
    );
}
