use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::cmp::Ordering;

pub fn day2() -> std::io::Result<()> {
    let mut file = File::open("day2.txt")?;
    let buff = BufReader::new(file);
    let mut count = 0;

    for line in buff.lines() {
        
        let line = line?;
        let v: Vec<u32> = line
            .split_whitespace()
            .filter_map(|num| num.parse::<u32>().ok())
            .collect();
        //println!("return={}",validate(&v));
        count += validate(&v);
    }
    println!("count={}", count);
    Ok(())
}


pub fn validate(v: &Vec<u32>) -> u32 {
    let mut inc = false;
    let mut order = v[0] < v[1];
    //println!("validating:{:?}", &v);

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
