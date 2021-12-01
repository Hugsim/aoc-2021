use crate::util::*;

pub fn solve() -> (Option<i64>, Option<i64>) {
    let contents = read_file_to_vec::<i32>("src/days/input/1");

    let mut res1 = 0;
    let mut last = &contents[0];
    for d in &contents[1..] {
        if d > last {
            res1 += 1;
        }
        last = d;
    }

    let mut res2 = 0;
    let mut last: i32 = contents[0..=2].iter().sum();
    for w in contents[1..].windows(3) {
        dprint(w);
        let sum = w.iter().sum();
        if sum > last {
            res2 += 1;
        }
        last = sum; 
    }

    (Some(res1 as i64), Some(res2 as i64))
}