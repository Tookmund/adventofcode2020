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
    let mut facing: i32 = 90;

    for i in 0..dir.len() {
        match dir[i] {
            'F' => shipmove(&mut loc, &facing, &val[i]),
            'L'  => newfacing(&mut facing, -val[i]),
            'R'  => newfacing(&mut facing, val[i]),
            _ => shipmove(&mut loc, &todeg(dir[i]), &val[i]),
        }
    }
    println!("{:?}\n{}", loc, loc[0].abs()+loc[1].abs());
    Ok(())
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

fn newfacing(facing: &mut i32, val: i32) {
    *facing = (*facing+val) % 360;
    *facing = match *facing {
        -90 => 270,
        -180 => 180,
        -270 => 90,
        _ => *facing
    }
}
