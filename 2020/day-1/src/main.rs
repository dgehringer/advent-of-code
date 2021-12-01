use std::fs::read_to_string;
use std::path::Path;
use itertools::Itertools;

fn main() {
    let contents = read_to_string(Path::new("/home/dominik/tmp/input.txt")).unwrap();
    let numbers: Vec<i64> = contents.as_str().lines().map(|line| line.parse::<i64>().unwrap()).sorted().collect();
    
    'outer: for (i, num1) in numbers.iter().enumerate() {
        // we can skip the the first i numbers since we want to avoid double counting
        'inner: for num2 in numbers.iter().skip(i) {
            match num1 + num2 {
                2020 => {
                    println!("{}", num1 * num2); 
                    break 'outer;
                },
                // we are between the sum of the first to numbers and 2020, we have to continue our search
                0..=2019 => continue,
                //numbers are larger than 2020 we do not have to check any more, break the inner loop
                _ => break 'inner
            }
        }
    }

    'whole: for (i, num1) in numbers.iter().enumerate() {
        'outer: for (j, num2) in numbers.iter().skip(i).enumerate() {
            // we do not have such a small number in out reportoire to fullfil the condition
            // thus we can already test the next outer number
            if 2020 - num1 - num2 < numbers[0] { break 'outer; }
            let outer_sum = num1 + num2;
            'inner: for num3 in numbers.iter().skip(j) {
                // same game as before
                match outer_sum + num3 {
                    2020 => {
                        println!("{}", num1 * num2 * num3); 
                        break 'whole;
                    },
                    0..=2019 => continue,
                    _ => break 'inner
                }
            }
        }
    }
}
