use counter::Counter;

type Password = (usize, usize, char, String);

#[aoc_generator(day2)]
pub fn generator(input: &str) -> Vec<Password> {

    input.lines().map(|entry| {
        let splitted: Vec<&str> = entry.split(' ').collect();
        let freq: Vec<&str> = splitted[0].split('-').collect();
        let freq_min: usize = freq[0].parse().unwrap();
        let freq_max: usize = freq[1].parse().unwrap();
        let c = splitted[1].chars().next().unwrap();
        let password = splitted[2];

        (freq_min, freq_max, c, password.to_owned())
    }).collect()
}

#[aoc(day2, part1)]
pub fn part1(passwords: &Vec<Password>) -> usize {
    passwords.iter().filter(
        |(min, max, c, password)| {
            let counter = password.chars().collect::<Counter<_>>();
            //println!("{:#?} {:#?} {:#?}", password, c, counter[c]);

            if counter[c] >= *min && counter[c] <= *max {
                true
            } else {
                false
            }
    }).count()
}

#[aoc(day2, part2)]
pub fn part2(passwords: &Vec<Password>) -> usize {
    passwords.iter().filter(
        |(p1, p2, c, password)| {
            let chars: Vec<char> = password.chars().collect();
            let pos1 = *p1 - 1;
            let pos2 = *p2 - 1;
            (chars[pos1] == *c || chars[pos2] == *c)
                && chars[pos1] != chars[pos2]
    }).count()
}
