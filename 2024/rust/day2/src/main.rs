use std::cmp::Ordering;

fn main() {
    let input = include_str!("../puzzle_input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let levels = line
                .split_whitespace()
                .map(|digits| digits.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            check_levels(&levels)
        })
        .sum()
}

fn check_levels(levels: &[usize]) -> usize {
    let mut prev = levels.first().unwrap();
    let mut direction: Option<Ordering> = None;
    for next in &levels[1..] {
        let new_direction = next.cmp(&prev);
        if next.abs_diff(*prev) > 3
            || direction.is_some_and(|d| new_direction == Ordering::Equal || d != new_direction)
        {
            return 0;
        }
        prev = next;
        direction = Some(new_direction);
    }
    1
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let levels = line
                .split_whitespace()
                .map(|digits| digits.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            if chk_levels_graceful(&levels) == 0 {
                check_levels(&levels[1..])
            } else {
                1
            }
        })
        .sum()
}

fn chk_levels_graceful(levels: &[usize]) -> usize {
    let mut prev = levels.first().unwrap();
    let mut direction: Option<Ordering> = None;
    let mut used_dampener = false;
    for next in &levels[1..] {
        let new_direction = next.cmp(&prev);
        if next.abs_diff(*prev) > 3
            || direction
                .as_ref()
                .is_some_and(|d| new_direction == Ordering::Equal || *d != new_direction)
        {
            if used_dampener {
                return 0;
            } else {
                used_dampener = true;
                continue;
            }
        }
        prev = next;
        direction = Some(new_direction);
    }
    1
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        assert_eq!(super::part1(TEST_INPUT), 2);
        assert_eq!(super::part1(TEST_INPUT2), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(TEST_INPUT), 4);
        assert_eq!(super::part2(TEST_INPUT2), 1);
        assert_eq!(super::part2("1 4 3 5 6 7"), 1);
        assert_eq!(super::part2("1 4 4 5 6 7"), 1);
        assert_eq!(super::part2("6 1 4 5 6 7"), 1);
        assert_eq!(super::part2("6 6 1 4 5 6 7"), 0);
    }

    const TEST_INPUT: &str = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    const TEST_INPUT2: &str = r"96 92 90 87 84";
}
