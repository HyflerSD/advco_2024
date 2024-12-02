use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::cmp::Ordering;

pub fn day2() -> std::io::Result<()> {
    let mut file = File::open("day2.txt")?;
    let mut buff = String::new();
    file.read_to_string(&mut buff)?;

    println!("count={}", part2(&buff[..]));
    Ok(())
}


fn is_ok(nums: &[i32]) -> bool {
    let sign = nums[0] < nums[1];
    nums[..]
        .windows(2)
        .all(|w| (w[0] - w[1]).abs() > 0 && (w[0] - w[1]).abs() < 4 && (w[0] < w[1]) == sign)
}

fn part2(s: &str) -> usize {
    s.lines()
        .filter(|l| {
            let nums = l
                .split_whitespace()
                .map(|w| w.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            is_ok(&nums[..])
                || (0..nums.len()).any(|i| is_ok(&[&nums[..i], &nums[i + 1..]].concat()))
        })
        .count()
}

pub fn part1(v: &Vec<u32>) -> u32 {
    let mut inc = false;
    let mut order = v[0] < v[1];

    for (i, num) in v.iter().enumerate(){
        if i > 0 {
            if num.abs_diff(v[i-1]) > 3 {
                return 0 as u32;
            }

            match num.cmp(&v[i-1]) {
                Ordering::Less => {
                    if order {
                        return 0 as u32;
                    }
                }
                Ordering::Greater => {
                    if !order {
                        return 0 as u32;
                    }
                } 
                Ordering::Equal => {
                   return  0 as u32;
                }
            }
        }
    }
    1 as u32
}
