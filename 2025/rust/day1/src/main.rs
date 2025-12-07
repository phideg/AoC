fn main() {
    let input = include_str!("../puzzle_input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> isize {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .fold((50, 0), |mut acc, line| {
            let mut chars = line.chars();
            match chars.next() {
                Some('R') => {
                    acc.0 = (acc.0 + chars.collect::<String>().parse::<isize>().unwrap()) % 100
                }
                Some('L') => {
                    acc.0 = ((100 + acc.0)
                        - (chars.collect::<String>().parse::<isize>().unwrap() % 100))
                        % 100
                }
                Some(x) => panic!("Unexpected code {x}"),
                _ => panic!("Unexpected empty line"),
            }
            if acc.0 == 0 {
                acc.1 += 1;
            }
            acc
        })
        .1
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .fold((50_isize, 0), |mut acc, line| {
            let mut chars = line.chars();
            match chars.next() {
                Some('R') => {
                    let n: isize = chars.collect::<String>().parse().unwrap();
                    acc.1 += (n as usize + acc.0 as usize) / 100;
                    acc.0 = (acc.0 + n) % 100;
                }
                Some('L') => {
                    let n: isize = chars.collect::<String>().parse().unwrap();
                    acc.1 += n as usize / 100;
                    let rest = n % 100;
                    if rest > 0 {
                        if rest >= acc.0 {
                            if acc.0 != 0 {
                                acc.1 += 1;
                            }
                            acc.0 = (100 + acc.0 - rest) % 100;
                        } else {
                            acc.0 = acc.0 - rest;
                        }
                    }
                }
                Some(x) => panic!("Unexpected code {x}"),
                _ => panic!("Unexpected empty line"),
            }
            acc
        })
        .1
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        assert_eq!(super::part1(TEST_INPUT), 3);
    }

    #[test]
    fn test_part2() {
        // assert_eq!(super::part2("L50"), 1);
        // assert_eq!(super::part2("R1000"), 10);
        assert_eq!(super::part2(TEST_INPUT), 6);
    }

    const TEST_INPUT: &str = r"
       L68
       L30
       R48
       L5
       R60
       L55
       L1
       L99
       R14
       L82
    ";
}
