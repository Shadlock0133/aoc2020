use std::iter::Peekable;

fn main() {
    let input = std::fs::read_to_string("inputs/day18.txt").unwrap();
    let res = sum_lines(&input, eval_expr_1);
    println!("Part 1 - Answer: {}", res);
    let res = sum_lines(&input, eval_expr_2);
    println!("Part 2 - Answer: {}", res);
}

#[derive(Debug)]
enum Op {
    Add,
    Mul,
}

#[derive(Debug)]
enum Expr {
    Binary(Op, Box<Expr>, Box<Expr>),
    Number(u64),
}

fn eval_expr_1(input: &str) -> u64 {
    let mut stack = vec![(0, Op::Add)];
    input
        .chars()
        .filter(|ch| !ch.is_whitespace())
        .for_each(|ch| match ch {
            '0'..='9' => {
                let n = ch.to_digit(10).unwrap() as u64;
                let (top, op) = stack.last_mut().unwrap();
                match op {
                    Op::Add => *top += n,
                    Op::Mul => *top *= n,
                }
            }
            '+' => stack.last_mut().unwrap().1 = Op::Add,
            '*' => stack.last_mut().unwrap().1 = Op::Mul,
            '(' => stack.push((0, Op::Add)),
            ')' => {
                let pop = stack.pop().unwrap().0;
                let (top, op) = stack.last_mut().unwrap();
                match op {
                    Op::Add => *top += pop,
                    Op::Mul => *top *= pop,
                }
            }
            ch => unreachable!("Unrecognized character: {}", ch),
        });
    assert_eq!(stack.len(), 1);
    stack[0].0
}

// expr -> term ( '*' term )*
// term -> primary ( '+' primary )*
// primary -> number | '(' expr ')'
fn parse_expr<I: Iterator<Item = char>>(tokens: &mut Peekable<I>) -> Expr {
    let mut term = parse_term(tokens);
    while let Some('*') = tokens.peek() {
        tokens.next();
        term = Expr::Binary(Op::Mul, Box::new(term), Box::new(parse_term(tokens)));
    }
    term
}

fn parse_term<I: Iterator<Item = char>>(tokens: &mut Peekable<I>) -> Expr {
    let mut primary = parse_primary(tokens);
    while let Some('+') = tokens.peek() {
        tokens.next();
        primary = Expr::Binary(Op::Add, Box::new(primary), Box::new(parse_primary(tokens)));
    }
    primary
}

fn parse_primary<I: Iterator<Item = char>>(tokens: &mut Peekable<I>) -> Expr {
    match tokens.next().unwrap() {
        ch @ '0'..='9' => Expr::Number(ch.to_digit(10).unwrap() as u64),
        '(' => {
            let expr = parse_expr(tokens);
            assert_eq!(tokens.next(), Some(')'));
            expr
        }
        ch => unreachable!("Unrecognized character: {}", ch),
    }
}

fn eval_expr(expr: Expr) -> u64 {
    match expr {
        Expr::Number(n) => n,
        Expr::Binary(Op::Add, l, r) => eval_expr(*l) + eval_expr(*r),
        Expr::Binary(Op::Mul, l, r) => eval_expr(*l) * eval_expr(*r),
    }
}

fn eval_expr_2(input: &str) -> u64 {
    let mut tokens = input.chars().filter(|ch| !ch.is_whitespace()).peekable();
    let expr = parse_expr(&mut tokens);
    eval_expr(expr)
}

fn sum_lines(input: &str, eval: fn(&str) -> u64) -> u64 {
    input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(eval)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let tests = [
            ("1 + 2 * 3 + 4 * 5 + 6", 71),
            ("1 + (2 * 3) + (4 * (5 + 6))", 51),
            ("2 * 3 + (4 * 5)", 26),
            ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 437),
            ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 12240),
            ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 13632),
        ];
        for &(input, output) in &tests {
            let res = sum_lines(input, eval_expr_1);
            assert_eq!(res, output);
        }
    }

    #[test]
    fn test2() {
        let tests = [
            ("1 + 2 * 3 + 4 * 5 + 6", 231),
            ("1 + (2 * 3) + (4 * (5 + 6))", 51),
            ("2 * 3 + (4 * 5)", 46),
            ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 1445),
            ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 669060),
            ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 23340),
        ];
        for &(input, output) in &tests {
            let res = sum_lines(input, eval_expr_2);
            assert_eq!(res, output);
        }
    }
}
