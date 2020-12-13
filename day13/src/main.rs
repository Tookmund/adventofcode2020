use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let first = get_string(lines.next());
    let earliest = match first.parse::<i32>() {
        Ok(v) => v,
        Err(e) => panic!("First line invalid: {}", e),
    };
    let second = get_string(lines.next());
    let buses = second.split(",");

    let mut minbus = [-1; 2];
    for b in buses {
        if b == "x" {
            continue;
        }
        let bn = match b.parse::<i32>() {
            Ok(v) => v,
            Err(e) => panic!("Parsing ({}): {}", b, e),
        };
        for i in earliest..i32::MAX {
            if i % bn == 0 {
                if minbus[0] == -1 || i < minbus[0] {
                    minbus[0] = i;
                    minbus[1] = bn;
                }
                break;
            }
        }
    }
    let waiting = minbus[0]-earliest;
    println!("ID: {} Waiting: {}\nMultiplied: {}", minbus[1], waiting,
             minbus[1]*waiting);
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
