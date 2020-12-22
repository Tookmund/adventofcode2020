use std::io;
use std::io::prelude::*;
use std::collections::VecDeque;

type Card = u32;
type Deck = VecDeque<Card>;

const NUMPLAYERS: usize = 2;

fn main() -> io::Result<()>{
    let mut players: [Deck; NUMPLAYERS] = Default::default();
    let mut totalcards = 0;

    let mut p = 0;
    for resline in io::stdin().lock().lines() {
        let line = resline?;
        if line == "" {
            p += 1;
        }
        else if line.split_whitespace().next().unwrap() != "Player" {
            players[p].push_back(match line.parse::<Card>() {
                Ok(v) => {
                    totalcards += 1;
                    v
                },
                Err(e) => panic!("Invalid Number ({}): {}", line, e),
            });

        }
    }
    let mut round = 1;
    let mut winneropt: Option<usize>;
    loop {
        winneropt = deck_all_cards(&players, &totalcards);
        if winneropt != None {
            break;
        }
        println!("-- Round {} --", round);
        for i in 0..players.len() {
            println!("Player {}'s deck: {}", i+1, print_deck(&players[i]));
        }
        let mut cardsplayed: [Card; NUMPLAYERS] = Default::default();
        for i in 0..players.len() {
            print!("Player {} plays", i+1);
            if players[i].len() > 0 {
                cardsplayed[i] = players[i].pop_front().unwrap();
                println!(": {}", cardsplayed[i]);
            }
            else {
                println!(" no card!");
            }
        }
        let mut max = 0;
        for i in 1..players.len() {
            if cardsplayed[i] > cardsplayed[max] {
                max = i;
            }
        }
        println!("Player {} wins the round!\n", max+1);
        // Append in highest to lowest order
        cardsplayed.sort();
        for i in (0..cardsplayed.len()).rev() {
            players[max].push_back(cardsplayed[i]);
        }
        round += 1;
    }
    let winner = winneropt.unwrap();
    println!("\n== Post-game results ==");
    for i in 0..players.len() {
        println!("Player {}'s deck: {}", i+1, print_deck(&players[i]));
    }
    println!("Winning Player {}'s Score: {}", winner+1,
             score(&mut players[winner]));
    Ok(())
}

fn deck_all_cards(players: &[Deck], total: &usize) -> Option<usize> {
    for i in 0..players.len() {
        if players[i].len() == *total {
            return Some(i);
        }
    }
    return None;
}

fn score(player: &mut Deck) -> Card {
    let mut total = 0;
    let mut multi = 1;

    while let Some(cur) = player.pop_back() {
        total += cur * multi;
        multi += 1;
    }
    return total;
}

fn print_deck(deck: &Deck) -> String {
    let mut ret: String = String::new();
    for i in 0..deck.len() {
        ret = format!("{}{}", ret, deck[i]);
        if i != deck.len()-1 {
            ret += ", ";
        }
    }
    ret
}
