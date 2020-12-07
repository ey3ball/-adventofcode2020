use std::collections::HashMap;
use std::collections::HashSet;
use counter::Counter;
use regex::Regex;

type Bags<'a> = HashMap<&'a str, HashMap<&'a str, usize>>;

pub fn parse<'a>(input: &'a str) -> Bags<'a> {
    let re_container = Regex::new(r"^(?P<bag>[a-z ]*) bags contain (?P<contents>.*)").unwrap();
    let re_contents = Regex::new(r"(?P<qty>\d)+ (?P<bag>[a-z ]*) bag[s]?[ .,]?").unwrap();
    input.lines().map(
        |line| {
            let container = re_container.captures(line).unwrap();
            let parsed_contents = re_contents
                .captures_iter(line)
                .map(|c| (c.name("bag").unwrap().as_str(), c["qty"].parse().unwrap()))
                .collect();
            (container.name("bag").unwrap().as_str(), parsed_contents)
        }
    ).collect()
}

#[aoc(day7, part1)]
fn part1(raw: &str) -> usize {
    let input = parse(raw);
    let mut target = HashSet::new();
    target.insert("shiny gold");
    loop {
        let new =
            input.keys()
            .filter(|k| &target & &input[*k].keys().copied().collect() != HashSet::new())
            .copied()
            .collect();
        if target == &target | &new {
            break
        } else {
            target = &target | &new
        }
    }
    target.iter().count() - 1
}

#[aoc(day7, part2)]
fn part2(raw: &str) -> usize {
    let input = parse(raw);

    let mut total_bags: usize = 0;
    let mut totals = Counter::<&str>::new();
    let mut step = vec![("shiny gold", 1)];
    loop {
        let next_step = step.iter().flat_map(|(x, count)| {
            (input[*x]).iter().map(move |(y, i)| (y, (count*i)))
        }).fold(Counter::<&str>::new(), |mut c, (y,i)| {
            c[y] += i;
            c
        });
        totals += next_step.clone();

        let next_total_bags = totals.iter().fold(0, |c, (_k,v)| c + v);
        step = next_step.clone().into_map().iter().map(|(k,v)| (*k, *v)).collect();
        if total_bags == next_total_bags {
            break
        } else {
            total_bags = next_total_bags
        }
    }
    total_bags
}
