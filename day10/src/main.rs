use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut joltages: Vec<i32> = Vec::new();
    for line in io::stdin().lock().lines() {
        match line?.parse::<i32>() {
            Ok(i) => joltages.push(i),
            Err(e) => println!("WARNING: {}\n", e),
        }
    }
    // The outlet is 0 jolts
    joltages.push(0);
    joltages.sort();
    // The computer is 3+largest
    joltages.push(joltages.last().unwrap()+3);
    println!("{:?}", joltages);

    let mut ways: Vec<i64> = vec![0; joltages.len()];
    ways[0] = 1;

    for i in 0..joltages.len() {
        for j in i+1..i+4 {
            if j >= joltages.len() {
                break;
            } else if joltages[j]-joltages[i] <= 3 {
                ways[j] += ways[i];
            } else {
                break;
            }
        }
    }
    println!("{}", ways[joltages.len()-1]);
    return Ok(())
}
