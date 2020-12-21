use std::io;
use std::io::prelude::*;

type CharStack = Vec<char>;
type Num = u64;

const OPERATORS: [char; 4] = ['*', '/', '+', '-'];
const WHITESPACE: [char; 3] = [' ', '\t', '\n'];

fn main() -> io::Result<()> {
    let mut total: Num = 0;
    for line in io::stdin().lock().lines() {
        let mut stack: CharStack = Vec::new();
        let mut postfix: CharStack = Vec::new();

        let l = line?;
        println!("{}", l);

        for c in l.chars() {
            if WHITESPACE.contains(&c) {
                continue;
            }
            else if c == '(' {
                stack.push(c);
            }
            else if c == ')' {
                while stack.len() != 0 && *stack.last().unwrap() != '(' {
                    postfix.push(stack.pop().unwrap());
                }
                // Pop left parenthesis
                stack.pop();
            }
            else if OPERATORS.contains(&c) {
                if stack.len() == 0 || *stack.last().unwrap() == '(' {
                    stack.push(c);
                }
                else {
                    while stack.len() != 0 && *stack.last().unwrap() != '('
                        && precedence(&c) <= precedence(stack.last().unwrap()) {
                        postfix.push(stack.pop().unwrap());
                    }
                    stack.push(c);
                }
            }
            else {
                // Operand/Number
                postfix.push(c);
            }
        }
        while stack.len() != 0 {
            postfix.push(stack.pop().unwrap());
        }

        let mut numstack: Vec<Num> = Vec::new();
        for p in postfix {
            if OPERATORS.contains(&p) {
                let f = numstack.pop().unwrap();
                let s = numstack.pop().unwrap();
                numstack.push(operate(&p, &f, &s));
            }
            else {
                match p.to_digit(10) {
                    Some(v) => numstack.push(v.into()),
                    None => panic!("Invalid Number {}", p),
                };
            }
        }
        let result = numstack.pop().unwrap();
        println!("= {}", result);
        total += result;
    }
    println!("Total: {}", total);
    Ok(())
}

fn operate(operator: &char, f: &Num, s: &Num) -> Num {
    match operator {
        '*' => f*s,
        '/' => f/s,
        '+' => f+s,
        '-' => f-s,
        _ => panic!("Invalid operator: {}", operator),
    }
}

fn precedence(o: &char) -> Num {
    match *o {
        '+' => 2,
        _ => 1,
    }
}
