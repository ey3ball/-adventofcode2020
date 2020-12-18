use std::str;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Value(u64),
    Expr(Box<Expr>, char, Box<Expr>),
    Eval(Box<Expr>),
}

pub fn expression_str(math: &[u8]) -> (bool, &[u8], &[u8]) {
    let (precedence, start, end) = if math[0] == '(' as u8 {
        let end = math[1..].iter().scan(1, |state, &x| {
            if x == ')' as u8 {
                *state = *state - 1;
            } else if x == '(' as u8 {
                *state = *state + 1;
            }

            if *state == 0 {
                None
            } else {
                Some(x)
            }
        }).count() + 1;
        (true, 1, end)
    } else {
        (false, 0, math.iter()
            .position(|x| ['+' as u8, '*' as u8].contains(x))
            .unwrap_or(math.iter().count()))
    };
    let expr = &math[start..end];
    let rest = &math[end..];
    let rest = if !rest.is_empty() && rest[0] == ')' as u8 {
        &rest[1..]
    } else {
        rest
    };
    (precedence, expr, rest)
}

pub fn operands(math: &[u8]) -> Expr {
    if !math.iter().any(|x| [')' as u8, '+' as u8, '*' as u8].contains(x)) {
        return Expr::Value(str::from_utf8(math).unwrap().parse().unwrap())
    }

    let (precedence, arg1, rest) = expression_str(math);
    let e1 = if precedence {
        Expr::Eval(Box::new(operands(arg1)))
    } else {
        operands(arg1)
    };

    let operator_at = rest
        .iter()
        .position(|x| ['+' as u8, '*' as u8].contains(x));
    if let Some(pos) = operator_at {
        let operator = &rest[..=pos];
        let arg2 = &rest[pos+1..];

        Expr::Expr(
            Box::new(e1),
            *operator.iter().next().unwrap() as char,
            Box::new(operands(arg2))
        )
    } else {
        e1
    }
}

pub fn apply(x: u64, op: char, y: u64) -> u64 {
    if op == '+' {
        x + y
    } else if op == '*' {
        x * y
    } else {
        panic!()
    }
}

pub fn dfs(e: &Expr, s: &mut Vec<(u64, char)>, next_op: char) -> () {
    match e {
        Expr::Value(x) => s.push((*x, next_op)),
        Expr::Expr(x, op, y) => {
            dfs(x, s, *op);
            dfs(y, s, next_op);
        },
        Expr::Eval(_) => {
            let value = eval(e);
            s.push((value, next_op))
        }
    }
}

pub fn compute(s: &Vec<(u64, char)>) -> u64 {
    s.iter().fold((0, '+'), |(acc, op), (value, next_op)| {
        (apply(acc, op, *value), *next_op)
    }).0
}

pub fn eval(e: &Expr) -> u64 {
    match e {
        Expr::Value(x) => *x,
        Expr::Expr(_, _, _) => {
            let mut s: Vec<(u64, char)> = Vec::new();
            dfs(e, &mut s, '=');
            println!("{:#?}", s);
            compute(&s)
        },
        Expr::Eval(wrapped) => {
            eval(wrapped)
        }
    }
}

#[aoc_generator(day18)]
pub fn generator(input: &str) -> Vec<Expr> {
    input.lines()
         .map(|l| {
            operands(l.replace(" ", "").as_bytes())
         })
         .collect()
}

#[aoc(day18, part1)]
pub fn part1(input: &Vec<Expr>) -> u64 {
    input.iter()
         .map(|e| eval(e))
         .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    // (()) and ()() both result in floor 0.
    #[test]
    fn parses() {
        //let expr = "(4)".replace(" ", "");
        //let parsed = operands(expr.as_bytes());
        //assert_eq!(
        //    parsed,
        //    Expr::Value(4)
        //);


        //let expr = "2".replace(" ", "");
        //let parsed = operands(expr.as_bytes());
        //assert_eq!(
        //    parsed,
        //    Expr::Value(2)
        //);

        let expr = "2+(4+1)".replace(" ", "");
        let parsed = operands(expr.as_bytes());
        println!("{:#?}", parsed);
        //assert_eq!(
        //    parsed,
        //    Expr::Expr(
        //        Box::new(Expr::Value(2)),
        //        '+', Box::new(
        //            Expr::Expr(
        //                Box::new(Expr::Value(4)), '+', Box::new(Expr::Value(1))
        //            )
        //        )
        //    )
        //);
    }

    #[test]
    fn sums() {
        //let expr = "1 + 2 + 3".replace(" ", "");
        //let parsed = operands(expr.as_bytes());
        //assert_eq!(6, reduce(&parsed));

        //let expr = "1 + (2 * 3)".replace(" ", "");
        //let parsed = operands(expr.as_bytes());
        //assert_eq!(7, reduce(&parsed));

        let expr = "1 + 2 * 3".replace(" ", "");
        let parsed = operands(expr.as_bytes());
        println!("{:#?}", parsed);
        assert_eq!(9, eval(&parsed));

        let expr = "1 + 2 * 3 + 3 + 5".replace(" ", "");
        let parsed = operands(expr.as_bytes());
        println!("{:#?}", expr);
        println!("{:#?}", parsed);
        assert_eq!(17, eval(&parsed));

        let expr = "1 + (2 * 3) + (3 * 5)".replace(" ", "");
        let parsed = operands(expr.as_bytes());
        println!("{:#?}", expr);
        println!("{:#?}", parsed);
        assert_eq!(22, eval(&parsed));
    }
}
