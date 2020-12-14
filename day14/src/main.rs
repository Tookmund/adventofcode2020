use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
use std::fmt;
use std::str;

fn main() -> io::Result<()> {
    let mut mem: HashMap<u64, u64> = HashMap::new();
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
            let newmembin = binform(newmem);
            println!("Mem:\t{}\t(decimal {})", newmembin, newmem);
            println!("value:\t{}\t(decimal {})", newbin, newnum);
            println!("mask:\t{}", mask);
            let mut mb = mask.as_bytes().to_owned();
            part2(0, &mut mem, &mut mb, &newmembin.as_bytes(), &newnum);
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

fn _part1(mask: &str, newnum: &str) -> String {
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

fn part2(curmaski: usize, mem: &mut HashMap<u64, u64>, mask: &mut [u8],
         addr: &[u8], val: &u64) {
    let mut writeaddr = addr.to_owned();
    for i in curmaski..mask.len() {
        if mask[i] == b'1' {
            writeaddr[i] = b'1';
        } else if mask[i] == b'X' {
            writeaddr[i] = b'0';
            part2(i+1, mem, mask, &writeaddr, val);
            writeaddr[i] = b'1';
            part2(i+1, mem, mask, &writeaddr, val);
        }
    }
    let wstr = match str::from_utf8(&writeaddr) {
        Ok(v) => v,
        Err(e) => panic!("Failed to parse ({:?}) into str: {}", writeaddr, e),
    };
    let wint = intform(&wstr);
    println!("{}\t(decimal {}) = {}", wstr, wint, *val);
    mem.insert(wint, *val);
}
