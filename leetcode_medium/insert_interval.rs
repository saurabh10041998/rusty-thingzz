// What could be possible states of two intervals overlap ??
pub enum Intervals {
    Partial,
    Full,
    NoOverlap,
}

impl Intervals {
    // Checking for the total, No overlap and partial overlap
    fn get_state(a: &Vec<i32>, b: &Vec<i32>) -> Self {
        if a[1] < b[0] || a[0] > b[1] {
            return Self::NoOverlap;
        } else if (b[0] >= a[0] && b[1] <= a[1]) || (a[0] >= b[1] && a[1] <= b[0]) {
            return Self::Full;
        }
        Self::Partial
    }
}

fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let n = intervals.len();        // Get the size of intervals
    let mut is_merged = false;      // Flag to keep track if interval is merged
    let mut i = 0;                 // Index to traverse through array
    let mut merged = vec![];    // Our answer
    let mut new_interval = new_interval;    // Make new_interval mutable
    while i < n {
        // Get the state of current interval with new_interval (new_interval is interval to be merged)
        let state = Intervals::get_state(&intervals[i], &new_interval);
        match state {
            // If intervals[i] non-overlapping and we have already merged new_interval, then break
            // Beacause all the intervals after intervals[i] will be non-overlapping with new_interval as intervals vec! is sorted
            Intervals::NoOverlap if is_merged => break,  
            
            // If intervals[i] is non overlapping and new_intervals in not merged yet
            // check for ordering among them..(which comes first)
            Intervals::NoOverlap => {
                if intervals[i][0] > new_interval[1] {      // new_interval come first
                    merged.push(new_interval.clone());      // merge new_interval
                    merged.push(intervals[i].clone());      // merge interval[i]
                    new_interval = intervals[i].clone();    // Ignore this 
                    is_merged = true;                       // We have merged new_interval yay :)
                } else {
                    merged.push(intervals[i].clone());      // Else intervals[i] comes first, merge only intervals[i]
                }
            }

            // If its full overlap, check which interval is bigger
            Intervals::Full => {                        
                if new_interval[0] >= intervals[i][0] {     // if intervals[i] overlapping the new_interval
                    merged.push(intervals[i].clone());      // merge bigger interval
                    is_merged = true;                       // Since new_interval lied inside intervals[i], we merged new_interval, YAYA :)
                }
            }

            // If its partial, wait!!! dont merge
            // [1, 3] and [2, 5] ==> then we can merge [1, 5] in future
            // just update the bounds
            Intervals::Partial => {                     
                new_interval[0] = i32::min(new_interval[0], intervals[i][0]);   //lower boundary
                new_interval[1] = i32::max(new_interval[1], intervals[i][1]);   // higher boundary
            }
        }
        i += 1;         // process next interval
    }
    // If there are still intervals[i] pending to be merged
    while i < n {
        assert!(is_merged);
        merged.push(intervals[i].clone());
        i += 1;
    }

    // If we processed entire array and new_intervals is out lier to all of those
    // finally push new_interval
    if i == n && !is_merged {
        merged.push(new_interval.clone());
        is_merged = true;
    }

    // While returning just check if our task is accomplished ?? 
    assert!(is_merged);

    // return our answer
    merged
}

#[cfg(test)]
pub mod tests {
    use crate::insert;
    #[test]
    fn run_tc1() {
        let intervals = vec![vec![1, 3], vec![6, 9]];
        let new_interval = vec![2, 5];
        assert_eq!(
            insert(intervals, new_interval),
            vec![vec![1, 5], vec![6, 9]]
        );
    }
    #[test]
    fn run_tc2() {
        let intervals = vec![
            vec![1, 2],
            vec![3, 5],
            vec![6, 7],
            vec![8, 10],
            vec![12, 16],
        ];
        let new_interval = vec![4, 8];
        assert_eq!(
            insert(intervals, new_interval),
            vec![vec![1, 2], vec![3, 10], vec![12, 16]]
        );
    }
    #[test]
    fn run_tc3() {
        let intervals = vec![vec![1, 2], vec![3, 5]];
        let new_interval = vec![6, 8];
        assert_eq!(
            insert(intervals, new_interval),
            vec![vec![1, 2], vec![3, 5], vec![6, 8]]
        );
    }
    #[test]
    fn run_tc4() {
        let intervals = vec![vec![1, 5]];
        let new_interval = vec![2, 3];
        assert_eq!(insert(intervals, new_interval), vec![vec![1, 5]]);
    }
    #[test]
    fn run_tc5() {
        let intervals = vec![vec![2, 5], vec![6, 7], vec![8, 9]];
        let new_interval = vec![0, 1];
        assert_eq!(
            insert(intervals, new_interval),
            vec![vec![0, 1], vec![2, 5], vec![6, 7], vec![8, 9]]
        )
    }
}
fn main() {
    let intervals = vec![vec![1, 3], vec![6, 9]];
    let new_interval = vec![2, 5];
    assert_eq!(
        insert(intervals, new_interval),
        vec![vec![1, 5], vec![6, 9]]
    );
}
