use std::collections::HashSet;
use std::collections::VecDeque;
fn min_mutation(start_gene:String, end_gene:String, bank: Vec<String>)->i32 {
    let bank_set: HashSet<String> = bank.into_iter().collect();
    if !bank_set.contains(&end_gene) {
        return -1;
    }
    let mut queue = VecDeque::new();
    queue.push_back((start_gene.clone(), 0));
    let mut visited: HashSet<String> = HashSet::new();
    visited.insert(start_gene);
    assert!(!queue.is_empty());
    while !queue.is_empty() {
        let ele = queue.pop_front().unwrap();
        let (gene, mut level) = ele;
        if gene == end_gene {
            return level;
        }
        let chrs: Vec<char> = gene.chars().into_iter().collect();
        println!("gene and  level: {}, {}", gene, level);
        for (i, _c) in chrs.iter().enumerate() {
            let mut mut_gene = String::from("");
            for l in "ACGT".chars(){
                mut_gene = String::from(&gene[0..i]) + &l.to_string() + &gene[i+1..]; 
                if !visited.contains(&mut_gene) && bank_set.contains(&mut_gene) {
                    queue.push_back((mut_gene.clone(), level + 1));
                    visited.insert(mut_gene);
                }
            }
        }
    }
    -1
}

fn main() {
    println!("Hello, world!");
    let start_gene = String::from("AACCGGTT");
    let end_gene  = String::from("AAACGGTA");
    let bank = vec![String::from("AACCGGTA"), String::from("AACCGCTA"), String::from("AAACGGTA")];
    println!("{}", min_mutation(start_gene, end_gene, bank));
}
