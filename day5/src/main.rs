use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    println!("Hello, world!");
    let mut seats = [ [false; 8]; 128];
    let mut maxseatid = 0;

    for line in io::stdin().lock().lines() {
        let mut minrow = 0;
        let mut maxrow = 127;
        let mut mincol = 0;
        let mut maxcol = 7;
        let mut currow: Option<usize> = None;
        let mut curcol: Option<usize> = None;

        for c in line?.chars() {
            if maxrow-minrow == 1  && currow == None {
                currow = Some(match c {
                    'F' => minrow,
                    'B' => maxrow,
                    _ => panic!("INVALID ROW-DETERMINING CHAR: {}", c),
                });
                continue;
            }
            if maxcol-mincol == 1 && curcol == None {
                curcol = Some(match c {
                    'L' => mincol,
                    'R' => maxcol,
                    _ => panic!("INVALID COL-DETERMINING CHAR: {}", c),
                });
                continue;
            }
            match c {
                'F' => maxrow -= (maxrow-minrow+1)/2,
                'B' => minrow += (maxrow-minrow+1)/2,
                'L' => maxcol -= (maxcol-mincol+1)/2,
                'R' => mincol += (maxcol-mincol+1)/2,
                _ => panic!("INVALID CHAR: {}", c),
            }
        }
        let r = currow.unwrap();
        let c = curcol.unwrap();
        seats[r][c] = true;
        let id = seat_id(r, c);
        println!("({}, {}), ID: {}", r, c, id);
        if id > maxseatid {
            maxseatid = id;
        }
    }
    println!("Max Seat ID: {}", maxseatid);
    let mut firstseat = true;
    for r in 0..seats.len() {
        for c in 0..seats[r].len() {
            if seats[r][c] == true && firstseat {
                firstseat = false;
            } else if !firstseat && seats[r][c] == false {
                println!("FOUND SEAT: ({}, {}) ID: {}", r, c, seat_id(r,c));
                return Ok(());
            }
        }
    }
    return Ok(());
}

fn seat_id(r: usize, c: usize) -> usize {
    r * 8 + c
}
