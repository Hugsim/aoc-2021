use crate::util::*;

pub fn solve() -> (Option<i32>, Option<i32>) {
    let contents = read_file_to_string_iter("src/days/input/3");
    let init = vec![Vec::new(); 12];
    let contents = contents.map(|s| s.chars().collect()).fold(
        init,
        |mut acc: Vec<Vec<char>>, s: Vec<char>| {
            for (i, c) in s.into_iter().enumerate() {
                acc[i].push(c);
            }
            acc
        },
    );

    let res: String = contents
        .into_iter()
        .map(|vs| {
            let num_1 = vs.clone().into_iter().filter(|v| *v == '1').count();
            let num_0 = vs.into_iter().filter(|v| *v == '0').count();
            if num_0 > num_1 {
                '0'
            } else {
                '1'
            }
        })
        .collect();

    let gamma_rate = i32::from_str_radix(&res, 2).unwrap();

    let res_inv: String = res.chars().map(|c| {
        match c {
            '0' => '1',
            '1' => '0',
            _ => unreachable!()
        }
    }).collect();

    let epsilon_rate = i32::from_str_radix(&res_inv, 2).unwrap();

    (Some(gamma_rate * epsilon_rate), None)
}
