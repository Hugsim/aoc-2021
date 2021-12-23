use crate::util::*;

pub fn solve() -> (Option<i32>, Option<i32>) {
    let contents = read_file_to_vec::<String>("src/days/input/3");
    let init = vec![Vec::new(); 12];
    let contents_transposed = contents.clone().into_iter().map(|s| s.chars().collect()).fold(
        init,
        |mut acc: Vec<Vec<char>>, s: Vec<char>| {
            for (i, c) in s.into_iter().enumerate() {
                acc[i].push(c);
            }
            acc
        },
    );

    let res1: String = contents_transposed
        .clone()
        .into_iter()
        .map(|vs| {
            let num_1 = vs.clone().into_iter().filter(|v| *v == '1').count();
            let num_0 = vs.clone().into_iter().filter(|v| *v == '0').count();
            if num_0 > num_1 {
                '0'
            } else {
                '1'
            }
        })
        .collect();

    let gamma_rate = i32::from_str_radix(&res1, 2).unwrap();

    let res1_inv: String = res1
        .chars()
        .map(|c| match c {
            '0' => '1',
            '1' => '0',
            _ => unreachable!(),
        })
        .collect();

    let epsilon_rate = bin_string_to_i32(&res1_inv);

    let sol1 = gamma_rate * epsilon_rate;

    // PART 2

    dprint("p2 starting");
    let mut contents_oxy = contents.clone();
    let mut pos = 0;
    while contents_oxy.len() != 1 {
        let (num_0, num_1) = contents_oxy.iter().fold((0,0), |(z, o), s| 
            if zero_in_pos(s, pos) {
                (z + 1, o)
            } else {
                (z, o + 1)
            }
        );
        contents_oxy = contents_oxy.into_iter().filter(|s|
            if num_0 > num_1 { 
                zero_in_pos(s, pos)
            } else {
                !zero_in_pos(s, pos)
            }
        ).collect();
        pos += 1;
    }
    let oxy_gen_rating = contents_oxy.first().unwrap();

    let mut contents_co2 = contents.clone();
    let mut pos = 0;
    while contents_co2.len() != 1 {
        let (num_0, num_1) = contents_co2.iter().fold((0,0), |(z, o), s| 
            if zero_in_pos(s, pos) {
                (z + 1, o)
            } else {
                (z, o + 1)
            }
        );
        contents_co2 = contents_co2.into_iter().filter(|s|
            if num_0 <= num_1 { 
                zero_in_pos(s, pos)
            } else {
                !zero_in_pos(s, pos)
            }
        ).collect();
        pos += 1;
    }
    let co2_scrub_rating = contents_co2.first().unwrap();

    (Some(sol1), Some(bin_string_to_i32(oxy_gen_rating) * bin_string_to_i32(co2_scrub_rating)))
}

fn zero_in_pos(s: &str, n: usize) -> bool {
    s.chars().nth(n) == Some('0')
}
