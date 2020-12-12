use std::io;
use std::io::prelude::*;
use std::fmt::Display;

fn main() -> io::Result<()> {
    let mut seats: Vec<Vec<char>> = Vec::new();
    for line in io::stdin().lock().lines() {
        seats.push(line?.chars().collect());
    }
    let mut newseats: Vec<Vec<char>>;
    loop {
        print2dvec(&seats);
        newseats = step(&seats);
        if compare_seats(&seats, &newseats) {
            break;
        }
        seats = newseats;
        println!("{}", seats.iter().map(|_| "-").collect::<String>());
    }
    println!("Occupied Seats in Stable State: {}", occuiped(&seats));
    return Ok(());
}

fn print2dvec<T: Display>(vec: &Vec<Vec<T>>) {
    for v in vec.iter() {
        for vv in v.iter() {
            print!("{}", vv);
        }
        print!("\n");
    }
}

fn step(seats: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut newseats: Vec<Vec<char>> = Vec::with_capacity(seats.len());
    for v in seats.iter() {
        newseats.push(v.clone());
    }
    for r in 0..seats.len() {
        for c in 0..seats[r].len() {
            if seats[r][c] == 'L' || seats[r][c] == '#' {
                let filled = check_adjacent(seats, r, c);
                if seats[r][c] == 'L' && filled == 0 {
                    newseats[r][c] = '#';
                } else if seats[r][c] == '#' && filled >= 4 {
                    newseats[r][c] = 'L';
                }
            }
        }
    }
    return newseats;
}

fn check_adjacent(seats: &Vec<Vec<char>>, r: usize, c: usize) -> i32 {
    let mut ret: i32 = 0;

    let sr: usize;
    if r != 0 {
        sr = r-1;
    } else {
        sr = r
    }
    let er: usize;
    if r != seats.len()-1 {
        er = r+1
    } else {
        er = r
    }
    let sc: usize;
    if c != 0 {
        sc = c-1;
    } else {
        sc = c;
    }
    let ec: usize;
    if c != seats[r].len()-1 {
        ec = c+1;
    } else {
        ec = c;
    }

    for ir in sr..=er {
        for ic in sc..=ec {
            if r == ir && c == ic {
                continue;
            } else if seats[ir][ic] == '#' {
                ret += 1;
            }
        }
    }
    return ret;
}

fn compare_seats(a: &Vec<Vec<char>>, b: &Vec<Vec<char>>) -> bool {
    for i in 0..a.len() {
        if a[i].iter().zip(b[i].iter()).filter(|&(k, l)| k == l).count() != a[i].len() {
            return false
        }
    }
    return true
}

fn occuiped(seats: &Vec<Vec<char>>) -> usize {
    let mut ret: usize = 0;
    for r in seats.iter() {
        for s in r.iter() {
            if *s == '#' {
                ret += 1;
            }
        }
    }
    return ret;
}
