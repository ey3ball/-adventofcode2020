use std::collections::HashMap;
use regex::Regex;

#[derive(Debug, Clone)]
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

fn matches(input: &str, rule: &Rule, rules: &HashMap<u64, Rule>) -> Option<Vec<usize>> {
    match rule {
        Rule::Token(c) => {
            if let Some(n) = input.chars().next() {
                if n == *c { Some(vec![1]) } else { None }
            } else {
                None
            }
        }
        Rule::Or(left, right) => {
            let m1 = matches(input, left, rules);
            let m2 = matches(input, right, rules);

            if m1.is_some() && m2.is_some() {
                let mut unm1 = m1.unwrap();
                unm1.append(&mut m2.unwrap());
                Some(unm1)
            } else if m1.is_some() {
                m1
            } else if m2.is_some() {
                m2
            } else {
                None
            }
        },
        Rule::Chain(ids) => {
            let mut acc = vec![0];
            let res = ids.iter()
               .try_for_each(|id| {
                    let prev = acc.clone();
                    acc.clear();
                    for &offset in prev.iter() {
                        let m = matches(&input[offset..], &rules[id], rules);
                        if m.is_some() {
                            m.unwrap().iter().for_each(|x| acc.push(x + offset));
                        }
                    }
                    if acc.iter().next().is_some() {
                        Ok(())
                    } else {
                        Err(())
                    }
               });
            if res.is_ok() {
                Some(acc)
            } else {
                None
            }
        }
    }
}

fn match_exact(input: &str, rule: &Rule, rules: &HashMap<u64, Rule>) -> bool {
    let m = matches(input, rule, rules);

    if m.is_some() {
        m.unwrap().contains(&input.chars().count())
    } else {
        false
    }
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

#[aoc(day19, part2)]
fn part2(input: &Parsed) -> usize {
    let mut rules = input.0.clone();
    rules.insert(8, decode("42 | 42 8"));
    rules.insert(11, decode("42 31 | 42 11 31"));

    let base_rule = &rules.get(&0).unwrap();
    input.1.iter()
           .filter(|x| match_exact(x, base_rule, &rules))
           .count()
}


#[cfg(test)]
mod tests {
    use super::*;

    // (()) and ()() both result in floor 0.
    #[test]
    fn rules_p1() {
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

    fn rules_chain_n() {
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
