#![feature(iter_array_chunks)]
#[derive(Debug)]
enum Operand {
    Value(usize),
    Variable,
}

#[derive(Debug)]
enum Operation {
    Add(Operand, Operand),
    Mul(Operand, Operand),
}

#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    test: (usize, usize, usize),
}

fn decode_input(input: &str) -> Vec<Monkey> {
    input
        .split_terminator('\n')
        .filter(|l| !l.is_empty())
        .array_chunks()
        .map(|[_, items, operation, test, if_true, if_false]| {
            let items = items
                .split_terminator(':')
                .last()
                .unwrap()
                .split_terminator(',')
                .map(|n| n.trim().parse().unwrap())
                .collect::<Vec<usize>>();
            let operation = operation
                .split_terminator('=')
                .last()
                .unwrap()
                .split_whitespace()
                .array_chunks()
                .map(|[operand1, operation, operand2]| {
                    let operand1 = operand1
                        .parse()
                        .map_or(Operand::Variable, |v| Operand::Value(v));
                    let operand2 = operand2
                        .parse()
                        .map_or(Operand::Variable, |v| Operand::Value(v));
                    match operation {
                        "+" => Operation::Add(operand1, operand2),
                        "*" => Operation::Add(operand1, operand2),
                        _ => panic!("Unexpected input {operand1:?}, {operation}, {operand2:?}"),
                    }
                })
                .last()
                .unwrap();
            let test = (
                test.split_whitespace().last().unwrap().parse().unwrap(),
                if_true.split_whitespace().last().unwrap().parse().unwrap(),
                if_false.split_whitespace().last().unwrap().parse().unwrap(),
            );
            Monkey {
                items,
                operation,
                test,
            }
        })
        .collect::<Vec<_>>()
}

fn part1(input: &[Monkey]) -> usize {
    0
}

fn part2(valley: &[Monkey]) -> usize {
    0
}

fn main() {
    println!("Part 1: {}", part1(&mut decode_input(INPUT)));
    println!("Part 2: {}", part2(&mut decode_input(INPUT)));
}

#[cfg(test)]
mod test {
    use crate::{decode_input, part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(18, part1(dbg!(&mut decode_input(TEST))));
    }

    #[test]
    fn test_part2() {
        assert_eq!(54, part2(&mut decode_input(TEST)));
    }

    const TEST: &str = r#"
Monkey 0:
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
    If false: throw to monkey 1

"#;
}

const INPUT: &str = r#"
Monkey 0:
  Starting items: 63, 84, 80, 83, 84, 53, 88, 72
  Operation: new = old * 11
  Test: divisible by 13
    If true: throw to monkey 4
    If false: throw to monkey 7

Monkey 1:
  Starting items: 67, 56, 92, 88, 84
  Operation: new = old + 4
  Test: divisible by 11
    If true: throw to monkey 5
    If false: throw to monkey 3

Monkey 2:
  Starting items: 52
  Operation: new = old * old
  Test: divisible by 2
    If true: throw to monkey 3
    If false: throw to monkey 1

Monkey 3:
  Starting items: 59, 53, 60, 92, 69, 72
  Operation: new = old + 2
  Test: divisible by 5
    If true: throw to monkey 5
    If false: throw to monkey 6

Monkey 4:
  Starting items: 61, 52, 55, 61
  Operation: new = old + 3
  Test: divisible by 7
    If true: throw to monkey 7
    If false: throw to monkey 2

Monkey 5:
  Starting items: 79, 53
  Operation: new = old + 1
  Test: divisible by 3
    If true: throw to monkey 0
    If false: throw to monkey 6

Monkey 6:
  Starting items: 59, 86, 67, 95, 92, 77, 91
  Operation: new = old + 5
  Test: divisible by 19
    If true: throw to monkey 4
    If false: throw to monkey 0

Monkey 7:
  Starting items: 58, 83, 89
  Operation: new = old * 19
  Test: divisible by 17
    If true: throw to monkey 2
    If false: throw to monkey 1
"#;
