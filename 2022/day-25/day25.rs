use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref SNAFU_TO_DEC: HashMap<char, i64> =
        [('1', 1), ('2', 2), ('0', 0), ('-', -1), ('=', -2)].into();
}

fn snafu_to_dec(snafu: &str) -> i64 {
    let snafu_len = snafu.len();
    snafu
        .chars()
        .enumerate()
        .map(|(i, c)| SNAFU_TO_DEC[&c] * (5_i64.pow((snafu_len - 1 - i) as u32)))
        .sum()
}

fn dec_to_snafu(num: i64) -> String {
    let num_snafu_digits = (1..)
        .map(|i| (0..i).map(|j| 2 * 5_i64.pow(j as u32)).sum::<i64>())
        .take_while(|snafup| snafup < &num)
        .count()
        + 1;
    let mut snafu = String::from_iter((0..num_snafu_digits).map(|_| '0'));
    (0..num_snafu_digits).for_each(|i| {
        snafu = SNAFU_TO_DEC
            .keys()
            .map(|c| {
                let x = String::from(*c);
                format!("{}{}{}", &snafu[0..i], x, &snafu[i + 1..snafu.len()])
            })
            .min_by_key(|snaf| (num - snafu_to_dec(snaf)).abs())
            .unwrap();
    });
    snafu
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("Failed to read input!");
    let sum = dec_to_snafu(contents.lines().map(snafu_to_dec).sum::<i64>());
    println!("Part 1: {}", sum);
}
