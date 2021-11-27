use std::io;
use std::io::prelude::*;

type Num = i64;

const INITSUB: Num = 7;

fn main() -> io::Result<()>{
    let lines = io::stdin().lock().lines().map(|i| i.unwrap()).collect::<Vec<String>>();
    let card_pub = lines[0].parse::<Num>().unwrap();
    let card_loop = transform_goal(INITSUB, card_pub);
    println!("Card Pub: {}\nCard Loop: {}", card_pub, card_loop);
    let door_pub = lines[1].parse::<Num>().unwrap();
    let door_loop = transform_goal(INITSUB, door_pub);
    println!("Door Pub: {}\nDoor Loop: {}", door_pub, door_loop);
    let enc = transform(door_pub, card_loop);
    println!("Encryption Key: {}", enc);
    Ok(())
}

fn transform_goal(subject: Num, goal: Num) -> usize {
    let mut val = 1;
    let mut i = 0;
    loop {
        i += 1;
        val *= subject;
        val %= 20201227;
        if val == goal {
            return i;
        }
    }
}

fn transform(subject: Num, loop_size: usize) -> Num {
    let mut val = 1;
    for _ in 0..loop_size {
        val *= subject;
        val %= 20201227;
    }
    val
}
