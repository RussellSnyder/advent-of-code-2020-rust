#[derive(Debug)]
struct Bag {
    color: String,
    count: i16,
}

#[derive(Debug)]
struct Rule {
    color: String, // top level bag color
    contained_bags: Option<Vec<Bag>>,
}

fn create_bag(color_1: &str, color_2: &str, count_str: &str) -> Bag {
    let mut color = color_1.to_owned();
    color.push_str(" ");
    color.push_str(color_2);

    let count = count_str.parse::<i16>().unwrap();

    Bag{ count, color }
}

pub fn part1(inp: String) {
    let lines = read_lines(&inp);

    let rules_with_shiny_gold = lines
        .iter()
        .filter(|line| line.len() > 0)
        .map(|line| parse_line(line))
        .filter(|rule| has_shiny_gold_bag(&rule.contained_bags))
        // TODO: go deep
        // [ x -> [..shinygold.]   y -> [ ...[ shiny gold]  ]  ]
        .fold(0, |acc, _| acc + 1);

    // let rules_with_shiny_gold = rules
    //     .iter()
    //     .filter(|rule| has_shiny_gold_bag(&rule.contained_bags))

    println!("{:?}", rules_with_shiny_gold);
}

// TABLE: red is shiny
// blue -> [green, red(calc.)]
// yellow -> [purple, red(lookup)]
// green -> []
// red -> [SHINYGOLD]

fn has_shiny_gold_bag(bags: &Option<Vec<Bag>>) -> bool {
    match bags {
        None => false,
        Some(bags) => bags.iter().any(
            // is bag of shiny gold color OR
            // -> search recursive in bag
            //
            |bag| bag.color == "shiny gold"
        ),
    }
}

pub fn part2(inp: String) {
    let lines = read_lines(&inp);

    println!("{:?}", lines);
}

fn read_lines(inp: &str) -> Vec<&str> {
    inp.split("\n")
        .filter(|line| line.len() > 0)
        .collect::<Vec<&str>>()
}

fn parse_line(line: &str) -> Rule {
    let groups = line.split(" contain ");

    let vec: Vec<&str> = groups.collect();

    let color = vec[0].to_owned();
    let contained_bags: Option<Vec<Bag>> = vec[1]
        .split(", ")
        .collect::<Vec<&str>>()
        .iter()
        .map(|value| value.split(" ").collect::<Vec<&str>>())
        .map(|value| match value[0] {
            "no" => None,
            _ => Some(create_bag(value[1], value[2], value[0])),
        })
        .collect();

    Rule { color, contained_bags }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn has_shiny_gold_bag__for_no_contained_bags__returns_false() {
        let rule = Rule{ color: "egal".to_owned(), contained_bags: None };
        let result = has_shiny_gold_bag(&rule.contained_bags);
        assert_eq!(result, false);
    }

    #[test]
    pub fn has_shiny_gold_bag__for_shiny_gold_bag__returns_true() {
        let shiny_bag = create_bag("shiny", "gold", "4");
        let rule = Rule{ color: "egal".to_owned(), contained_bags: Some(vec![shiny_bag]) };
        let result = has_shiny_gold_bag(&rule.contained_bags);
        assert_eq!(result, true);
    }

    #[test]
    pub fn has_shiny_gold_bag__for_other_colored_bag__returns_false() {
        let non_shiny_bag = create_bag("boring", "grey", "42");
        let rule = Rule{ color: "egal".to_owned(), contained_bags: Some(vec![non_shiny_bag]) };
        let result = has_shiny_gold_bag(&rule.contained_bags);
        assert_eq!(result, false);
    }

}
