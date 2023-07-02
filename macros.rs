use std::collections::BTreeMap;
use std::collections::HashMap;

macro_rules! map_string_object {
    ($($k: expr => $v: expr),* $(,)?) => {
        std::iter::Iterator::collect(IntoIterator::into_iter([$(($k.to_string(), $v),)*]))
    };
}

fn main() {
    let config_map: HashMap<String, String> = map_string_object! {
        "Test" => String::from("test_val"),
        "Calculate" => String::from("calculate_val"),
    };
    println!("{:#?}", config_map);

    let counter: BTreeMap<String, u32> = map_string_object! {
        "31" => "31".parse().expect("[!!] Unable to parse"),
        "32" => "31".parse().expect("[!!] Unable to parse"),
        "33" => "31".parse().expect("[!!] Unable to parse"),
        "34" => "31".parse().expect("[!!] Unable to parse"),
    };

    println!("{:#?}", counter);
}
