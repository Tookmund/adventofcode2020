use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let first = get_string(lines.next());
    let _earliest = match first.parse::<usize>() {
        Ok(v) => v,
        Err(e) => panic!("First line invalid: {}", e),
    };
    let second = get_string(lines.next());
    let buses = second.split(",");
    let mut sched: Vec<Option<usize>> = Vec::new();

    let mut largest: usize = 0;
    for b in buses {
        if b == "x" {
            sched.push(None);
        } else {
            match b.parse::<usize>() {
                Ok(v) => {
                    sched.push(Some(v));
                    if largest < v {
                        largest = v;
                    }
                },
                Err(e) => panic!("Parsing ({}): {}", b, e),
            }
        }
    }
    let mut timestamp = 0;
    let mut inc = sched[0].unwrap();
    for j in 1..sched.len() {
        match sched[j] {
            None => continue,
            Some(v) => {
                while (timestamp + j) % v != 0 {
                    timestamp += inc;
                }
                inc *= v;
            }
        }
    }
    println!("Found: {}", timestamp);
}

fn get_string(or: Option<Result<String, io::Error>>) -> String {
    match or {
        None => panic!("No Lines?"),
        Some(r) => match r {
            Ok(v) => v,
            Err(e) => panic!("Line: {}", e),
        },
    }
}
