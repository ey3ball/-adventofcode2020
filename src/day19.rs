use std::collections::HashMap;
use regex::Regex;

#[derive(Debug)]
pub enum Rule {
    Token(char),
    Or(Box<Rule>, Box<Rule>),
    Chain(Vec<u64>)
}

type Parsed = (HashMap<u64, Rule>, Vec<String>);

lazy_static! {
    static ref DEF_RULE: Regex = Regex::new("^([0-9]*): (.*)$").unwrap();

    static ref OR_RULE: Regex = Regex::new("^([0-9 ]+) [|] ([0-9 ]+)$").unwrap();
    static ref CHAIN_RULE: Regex = Regex::new("([0-9]+)[ ]?").unwrap();
    static ref TOKEN_RULE: Regex = Regex::new("^[\"]([a-z])[\"]$").unwrap();
}

fn decode(rule: &str) -> Rule {
    if OR_RULE.is_match(rule) {
        let captures = OR_RULE.captures(rule).unwrap();
        Rule::Or(Box::new(decode(&captures[1])), Box::new(decode(&captures[2])))
    } else if TOKEN_RULE.is_match(rule) {
        Rule::Token(rule.chars().nth(1).unwrap())
    } else {
        Rule::Chain(
            CHAIN_RULE
                .captures_iter(rule)
                .map(|c| c[1].parse().unwrap())
                .collect()
        )
    }
}

fn matches(input: &str, rule: &Rule, rules: &HashMap<u64, Rule>) -> usize {
    match rule {
        Rule::Token(c) => if input.chars().next().unwrap() == *c { 1 } else { 0 },
        Rule::Or(left, right) =>
            std::cmp::max(
                matches(input, left, rules), matches(input, right, rules)
            ),
        Rule::Chain(ids) => {
            let res = ids.iter()
               .try_fold(0, |acc, id| {
                    let count = matches(&input[acc..], &rules[id], rules);
                    if count != 0 {
                        Some(acc + count)
                    } else {
                        None
                    }
               });
            if let Some(count) = res {
                count
            } else {
                0
            }
        }
    }
}

fn match_exact(input: &str, rule: &Rule, rules: &HashMap<u64, Rule>) -> bool {
    matches(input, rule, rules) == input.chars().count()
}

#[aoc_generator(day19)]
pub fn generator(input: &str) -> Parsed {
    let mut iter = input.lines();
    let rules = iter
        .by_ref()
        .take_while(|x| x != &"")
        .map(|l| {
            let captures = DEF_RULE.captures(l).unwrap();
            (captures[1].parse().unwrap(), decode(&captures[2]))
        })
        .collect();
    let messages = iter.map(|x| x.to_owned()).collect();

    (rules, messages)
}

#[aoc(day19, part1)]
fn part1(input: &Parsed) -> usize {
    let base_rule = &input.0.get(&0).unwrap();
    input.1.iter()
           .filter(|x| match_exact(x, base_rule, &input.0))
           .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    // (()) and ()() both result in floor 0.
    #[test]
    fn rules() {
        let test = concat!(
            r#"0: 4 1 5"#, "\n",
            r#"1: 2 3 | 3 2"#, "\n",
            r#"2: 4 4 | 5 5"#, "\n",
            r#"3: 4 5 | 5 4"#, "\n",
            r#"4: "a""#, "\n",
            r#"5: "b""#, "\n",
        );

        let testrules: HashMap<u64, Rule>
            = test.lines()
                  .map(|l| {
                        let captures = DEF_RULE.captures(l).unwrap();
                        (captures[1].parse().unwrap(), decode(&captures[2]))
                  })
                  .collect();

        assert!(match_exact("ababbb", &testrules[&0], &testrules));
        assert!(match_exact("ababbb", &testrules[&0], &testrules));
        assert!(!match_exact("bababa", &testrules[&0], &testrules));
        assert!(!match_exact("aaabbb", &testrules[&0], &testrules));
        assert!(!match_exact("aaaabbb", &testrules[&0], &testrules));
    }
}
