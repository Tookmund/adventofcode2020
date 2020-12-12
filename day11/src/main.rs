use std::io;
use std::io::prelude::*;
use std::fmt::Display;


const FILLED: char = '#';
const EMPTY: char = 'L';

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
            if seats[r][c] == EMPTY || seats[r][c] == FILLED {
                let filled = check_adjacent(seats, r, c);
                if seats[r][c] == EMPTY && filled == 0 {
                    newseats[r][c] = FILLED;
                } else if seats[r][c] == FILLED && filled >= 5 {
                    newseats[r][c] = EMPTY;
                }
            }
        }
    }
    return newseats;
}

fn check_adjacent(seats: &Vec<Vec<char>>, r: usize, c: usize) -> i32 {
    let mut ret: i32 = 0;
    // Diagonals
    let delta: [[i32; 2]; 8] = [
        // Up
        [-1, 0],
        // Down
        [1, 0],
        // Left
        [0, -1],
        // Right
        [0, 1],
        // Up-Left
        [-1, -1],
        // Up-Right
        [-1, 1],
        // Down-Left
        [1, -1],
        // Down-Right
        [1, 1],
    ];
    for d in delta.iter() {
        let mut ir = r;
        let mut ic = c;
        loop {
            match usize_add(ir, d[0]) {
                Some(v) => ir = v,
                None => break,
            }
            if ir >= seats.len() {
                break;
            }
            match usize_add(ic, d[1]) {
                Some(v) => ic = v,
                None => break,
            }
            if ic >= seats[ir].len() {
                break;
            }
            if seats[ir][ic] == EMPTY {
                break;
            } else if seats[ir][ic] == FILLED {
                ret += 1;
                break
            }
        }
    }
    return ret;
}

fn usize_add(u: usize, i: i32) -> Option<usize> {
    if i.is_negative() {
        return u.checked_sub(i.wrapping_abs() as u32 as usize);
    } else {
        return u.checked_add(i as usize);
    }
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
            if *s == FILLED {
                ret += 1;
            }
        }
    }
    return ret;
}
