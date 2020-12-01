pub fn part1(inp: String) {
    let lines = inp.split("\n").collect::<Vec<&str>>();
    let numbers = lines.iter()
        .filter(|line| line.len() > 0)
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut iter1 = numbers.iter().peekable();

    while iter1.peek() != None {
        let cur1 = iter1.next().unwrap();
        let mut iter2 = iter1.clone();
        while iter2.peek() != None {
            let cur2 = iter2.next().unwrap();
            if cur1 + cur2 == 2020 {
                let result = cur1 * cur2;
                println!("{}", result);
                return;
            }
        }
    }
}
