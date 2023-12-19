//! This is in-place implementation,
//! apparently question has following constraint,
//! which made in-place possible
//! 0 <= img[i][j] <= 255
//! Each element (Even though represent as i32)in only a byte wide.
//! So we have remaining 24bits of zero on the left to hide our answer
//! So we can hide the calculate average by shifting it 8 bits
//! to left, AND with img[i][j]
//! Then after whole img matrix computation done
//! Shift each number in matrix by 8 bit to left
//! to get the required matrix

fn is_valid(x: isize, y: isize, n: isize, m: isize) -> bool {
    y >= 0 && y < n && x >= 0 && x < m
}

fn image_smoother(mut img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = img.len();
    let m = img[0].len();
    for y in 0..n {
        for x in 0..m {
            let mut cnt = 0;
            let mut sum = 0;
            for dy in [-1, 0, 1] {
                for dx in [-1, 0, 1] {
                    if is_valid(x as isize + dx, y as isize + dy, n as isize, m as isize) {
                        sum += (img[(y as isize + dy) as usize][(x as isize + dx) as usize]) & 0xff;
                        cnt += 1;
                    }
                }
            }
            let data = sum / cnt;
            img[y][x] = img[y][x] ^ (data << 8);
        }
    }
    for y in 0..n {
        for x in 0..m {
            img[y][x] = img[y][x] >> 8;
        }
    }
    img
}

#[cfg(test)]
pub mod tests {
    use crate::image_smoother;
    #[test]
    fn run_tc1() {
        let img = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        assert_eq!(
            image_smoother(img),
            vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]
        );
    }
    #[test]
    fn run_tc2() {
        let img = vec![vec![100, 200, 100], vec![200, 50, 200], vec![100, 200, 100]];
        assert_eq!(
            image_smoother(img),
            vec![
                vec![137, 141, 137],
                vec![141, 138, 141],
                vec![137, 141, 137]
            ]
        );
    }
}

fn main() {
    let img = vec![vec![100, 200, 100], vec![200, 50, 200], vec![100, 200, 100]];
    assert_eq!(
        image_smoother(img),
        vec![
            vec![137, 141, 137],
            vec![141, 138, 141],
            vec![137, 141, 137]
        ]
    );
}
