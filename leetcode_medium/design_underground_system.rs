use std::collections::HashMap;
#[derive(Debug, PartialEq, Eq)]
struct UndergroundSystem {
    check_in_map: HashMap<i32, (String, i32)>,
    total_map: HashMap<(String, String), (i32, i32)>,
}

impl UndergroundSystem {
    fn new() -> Self {
        UndergroundSystem {
            check_in_map: HashMap::new(),
            total_map: HashMap::new(),
        }
    }

    fn check_in(&mut self, id: i32, start_station: String, t: i32) {
        self.check_in_map.entry(id).or_insert((start_station, t));
    }

    fn check_out(&mut self, id: i32, end_station: String, t: i32) {
        let (start_station, start_time) = match self.check_in_map.remove(&id) {
            Some(val) => val,
            None => unreachable!(),
        };
        self.total_map
            .entry((start_station, end_station))
            .and_modify(|(x, y)| {
                *x += t - start_time;
                *y += 1;
            })
            .or_insert((t - start_time, 1));
    }

    fn get_average_time(&mut self, start_station: String, end_station: String) -> f64 {
        match self.total_map.get(&(start_station, end_station)) {
            Some(&(total, count)) => (total as f64) / (count as f64),
            None => {
                unreachable!()
            }
        }
    }
}

// To view test result
//  cargo test -- --nocapture --test-threads 1

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let mut system = UndergroundSystem::new();
        system.check_in(45, String::from("Leyton"), 3);
        system.check_in(32, String::from("Paradise"), 8);
        system.check_in(27, String::from("Leyton"), 10);
        system.check_out(45, String::from("Waterloo"), 15);
        system.check_out(27, String::from("Waterloo"), 20);
        system.check_out(32, String::from("Cambridge"), 22);

        println!(
            "{}",
            system.get_average_time(String::from("Paradise"), String::from("Cambridge"))
        );
        println!(
            "{}",
            system.get_average_time(String::from("Leyton"), String::from("Waterloo"))
        );

        system.check_in(10, String::from("Leyton"), 24);
        println!(
            "{}",
            system.get_average_time(String::from("Leyton"), String::from("Waterloo"))
        );
        system.check_out(10, String::from("Waterloo"), 38);
        println!(
            "{}",
            system.get_average_time(String::from("Leyton"), String::from("Waterloo"))
        );
    }
    #[test]
    fn run_tc2() {
        let mut system = UndergroundSystem::new();
        system.check_in(10, String::from("Leyton"), 3);
        system.check_out(10, String::from("Paradise"), 8);
        println!(
            "{}",
            system.get_average_time(String::from("Leyton"), String::from("Paradise"))
        );
        system.check_in(5, String::from("Leyton"), 10);
        system.check_out(5, String::from("Paradise"), 16);
        println!(
            "{}",
            system.get_average_time(String::from("Leyton"), String::from("Paradise"))
        );
        system.check_in(2, String::from("Leyton"), 21);
        system.check_out(2, String::from("Paradise"), 30);
        println!(
            "{}",
            system.get_average_time(String::from("Leyton"), String::from("Paradise"))
        );
    }
}

fn main() {
    let mut system = UndergroundSystem::new();
    system.check_in(10, String::from("Leyton"), 3);
    system.check_out(10, String::from("Paradise"), 8);
    println!(
        "{}",
        system.get_average_time(String::from("Leyton"), String::from("Paradise"))
    );
    system.check_in(5, String::from("Leyton"), 10);
    system.check_out(5, String::from("Paradise"), 16);
    println!(
        "{}",
        system.get_average_time(String::from("Leyton"), String::from("Paradise"))
    );
    system.check_in(2, String::from("Leyton"), 21);
    system.check_out(2, String::from("Paradise"), 30);
    println!(
        "{}",
        system.get_average_time(String::from("Leyton"), String::from("Paradise"))
    );
}
