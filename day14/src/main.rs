use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
use std::fmt;

fn main() -> io::Result<()> {
    let mut mem = HashMap::new();
    let mut mask = String::from("UNSET");
    for line in io::stdin().lock().lines() {
        let l = line?;
        let split = l.split_whitespace().collect::<Vec<&str>>();
        if split[0] == "mask" {
            mask = String::from(split[2]);
        } else {
            let newmem = strtoint(&split[0][4..split[0].len()-1]);
            let newnum = strtoint(split[2]);
            let newbin = binform(newnum);
            println!("Mem: {}", newmem);
            println!("value:\t{}\t(decimal {})", newbin, newnum);
            println!("mask:\t{}", mask);
            let resultbin = getresultnum(&mask, &newbin);
            let resultnum = intform(&resultbin);
            println!("result:\t{}\t(decimal {})\n", resultbin, resultnum);
            mem.insert(newmem, resultnum);
        }
    }
    let mut total = 0;
    for (_, v) in mem {
        total += v;
    }
    println!("Total: {}", total);
    Ok(())
}

fn strtoint(b: &str) -> u64 {
    match b.parse::<u64>() {
        Ok(v) => v,
        Err(e) => panic!("Failed to parse ({}): {}", b, e),
    }
}

fn binform<T: fmt::Binary>(b: T) -> String {
    format!("{:0>36b}", b)
}

fn intform(b: &str) -> u64 {
    u64::from_str_radix(b, 2).unwrap()
}

fn getresultnum(mask: &str, newnum: &str) -> String {
    let mut ret: String = "".to_owned();
    let maskb = mask.as_bytes();
    for i in 0..maskb.len() {
        if maskb[i] == b'X' {
            ret.push_str(&newnum[i..i+1]);
        } else {
            ret.push_str(&mask[i..i+1]);
        }
    }
    return ret;
}
