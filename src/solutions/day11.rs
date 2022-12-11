use regex::Regex;
use std::collections::HashMap;
use math::round;

struct Operation {
    a: String,
    op: String,
    b: String,
}

struct Monkey {
    count: usize,
    items: Vec<u32>,
    op: Operation,
    test: u32,
    if_true: usize,
    if_false: usize,
}

pub fn solve_one(input: Vec<String>) -> String {
    let mut monkeys = parse_monkey(split_input(input));
    let mut item: HashMap<u32, usize> = HashMap::new();
    for _ in 0..20 {
        for idx in 0..monkeys.len() {
            monkeys[idx].count += monkeys[idx].items.len();
            for item in monkeys[idx].items.iter() {
                let val = calculate_worry(*item, &monkeys[idx].op);
            }
        }
    }

    format!("{}", 0)
}

pub fn solve_two(input: Vec<String>) -> String {
    let monkeys = parse_monkey(split_input(input));

    format!("{}", 0)
}

fn calculate_worry(val: u32, op: &Operation) -> u32 {
    let b = op.b.parse::<u32>().unwrap_or(val);
    // judging from the input, the worry value always increases
    if op.op == "+" {
        return val + b;
    }

    val * b
}

fn split_input(input: Vec<String>) -> Vec<Vec<String> > {
    let mut result: Vec<Vec<String> > = vec![];
    let mut temp: Vec<String> = vec![];

    let mut skip: usize = 6;

    for (idx, val) in input.iter().enumerate() {
        if idx == skip {
            result.push(temp.clone());
            temp.clear();

            skip += 7;
            continue;
        }

        temp.push(val.trim().to_string());
    }

    result.push(temp.clone());

    result
}

fn parse_monkey(input: Vec<Vec<String> >) -> Vec<Monkey> {
    let mut monkeys = vec![];

    let digit = Regex::new(r"(\d+)").unwrap();
    let op = Regex::new(r"Operation: new = (\w+) ([\+\-*/]) (\w+)").unwrap();

    for data in input.iter() {
        let items: Vec<u32> = digit.find_iter(&data[1])
            .map(|x| x.as_str().parse::<u32>().unwrap())
            .collect::<Vec<_> >();
        
        let ops = op.captures(&data[2]).unwrap();
        let operation = Operation{
            a: ops.get(1).unwrap().as_str().to_string(),
            op: ops.get(2).unwrap().as_str().to_string(),
            b: ops.get(3).unwrap().as_str().to_string(),
        };

        let test = digit.captures(&data[3]).unwrap().get(0)
            .unwrap().as_str().parse::<u32>().unwrap();
        let if_true = digit.captures(&data[4]).unwrap().get(0)
            .unwrap().as_str().parse::<usize>().unwrap();
        let if_false = digit.captures(&data[5]).unwrap().get(0)
            .unwrap().as_str().parse::<usize>().unwrap();

        monkeys.push(Monkey{
            count: 0,
            items: items,
            op: operation,
            test: test,
            if_true: if_true,
            if_false: if_false,
        });
    }

    monkeys
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_one() {
        let str = "Monkey 0:
  Starting items: 98, 70, 75, 80, 84, 89, 55, 98
  Operation: new = old * 2
  Test: divisible by 11
    If true: throw to monkey 1
    If false: throw to monkey 4
      
Monkey 1:
  Starting items: 59
  Operation: new = old * old
  Test: divisible by 19
    If true: throw to monkey 7
    If false: throw to monkey 3";
    
        let input: Vec<String> = str.split("\n").map(|x| x.to_string()).collect();

        assert_eq!(solve_one(input), "1");
    }

    #[test]
    fn test_solve_two() {
        let str = "";
    
        let input: Vec<String> = str.split("\n").map(|x| x.to_string()).collect();

        assert_eq!(solve_two(input), "");
    }
}