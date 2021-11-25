use std::io;
use std::io::prelude::*;
use std::ops::Range;

#[derive(PartialEq,Debug)]
enum PState {
    FIELDS,
    YOURS,
    NEARBY,
}

type FieldVal = i32;

#[derive(PartialEq,Debug)]
struct Ticket {
    fields: Vec<FieldVal>,
}

type FieldRange = Range<FieldVal>;
type FieldRule = Vec<FieldRange>;

#[derive(PartialEq,Debug)]
struct Rules {
    rules: Vec<FieldRule>,
    names: Vec<String>,
}

impl Rules {
    fn new() -> Self {
        Rules {
            rules: Vec::<FieldRule>::new(),
            names: Vec::<String>::new(),
        }
    }

    fn addrule(&mut self, n: &str, r: &[FieldVal]) {
        self.names.push(n.to_string());
        let mut rv = FieldRule::new();
        for range in r.chunks(2) {
            rv.push(range[0]..range[1]);
        }
        self.rules.push(rv);
    }
}

impl Ticket {
    fn new() -> Self {
        Ticket {
            fields: Vec::<FieldVal>::new(),
        }
    }

    fn addfield(&mut self, f: FieldVal) {
        self.fields.push(f);
    }

    fn validate(&self, rules: Rules) -> bool {
        // TODO
        true
    }
}

fn main() -> io::Result<()> {
    let mut state = PState::FIELDS;
    let mut rules = Rules::new();
    let mut your: Ticket;
    let mut nearby = Vec::<Ticket>::new();
    for line in io::stdin().lock().lines() {
        let line = line?;
        println!("{}", line);
        if line == "" {
            continue;
        }
        match stateChange(&state, &line) {
            Some(v) => state = v,
            None => match state {
                PState::FIELDS => parserules(&line, &mut rules),
                PState::YOURS => your = parseticket(&line),
                PState::NEARBY => nearby.push(parseticket(&line)),
            },
        };
    }
    Ok(())
}

fn stateChange(state: &PState, line: &str) -> Option<PState> {
    match line {
        "your ticket:" if *state == PState::FIELDS => Some(PState::YOURS),
        "nearby tickets:" if *state == PState::YOURS => Some(PState::NEARBY),
        _ => None,
    }
}

fn parserules(line: &str, rules: &mut Rules) {
    let lvec: Vec<&str> = line.split(":").collect();
    let name = lvec[0];
    let mut rvec = Vec::<FieldVal>::new();
    for r in lvec[1].split(" or ") {
        for b in r.split("-") {
            rvec.push(b.parse().unwrap());
        }
    }
    rules.addrule(name, &rvec);
}

fn parseticket(line: &str) -> Ticket {
    let mut ticket = Ticket::new();
    for v in line.split(",") {
        ticket.addfield(v.parse().unwrap());
    }
    return ticket;
}
