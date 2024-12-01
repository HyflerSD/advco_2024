use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Vals {
    left: Vec<u32>,
    right: Vec<u32>,
}

#[derive(Debug)]
pub struct Valsm {
    left: HashMap<u32, u32>,
}

pub fn day1() -> std::io::Result<()> {
    let mut file = File::open("day1.txt")?;
    let buff = BufReader::new(file);
    let mut nums: Vals = Vals::new();
    for line in buff.lines() {
        let line = line?;
        let v: Vec<u32> = line
            .split_whitespace()
            .filter_map(|num| num.parse::<u32>().ok())
            .collect();
        nums.left.push(v[0]);
        nums.right.push(v[1]);
    }
    println!("{}",nums.sum());
    println!("{}",nums._sum());

    Ok(())

}

pub fn day1p2() -> std::io::Result<()> {
    let mut file = File::open("day1.txt")?;
    let buff = BufReader::new(file);
    let mut nums: Valsm = Valsm::new();
    for line in buff.lines() {
        let line = line?;
        let v: Vec<u32> = line
            .split_whitespace()
            .filter_map(|num| num.parse::<u32>().ok())
            .collect();

        match nums.left.get(&v[0]) {
            Some(count) => {
                nums.left.insert(v[0], count +1);
            }
            None => {
                nums.left.insert(v[0], 1);
            }
        }

        match nums.left.get(&v[1]) {
            Some(count) => {
                nums.left.insert(v[1], count +1);
            }
            None => {
                nums.left.insert(v[1], 1);
            }
        }
    }

    println!("{}", nums.sum());

    //println!("{:#?}", nums);
    Ok(())
}

impl Valsm {
    pub fn new() -> Self {
        Valsm {
            left: HashMap::new(),
        }
    }

    pub fn sum(&mut self) -> u32 {
        let mut tot: u32 = 0;
        for (key, val) in &mut self.left {
            if *val >= 2 {
                tot += *key * *val;
            }
        }
        tot
    }

}

impl Vals {
    pub fn new() -> Self {
        Vals {
            left: Vec::new(),
            right: Vec::new(),
        }
    }

    pub fn sort(&mut self) {
        self.left.sort();
        self.right.sort();
    }

    pub fn sum(&mut self) -> u32 {
        self.sort();
        let mut total: u32 = 0;
        for i in 0..self.left.len() {
            total += self.left[i].abs_diff(self.right[i]);
        }
        total
    }

    pub fn _sum(&mut self) -> u32 {
        self.sort();
        let mut total: u32 = self.left.iter().zip(self.right.iter())
            .map(|(a, b)| a.abs_diff(*b))
            .sum();
        total
    }
}
