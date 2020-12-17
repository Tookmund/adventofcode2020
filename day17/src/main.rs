use std::io;
use std::io::prelude::*;
use std::collections::HashSet;

type Coord = (i64, i64, i64);
type ConwayCubes = HashSet<Coord>;

const CHANGE: i64 = 2;

fn main() -> io::Result<()> {
    let mut map: ConwayCubes = HashSet::new();
    let mut min: Coord = (-CHANGE, -CHANGE, -CHANGE);
    let mut max: Coord = (0, 0, 1);
    let mut x = 0;
    let mut y = 0;


    for line in io::stdin().lock().lines() {
        y = 0;
        for c in line?.chars() {
            match c {
                '#' => {
                    map.insert((x,y,0));
                    ()
                },
                '.' => (),
                _ =>  panic!("Invalid char ({}) in input!", c),
            }
           y += 1;
        }
        x += 1;
    }
    max.0 = x+CHANGE;
    max.1 = y+CHANGE;

    for _ in 0..6 {
        let mut newmap = map.clone();
        for x in min.0..=max.0 {
            for y in min.1..=max.1 {
                for z in min.2..=max.2 {
                    let neighbors = get_neighbors(&map, &(x, y, z));
                    let alive = map.contains(&(x,y,z));
                    if !alive && neighbors == 3 {
                        newmap.insert((x, y, z));
                    }
                    else if alive && !(neighbors == 2 || neighbors == 3) {
                        newmap.remove(&(x, y, z));
                    }
                }
            }
        }
        map = newmap;
        min.0 -= CHANGE;
        min.1 -= CHANGE;
        min.2 -= CHANGE;

        max.0 += CHANGE;
        max.1 += CHANGE;
        max.2 += CHANGE;
        println!("ACTIVE: {}", map.len());
    }
    Ok(())
}

fn get_neighbors(map: &ConwayCubes, c: &Coord) -> i32 {
    let mut ret = 0;
    for x in c.0-1..=c.0+1 {
        for y in c.1-1..=c.1+1 {
            for z in c.2-1..=c.2+1 {
                if *c != (x,y,z) && map.contains(&(x, y, z)) {
                    ret += 1;
                }
            }
        }
    }
    return ret;
}
