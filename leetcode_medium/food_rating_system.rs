//! This question is all about preparing the HashTable mappings
//! to optimize the search and update queries.
//! using `HashMap` : to optimize the search
//! using `BTreeSet`: so that we can highest rated food by refering to last element of set
//!
//! Create three datastructure
//! 1. name_to_cuisine map => Given the food name, return the cuisine type for that food.
//!                           This map is useful for search in `cuisine_to_food` map
//!
//! 2. name_to_food map =>  Given the food name, get food object corresponding to that food,
//!                         This is useful to update the ratings of the food.
//!                         Also obtained food_object is useful for search and update operation
//!                         in BTreeSet of food store in `cuisine_to_food` map.
//! 3. cuisine_to_food map => Given the cuisine name, return the BTreeSet of foods corresponding to
//!                           that setting.
//!                           Useful for :
//!                           Finding the highest rated food for that cuisine type
//!
//! Since we create new struct to represent (food, cuisine, rating) mapping called `Food`
//! We need to implement the `Ord` and `PartialOrd` trait for that custom `Food` struct
//! so that it can be ranked in BTreeSet stored in `cuisine_to_food` map
//!

use std::collections::BTreeSet;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Food {
    name: String,
    cuisine: String,
    rating: i32,
}

impl PartialOrd for Food {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.rating == other.rating {
            return other.name.partial_cmp(&self.name);
        }
        self.rating.partial_cmp(&other.rating)
    }
}
impl Ord for Food {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.rating == other.rating {
            return other.name.cmp(&self.name);
        }
        self.rating.cmp(&other.rating)
    }
}

struct FoodRatings {
    cuisine_to_food: HashMap<String, BTreeSet<Food>>,
    name_to_food: HashMap<String, Food>,
    name_to_cuisine: HashMap<String, String>,
}

impl FoodRatings {
    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        let mut cuisine_to_food = HashMap::new();
        let mut name_to_food = HashMap::new();
        let mut name_to_cuisine = HashMap::new();
        for idx in 0..foods.len() {
            let name = foods[idx].clone();
            let cuisine = cuisines[idx].clone();
            let rating = ratings[idx];

            let food = Food {
                name: name.clone(),
                cuisine: cuisine.clone(),
                rating,
            };

            cuisine_to_food
                .entry(cuisine.clone())
                .and_modify(|foodset: &mut BTreeSet<Food>| {
                    foodset.insert(food.clone());
                })
                .or_insert(BTreeSet::from([food.clone()]));
            name_to_food.entry(name.clone()).or_insert(food);
            name_to_cuisine.entry(name).or_insert(cuisine);
        }

        FoodRatings {
            cuisine_to_food,
            name_to_food,
            name_to_cuisine,
        }
    }

    fn change_rating(&mut self, food: String, new_rating: i32) {
        if let Some(food_obj) = self.name_to_food.get_mut(&food) {
            if let Some(cuisine) = self.name_to_cuisine.get(&food) {
                match self.cuisine_to_food.get_mut(cuisine) {
                    Some(foodset) => {
                        foodset.remove(food_obj);
                        food_obj.rating = new_rating;
                        foodset.insert(food_obj.clone());
                    }
                    None => unreachable!(),
                }
            }
        }
    }
    fn highest_rated(&self, cuisine: String) -> String {
        if let Some(foodset) = self.cuisine_to_food.get(&cuisine) {
            return foodset.iter().last().unwrap().name.clone();
        }
        unreachable!()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let food = vec![
            String::from("kimchi"),
            String::from("miso"),
            String::from("sushi"),
            String::from("moussaka"),
            String::from("ramen"),
            String::from("bulgogi"),
        ];
        let cuisine = vec![
            String::from("korean"),
            String::from("japanese"),
            String::from("japanese"),
            String::from("greek"),
            String::from("japanese"),
            String::from("korean"),
        ];
        let ratings = vec![9, 12, 8, 15, 14, 7];
        let mut fr = FoodRatings::new(food, cuisine, ratings);
        assert_eq!(
            fr.highest_rated(String::from("korean")),
            String::from("kimchi")
        );
        fr.change_rating(String::from("sushi"), 16);
        assert_eq!(
            fr.highest_rated(String::from("japanese")),
            String::from("sushi")
        );
        fr.change_rating(String::from("ramen"), 16);
        assert_eq!(
            fr.highest_rated(String::from("japanese")),
            String::from("ramen")
        );
    }
}

fn main() {
    let food = vec![
        String::from("kimchi"),
        String::from("miso"),
        String::from("sushi"),
        String::from("moussaka"),
        String::from("ramen"),
        String::from("bulgogi"),
    ];
    let cuisine = vec![
        String::from("korean"),
        String::from("japanese"),
        String::from("japanese"),
        String::from("greek"),
        String::from("japanese"),
        String::from("korean"),
    ];
    let ratings = vec![9, 12, 8, 15, 14, 7];
    let mut fr = FoodRatings::new(food, cuisine, ratings);
    assert_eq!(
        fr.highest_rated(String::from("korean")),
        String::from("kimchi")
    );
    fr.change_rating(String::from("sushi"), 16);
    assert_eq!(
        fr.highest_rated(String::from("japanese")),
        String::from("sushi")
    );
    fr.change_rating(String::from("ramen"), 16);
    assert_eq!(
        fr.highest_rated(String::from("japanese")),
        String::from("ramen")
    );
}
