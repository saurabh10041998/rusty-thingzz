use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;
use std::hash::BuildHasher;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

pub struct Splitmix64 {}

impl Splitmix64 {
    fn splitmix64(mut x: u64) -> u64 {
        x += 0x9e3779b97f4a7c15;
        x = (x ^ (x >> 30)) * 0xbf58476d1ce4e5b9;
        x = (x ^ (x >> 27)) * 0x94d049bb133111eb;
        return x ^ (x >> 31);
    }
}

pub struct CustomHasher{
    state: u64,
}

impl Hasher for CustomHasher {
    fn write(&mut self, bytes: &[u8]) {
        println!("[+] Computing the hash for : {:?}", bytes);
        for &byte in bytes {
            self.state = self.state.rotate_left(8) ^ u64::from(byte);
        }
        // computation on the key..
        let fixed_random = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs();
        self.state = Splitmix64::splitmix64(self.state + fixed_random);
    }
    fn finish(&self) -> u64 {
        eprintln!("hasher is called: {:#010x}", self.state);
        self.state
    }
}

pub struct BuildCustomHasher;

impl BuildHasher for BuildCustomHasher {
    type Hasher = CustomHasher;
    fn build_hasher(&self) -> Self::Hasher {
        println!("Preparing the hash function");
        CustomHasher {
            state: 0
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        println!("[+] Fetching the point");
        (self.x, self.y).hash(state)
    }
}


fn main() {
    let mut hm = HashMap::with_hasher(BuildCustomHasher);
    hm.insert(Point {x: 2, y: 3}, -1);
    println!("{:?}", hm);
}