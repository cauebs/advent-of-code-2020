enum Expr {
    BinOp(char, Box<Expr>, Box<Expr>),
    Literal(u64),
}

fn parse(input: &str, precedence: impl Fn(char) -> u8) -> Expr {
    let mut output = Vec::new();
    let mut ops_stack = Vec::new();

    let pop_two = |stack: &mut Vec<_>| {
        let lhs = Box::new(stack.pop().unwrap());
        let rhs = Box::new(stack.pop().unwrap());
        (lhs, rhs)
    };

    for symbol in input.chars() {
        if symbol.is_whitespace() {
            continue;
        }

        match symbol {
            '0'..='9' => {
                let number = symbol.to_digit(10).unwrap() as u64;
                output.push(Expr::Literal(number));
            }

            '+' | '*' => {
                while let Some(&op) = ops_stack.last() {
                    if precedence(op) >= precedence(symbol) {
                        let (lhs, rhs) = pop_two(&mut output);
                        output.push(Expr::BinOp(op, lhs, rhs));
                        ops_stack.pop();
                    } else {
                        break;
                    }
                }
                ops_stack.push(symbol);
            }

            '(' => ops_stack.push(symbol),

            ')' => loop {
                match ops_stack.pop() {
                    Some(op @ '+') | Some(op @ '*') => {
                        let (lhs, rhs) = pop_two(&mut output);
                        output.push(Expr::BinOp(op, lhs, rhs));
                    }
                    Some('(') => break,
                    _ => panic!(),
                }
            },

            _ => panic!(),
        }
    }

    while let Some(op) = ops_stack.pop() {
        let (lhs, rhs) = pop_two(&mut output);
        output.push(Expr::BinOp(op, lhs, rhs));
    }

    assert_eq!(output.len(), 1);
    output.pop().unwrap()
}

fn eval(expr: Expr) -> u64 {
    match expr {
        Expr::BinOp('+', lhs, rhs) => eval(*lhs) + eval(*rhs),
        Expr::BinOp('*', lhs, rhs) => eval(*lhs) * eval(*rhs),
        Expr::Literal(n) => n,
        _ => panic!(),
    }
}

fn results_sum(input: &str, precedence: impl Fn(char) -> u8) -> u64 {
    input
        .lines()
        .map(|line| parse(line, &precedence))
        .map(eval)
        .sum::<u64>()
}

fn main() {
    let input = include_str!("../../inputs/day18.txt");

    println!(
        "{}",
        results_sum(input, |op| match op {
            '+' => 1,
            '*' => 1,
            _ => 0,
        })
    );

    println!(
        "{}",
        results_sum(input, |op| match op {
            '+' => 2,
            '*' => 1,
            _ => 0,
        })
    );
}
