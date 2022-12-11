use regex::Regex;

struct Operation {
    op: String,
    b: String,
}

struct Monkey {
    count: usize,
    items: Vec<u64>,
    op: Operation,
    test: u64,
    if_true: usize,
    if_false: usize,
}

pub fn solve_one(input: Vec<String>) -> String {
    let mut monkeys = parse_monkey(split_input(input));
    for _ in 0..20 {
        for idx in 0..monkeys.len() {
            monkeys[idx].count += monkeys[idx].items.len();
            for i in 0..monkeys[idx].items.len() {
                let worry = calculate_worry(monkeys[idx].items[i], &monkeys[idx].op);
                let val = worry / 3;

                let mut dest = monkeys[idx].if_true;
                if val % monkeys[idx].test != 0 {
                    dest = monkeys[idx].if_false;
                }

                monkeys[dest].items.push(val);
            }

            monkeys[idx].items.clear();
        }
    }

    monkeys.sort_by_key(|m| m.count);
    monkeys.reverse();

    format!("{}", monkeys[0].count * monkeys[1].count)
}

pub fn solve_two(input: Vec<String>) -> String {
    let mut monkeys = parse_monkey(split_input(input));

    // to keep the divisibilty intact, mod with all the divisor
    let normalizer: u64 = monkeys.iter().map(|m| m.test).product();

    for _ in 0..10000 {
        for idx in 0..monkeys.len() {
            monkeys[idx].count += monkeys[idx].items.len();
            for i in 0..monkeys[idx].items.len() {
                let worry = calculate_worry(monkeys[idx].items[i].clone(), &monkeys[idx].op);
                let val = worry % normalizer;

                let mut dest = monkeys[idx].if_true;
                if val % monkeys[idx].test != 0 {
                    dest = monkeys[idx].if_false;
                }

                monkeys[dest].items.push(val);
            }

            monkeys[idx].items.clear();
        }
    }

    monkeys.sort_by_key(|m| m.count);
    monkeys.reverse();

    format!("{}", monkeys[0].count * monkeys[1].count)
}

fn calculate_worry(val: u64, op: &Operation) -> u64 {
    let b = op.b.parse::<u64>().unwrap_or(val);

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
        let items: Vec<u64> = digit.find_iter(&data[1])
            .map(|x| x.as_str().parse::<u64>().unwrap())
            .collect::<Vec<_> >();
        
        let ops = op.captures(&data[2]).unwrap();
        let operation = Operation{
            op: ops.get(2).unwrap().as_str().to_string(),
            b: ops.get(3).unwrap().as_str().to_string(),
        };

        let test = digit.captures(&data[3]).unwrap().get(0)
            .unwrap().as_str().parse::<u64>().unwrap();
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
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3
      
Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0
      
Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3
      
Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
    
        let input: Vec<String> = str.split("\n").map(|x| x.to_string()).collect();

        assert_eq!(solve_one(input), "10605");
    }

    #[test]
    fn test_solve_two() {
        let str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3
      
Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0
      
Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3
      
Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
    
        let input: Vec<String> = str.split("\n").map(|x| x.to_string()).collect();

        assert_eq!(solve_two(input), "2713310158");
    }
}