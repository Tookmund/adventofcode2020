use std::io;
use std::io::prelude::*;
use std::ops::Range;

#[derive(PartialEq,Debug)]
enum PState {
    Fields,
    Yours,
    Nearby,
}

type FieldVal = i32;

#[derive(PartialEq,Debug)]
struct Ticket {
    fields: Vec<FieldVal>,
}

type FieldRange = Range<FieldVal>;
type FieldRule = Vec<FieldRange>;

#[derive(PartialEq,Debug)]
struct Rule {
    name: String,
    rule: FieldRule,
}

impl Rule {
    fn new(n: &str, r: &[FieldVal]) -> Self {
        let mut s = Rule {
            rule: FieldRule::new(),
            name: String::new(),
        };
        s.name = n.to_string();
        s.rule = FieldRule::new();
        for range in r.chunks(2) {
            s.rule.push(range[0]..range[1]);
        }
        s
    }
    fn contains(&self, v: FieldVal) -> bool {
        for range in &self.rule {
            if range.contains(&v) {
                return true;
            }
        }
        false
    }
}

impl Ticket {
    fn new() -> Self {
        Ticket {
            fields: Vec::<FieldVal>::new(),
        }
    }

    fn add_field(&mut self, f: FieldVal) {
        self.fields.push(f);
    }

    fn validate(&self, rules: &[Rule]) -> FieldVal {
        let mut err: FieldVal = 0;
        for f in &self.fields {
            let mut contain = false;
            for rule in rules {
                if rule.contains(*f) {
                    contain = true;
                }
            }
            if !contain {
                err += f;
            }
        }
        err
    }
}

fn main() -> io::Result<()> {
    let mut state = PState::Fields;
    let mut rules = Vec::<Rule>::new();
    let mut your = Ticket::new();
    let mut nearby = Vec::<Ticket>::new();
    for line in io::stdin().lock().lines() {
        let line = line?;
        println!("{}", line);
        if line == "" {
            continue;
        }
        match state_change(&state, &line) {
            Some(v) => state = v,
            None => match state {
                PState::Fields => rules.push(parse_rule(&line)),
                PState::Yours => your = parse_ticket(&line),
                PState::Nearby => nearby.push(parse_ticket(&line)),
            },
        };
    }
    let mut err = your.validate(&rules);
    for t in nearby {
        err += t.validate(&rules)
    }
    println!("{}", err);
    Ok(())
}

fn state_change(state: &PState, line: &str) -> Option<PState> {
    match line {
        "your ticket:" if *state == PState::Fields => Some(PState::Yours),
        "nearby tickets:" if *state == PState::Yours => Some(PState::Nearby),
        _ => None,
    }
}

fn parse_rule(line: &str) -> Rule {
    let lvec: Vec<&str> = line.split(": ").collect();
    let name = lvec[0];
    let mut rvec = Vec::<FieldVal>::new();
    for r in lvec[1].split(" or ") {
        for b in r.split('-') {
            rvec.push(b.parse().unwrap());
        }
    }
    Rule::new(name, &rvec)
}

fn parse_ticket(line: &str) -> Ticket {
    let mut ticket = Ticket::new();
    for v in line.split(",") {
        ticket.add_field(v.parse().unwrap());
    }
    return ticket;
}
