use std::collections::hash_map::Entry::Occupied;
use std::collections::hash_map::Entry::Vacant;
use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

#[derive(Debug)]
struct Num {
    times: i32,
    when: i32,
}
const MAXTURN: i32 = 30000000;

fn main() -> io::Result<()> {
    for line in io::stdin().lock().lines() {
        let mut nums = HashMap::new();
        let mut turn = 1;
        let mut last = 0;
        let l = line?;
        for n in l.split(",") {
            let s = match n.parse::<i32>() {
                Ok(v) => v,
                Err(e) => panic!("Failed to parse ({}) into int: {}", n, e),
            };
            nums.insert(
                s,
                Num {
                    times: 1,
                    when: turn,
                },
            );
            //println!("Turn {}: {}", turn, s);
            turn += 1;
            last = s;
        }
        let mut say = 0;
        for i in turn..=MAXTURN {
            match nums.entry(last) {
                Vacant(v) => panic!("{} missing from nums?", v.key()),
                Occupied(mut o) => {
                    let mut v = o.get_mut();
                    if v.times != 1 {
                        say = (i - 1) - v.when;
                    } else {
                        say = 0;
                    }
                    v.when = i - 1;
                }
            }
            match nums.entry(say) {
                Vacant(v) => {
                    v.insert(Num { times: 1, when: i });
                }
                Occupied(mut o) => {
                    let mut v = o.get_mut();
                    v.times += 1;
                }
            }
            //println!("Turn {}: {}", i, say);
            last = say;
        }
        println!("{} Turn {}: {}", l, MAXTURN, say);
    }

    Ok(())
}
