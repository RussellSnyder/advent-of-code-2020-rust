use regex::Regex;

pub fn part1(inp: String) {
    let lines = inp.split("\n").collect::<Vec<&str>>();

    let valid_passwords = lines.iter()
        .filter(|line| line.len() > 0)
        .map(|x| is_valid(x))
        .fold(0, |acc, cur| acc + match cur {
            true  => 1,
            false => 0,
        });

    println!("{}", valid_passwords);
}

pub fn part2(inp: String) {
    // TODO
}

// sample: 1-3 a: abaaabab
fn is_valid(line: &str) -> bool {
    let regex = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();

    for cap in regex.captures_iter(line) {
        let min    = cap[1].parse::<usize>().unwrap();
        let max    = cap[2].parse::<usize>().unwrap();
        let letter = &cap[3];
        let passwd = &cap[4];

        let letters_in_passwd = passwd.matches(letter).count();

        return min <= letters_in_passwd && letters_in_passwd <= max;
    }

    false
}
