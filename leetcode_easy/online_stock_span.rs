pub struct StockSpanner {
    history: Vec<(i32, i32)>
}

impl StockSpanner { 
    fn new() -> Self {
        StockSpanner {
            history: Vec::new()
        }
    }
    
    fn next(&mut self, price: i32) -> i32 {
        let mut span = 1;
        while self.history.last().is_some() && self.history.last().unwrap().0 <= price {
            let (_, ref val) = self.history.last().unwrap();
            span = span + val;
            self.history.pop();
        } 
        self.history.push((price, span));
        span
    }
}



#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() { 
        let mut obj = StockSpanner::new();
        assert_eq!(obj.next(100), 1);
        assert_eq!(obj.next(80), 1);
        assert_eq!(obj.next(60), 1);
        assert_eq!(obj.next(70), 2);
        assert_eq!(obj.next(60), 1);
        assert_eq!(obj.next(75), 4);
        assert_eq!(obj.next(85), 6);
    }
}

fn main() {
    println!("Hello, world!");
    let mut obj = StockSpanner::new();
    assert_eq!(obj.next(100), 1);
    assert_eq!(obj.next(80), 1);
    assert_eq!(obj.next(60), 1);
    assert_eq!(obj.next(70), 2);
    assert_eq!(obj.next(60), 1);
    assert_eq!(obj.next(75), 4);
    assert_eq!(obj.next(85), 6);
}
