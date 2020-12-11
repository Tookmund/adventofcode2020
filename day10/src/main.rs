use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut joltages: Vec<i32> = Vec::new();
    for line in io::stdin().lock().lines() {
        joltages.push(line?.parse::<i32>().unwrap());
    }
    // The outlet is 0 jolts
    joltages.push(0);
    joltages.sort();
    // The computer is 3+largest
    joltages.push(joltages.last().unwrap()+3);
    println!("{:?}", joltages);
    let mut prev: i32 = -1;
    let mut ones: i32 = 0;
    let mut threes: i32 = 0;
    for j in joltages.iter() {
        if prev >= 0 {
            match j-prev {
                1 => ones += 1,
                3 => threes += 1,
                _ => panic!("INVALID"),
            }
        }
        prev = *j;
    }
    println!("Ones: {}\nThrees: {}\nMultiplied:{}\n", ones, threes, ones*threes);
    return Ok(())
}
