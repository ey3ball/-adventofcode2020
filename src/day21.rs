use std::collections::HashMap;
use std::collections::HashSet;

use regex::Regex;

type Parsed<'a> = Vec<(HashSet<&'a str>, HashSet<&'a str>)>;

lazy_static! {
    static ref ALLERGENS: Regex = Regex::new("[(]contains ([a-z, ]+)[)]$").unwrap();
}


pub fn parse(input: &str) -> Parsed {
    input.lines()
        .map(|l| {
            let allergens = if ALLERGENS.is_match(l) {
                let captures = ALLERGENS.captures(l).unwrap();
                captures.get(1)
                    .unwrap()
                    .as_str()
                    .split(',')
                    .map(|s| s.trim())
                    .collect()
            } else {
                HashSet::new()
            };
            let ingredients = l.split(" (").next().unwrap()
                               .split(" ").collect();
            (ingredients, allergens)

        })
        .collect()
}

#[aoc(day21, part1)]
fn part1(input: &str) -> usize {
    let parsed = parse(input);
    // Allergen can contain ingredient
    let mut contained: HashMap<&str, HashSet<&str>> = HashMap::new();

    let mut all_allergens: HashSet<&str> = HashSet::new();
    let mut all_ingredients: HashSet<&str> = HashSet::new();

    parsed.iter()
        .for_each(|(ingredients, allergens)| {
            ingredients.iter().for_each(|i| { all_ingredients.insert(i); });
            allergens.iter().for_each(|a| { all_allergens.insert(a); });
        });


    parsed.iter()
        .for_each(|(ingredients, allergens)| {
            allergens.iter()
                .for_each(|a| {
                    if let Some(contents) = contained.get(a).cloned() {
                        contained.insert(a, ingredients & &contents);
                    } else {
                        contained.insert(a, ingredients.clone());
                    };
                })
        });
    let maybe: HashSet<&str> = contained.iter()
        .flat_map(|(_k, v)| { v.iter() })
        .copied()
        .collect();

    let no_allergens: HashSet<&str> = all_ingredients.difference(&maybe).copied().collect();

    parsed.iter()
        .flat_map(|(is, _)| {
            is.iter().filter(|i| no_allergens.contains(*i))
        })
        .count()
}

#[aoc(day21, part2)]
fn part2(input: &str) -> usize {
    let parsed = parse(input);
    // Allergen can contain ingredient
    let mut contained: HashMap<&str, HashSet<&str>> = HashMap::new();

    let mut all_allergens: HashSet<&str> = HashSet::new();
    let mut all_ingredients: HashSet<&str> = HashSet::new();

    parsed.iter()
        .for_each(|(ingredients, allergens)| {
            ingredients.iter().for_each(|i| { all_ingredients.insert(i); });
            allergens.iter().for_each(|a| { all_allergens.insert(a); });
        });


    parsed.iter()
        .for_each(|(ingredients, allergens)| {
            allergens.iter()
                .for_each(|a| {
                    if let Some(contents) = contained.get(a).cloned() {
                        contained.insert(a, ingredients & &contents);
                    } else {
                        contained.insert(a, ingredients.clone());
                    };
                })
        });

    println!("{:#?}", contained);

    let mut i = 0;
    loop {
        let keys: Vec<&str> = contained.keys().copied().collect();
        for k in keys.iter() {
            if contained[k].len() == 1 {
                let remove = contained[k].iter().next().unwrap().clone();
                contained.iter_mut()
                      .filter(|(l, _)| *l != k)
                      .for_each(|(_, v)| {v.remove(&remove);});
            }
        }
        if (i >= 4) {
            break;
        }
        i = i + 1;
    }

    let mut dangerous: Vec<(&str, &str)> =
        contained
            .iter()
            .map(|(k, v)| (*k, *v.iter().next().unwrap()))
            .collect();
    dangerous.sort();

    println!("{:#?}", dangerous.iter().map(|(_a,i)| *i).collect::<Vec<&str>>().join(","));
    0
}
