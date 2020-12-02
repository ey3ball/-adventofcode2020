use std::fs;
use counter::Counter;
//use std::string::String;

fn parse<'a>(input: &'a str) -> (usize, usize, char, &'a str) {
    let splitted: Vec<&str> = input.split(' ').collect();
    let freq: Vec<&str> = splitted[0].split('-').collect();
    let freq_min: usize = freq[0].parse().unwrap();
    let freq_max: usize = freq[1].parse().unwrap();
    let c = splitted[1].chars().next().unwrap();
    let password = splitted[2];

    (freq_min, freq_max, c, password)
}

fn main() {
    let input = "input/input.txt";
    let contents = fs::read_to_string(input).unwrap();

    let passwords: Vec<(usize, usize, char, &str)> = contents.lines().map(
        |x| parse(x)).collect();


    let valid_passwords = passwords.iter().filter(
        |(min, max, c, password)| {
            let counter = password.chars().collect::<Counter<_>>();
            println!("{:#?} {:#?} {:#?}", password, c, counter[c]);

            if counter[c] >= *min && counter[c] <= *max {
                true
            } else {
                false
            }
    }).count();

    println!("{:#?}", valid_passwords);
}
