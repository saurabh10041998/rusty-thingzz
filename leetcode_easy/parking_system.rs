struct ParkingSystem {
    big: i32,
    medium: i32,
    small: i32,
}

enum Car {
    Big,
    Medium,
    Small,
}

impl From<i32> for Car {
    fn from(value: i32) -> Self {
        match value {
            1 => Car::Big,
            2 => Car::Medium,
            3 => Car::Small,
            _ => unreachable!(),
        }
    }
}

impl ParkingSystem {
    fn new(big: i32, medium: i32, small: i32) -> Self {
        ParkingSystem { big, medium, small }
    }
    fn add_car(&mut self, car_type: i32) -> bool {
        let car: Car = car_type.into();
        match car {
            Car::Big => {
                self.big -= 1;
                self.big > -1
            }
            Car::Small => {
                self.small -= 1;
                self.small > -1
            }
            Car::Medium => {
                self.medium -= 1;
                self.medium > -1
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let mut system = ParkingSystem::new(1, 1, 0);
        assert_eq!(system.add_car(1), true);
        assert_eq!(system.add_car(2), true);
        assert_eq!(system.add_car(3), false);
        assert_eq!(system.add_car(1), false);
    }
    #[test]
    fn run_tc2() {
        let mut system = ParkingSystem::new(0, 0, 0);
        assert_eq!(system.add_car(1), false);
        assert_eq!(system.add_car(2), false);
        assert_eq!(system.add_car(3), false);
        assert_eq!(system.add_car(1), false);
    }
}

fn main() {
    let mut system = ParkingSystem::new(1, 1, 0);
    assert_eq!(system.add_car(1), true);
    assert_eq!(system.add_car(2), true);
    assert_eq!(system.add_car(3), false);
    assert_eq!(system.add_car(1), false);
}
