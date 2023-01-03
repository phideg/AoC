fn part1(valley: &mut Valley) -> usize {
    println!("{valley}");
    let mut shortest_path = 0;
    while shortest_path == 0 {
        valley.increment_minute();
        shortest_path = valley.try_move();
        // println!("{valley}");
    }
    shortest_path + 1
}

fn part2(valley: &mut Valley) -> usize {
    let mut shortest_path = part1(valley);
    for _ in 0..2 {
        valley.reset_tracks();
        valley.swap_entry_and_exit();
        shortest_path = part1(valley);
    }
    shortest_path
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
        assert_eq!(18, part1(&mut decode_input(TEST)));
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
