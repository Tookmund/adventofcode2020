use std::io;
use std::io::prelude::*;
use std::collections::VecDeque;

const NUMPLAYERS: usize = 2;

type Card = usize;
type Deck = VecDeque<Card>;
type Players = [Deck; NUMPLAYERS];
type CardsPlayed = [Card; NUMPLAYERS];


fn main() -> io::Result<()>{
    let mut players: Players = Default::default();
    let mut total = 0;

    let mut p = 0;
    for resline in io::stdin().lock().lines() {
        let line = resline?;
        if line == "" {
            p += 1;
        }
        else if line.split_whitespace().next().unwrap() != "Player" {
            players[p].push_back(match line.parse::<Card>() {
                Ok(v) => {
                    total += 1;
                    v
                },
                Err(e) => panic!("Invalid Number ({}): {}", line, e),
            });

        }
    }
    play_game(&mut players, &total, 1);
    Ok(())
}


fn play_game(players: &mut Players, total: &usize, game: usize) -> (usize, usize) {
    let mut round = 1;
    let mut winneropt: Option<usize>;
    let mut memory: Vec<String> = Vec::new();
    let mut newgame = game;

    println!("=== Game {} ===\n", game);
    loop {
        winneropt = deck_all_cards(players, total);
        if winneropt != None {
            break;
        }
        let state = player_state(players);
        if memory.contains(&state) {
            post_game(players, 0, game);
            return (0, newgame);
        }
        else {
            memory.push(state);
        }
        println!("-- Round {} (Game {}) --", round, game);
        for i in 0..players.len() {
            println!("Player {}'s deck: {}", i+1, print_deck(&players[i]));
        }
        let mut cardsplayed: CardsPlayed = Default::default();
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
        if remaining_equals_drawn(&players, &cardsplayed) {
            let mut newplayers: Players = Default::default();
            for i in 0..players.len() {
                for j in 0..cardsplayed[i] {
                    newplayers[i].push_back(players[i][j]);
                }
            }
            let mut newtotal = 0;
            for p in newplayers.iter() {
                for _ in p {
                    newtotal += 1;
                }
            }
            println!("Playing a sub-game to determine the winner...\n");
            newgame += 1;
            let gameret = play_game(&mut newplayers, &newtotal, newgame);
            max = gameret.0;
            newgame = gameret.1;
            println!("\n...anyway, back to game {}.", game);
        }
        println!("Player {} wins round {} of game {}!\n", max+1, round, game);
        // Always put the winning player's card first
        cardsplayed.swap(0, max);
        for i in 0..cardsplayed.len() {
            players[max].push_back(cardsplayed[i]);
        }
        round += 1;
    }
    let winner = winneropt.unwrap();
    post_game(players, winner, game);
    (winner, newgame)
}

fn post_game(players: &mut Players, winner: usize, game: usize) {
    println!("The winner of game {} is player {}!", game, winner+1);
    if game != 1 {
        return;
    }
    println!("\n\n== Post-game results ==");
    for i in 0..players.len() {
        println!("Player {}'s deck: {}", i+1, print_deck(&players[i]));
    }
    println!("Winning Player {}'s Score: {}", winner+1,
             score(&mut players[winner]));
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

fn player_state(players: &Players) -> String {
    let mut ret: String = String::new();
    for p in players {
        ret = format!("{}|{}", ret, print_deck(&p));
    }
    ret
}

fn remaining_equals_drawn(players: &Players, cardsplayed: &CardsPlayed) -> bool {
    for i in 0..players.len() {
        if players[i].len() < cardsplayed[i] {
            return false;
        }
    }
    return true;
}
