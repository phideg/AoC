use std::collections::HashMap;

fn main() {
    let input = include_str!("../puzzle_input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn parse_list(input: &str) -> (Vec<usize>, Vec<usize>) {
    let (mut left, mut right) = input.lines().fold((vec![], vec![]), |mut acc, line| {
        let mut list_iter = line.split_whitespace();
        let left: usize = list_iter.next().unwrap().parse().unwrap();
        acc.0.push(left);
        let right: usize = list_iter.next().unwrap().parse().unwrap();
        acc.1.push(right);
        acc
    });
    left.sort();
    right.sort();
    (left, right)
}

fn part1(input: &str) -> usize {
    let (left, right) = parse_list(input);
    left.iter()
        .zip(right.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum()
}

fn part2(input: &str) -> usize {
    let (left, right) = parse_list(input);
    let right = right.iter().fold(HashMap::new(), |mut acc, next| {
        acc.entry(*next)
            .and_modify(|v| *v += *next)
            .or_insert(*next);
        acc
    });
    left.iter().map(|v| right.get(v).unwrap_or(&0_usize)).sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        assert_eq!(super::part1(TEST_INPUT), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(TEST_INPUT), 31);
    }

    const TEST_INPUT: &str = r"3   4
4   3
2   5
1   3
3   9
3   3";
}
