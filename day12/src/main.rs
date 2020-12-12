use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut dir: Vec<char> = Vec::new();
    let mut val: Vec<i32> = Vec::new();

    for line in io::stdin().lock().lines() {
        let l = line?;
        let mut it = l.chars();
        match it.next() {
            Some(v) => dir.push(v),
            None => println!("NO DIRECTION?"),
        }
        let s = it.collect::<String>();
        match s.parse::<i32>() {
            Ok(v) => val.push(v),
            Err(e) => println!("WARNING: {}", e),
        }
    }
    println!("{:?}\n{:?}", dir, val);
    let mut loc = [0, 0];
    let mut wayloc = [1, 10];

    println!("SHIP: {:?}\nWAYPOINT: {:?}\n", loc, wayloc);
    for i in 0..dir.len() {
        match dir[i] {
            'F' => movetoway(&mut loc, &wayloc, &val[i]),
            'L' => rotateway(&mut wayloc, -val[i]),
            'R' => rotateway(&mut wayloc, val[i]),
            _ => shipmove(&mut wayloc, &todeg(dir[i]), &val[i]),
        }
        println!("{}{}\nSHIP: {:?}\nWAYPOINT: {:?}\n", dir[i], val[i], loc, wayloc);
    }
    println!("{:?}\n{}", loc, loc[0].abs()+loc[1].abs());
    Ok(())
}

fn movetoway(loc: &mut [i32; 2], wayloc: &[i32; 2], val: &i32) {
    for _ in 0..*val {
        loc[0] += wayloc[0];
        loc[1] += wayloc[1];
    }
}

fn rotateway(wayloc: &mut [i32; 2], val: i32) {
    match val {
        90 | -270 => {
            let temp = wayloc[0];
            wayloc[0] = -wayloc[1];
            wayloc[1] = temp;
        },
        180 | -180 => {
            for _ in 0..2 {
                rotateway(wayloc, 90);
            }
        },
        270 | -90 => {
            rotateway(wayloc, 180);
            rotateway(wayloc, 90);
        },
        0 => return,
        _ => panic!("INVALID ROTATION: {}", val),
    }
}

fn shipmove(loc: &mut [i32; 2], facing: &i32, val: &i32) {
    match facing {
        0 => loc[0] += val,
        180 => loc[0] -= val,
        90 => loc[1] += val,
        270 => loc[1] -= val,
        _ => panic!("INVALID FACING: {}", facing),
    }
}

fn todeg(d: char) -> i32 {
    match d {
        'N' => 0,
        'E' => 90,
        'S' => 180,
        'W' => 270,
        _ => panic!("INVALID DIRECTION: {}", d),
    }
}
