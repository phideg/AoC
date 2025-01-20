use std::str::FromStr;

#[derive(Debug)]
struct RobotParseError;

#[derive(Default, Debug, PartialEq)]
struct Robot {
    p_x: isize,
    p_y: isize,
    v_x: isize,
    v_y: isize,
}

impl FromStr for Robot {
    type Err = RobotParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.split_whitespace().fold(Robot::default(), |mut acc, v| {
            let mut tokens = v.split_terminator('=');
            match tokens.next().unwrap() {
                "p" => {
                    tokens
                        .next()
                        .unwrap()
                        .split_terminator(',')
                        .enumerate()
                        .for_each(|(i, s)| {
                            if i == 0 {
                                acc.p_x = s.parse().unwrap();
                            } else {
                                acc.p_y = s.parse().unwrap();
                            }
                        });
                }
                "v" => {
                    tokens
                        .next()
                        .unwrap()
                        .split_terminator(',')
                        .enumerate()
                        .for_each(|(i, s)| {
                            if i == 0 {
                                acc.v_x = s.parse().unwrap();
                            } else {
                                acc.v_y = s.parse().unwrap();
                            }
                        });
                }
                _ => panic!("Invalid user input!"),
            }
            acc
        }))
    }
}

fn main() {
    let input = include_str!("../puzzle_input");
    println!("Part 1: {}", part1(input, 100, 101, 103));
    // println!("Part 2: {}", part2(input));
}

// fn eval_robots(robots: &[Robot]) -> usize {
//     let quadrants = robots
//         .iter()
//         .filter(|v| v.p_x != middle_x && v.p_y != middle_y)
//         .fold([0_usize, 0, 0, 0], |mut acc, v| {
//             match (v.p_x < middle_x, v.p_y < middle_y) {
//                 (true, true) => acc[0] += 1,
//                 (true, false) => acc[1] += 1,
//                 (false, true) => acc[2] += 1,
//                 (false, false) => acc[3] += 1,
//             }
//             acc
//         });
//     quadrants.iter().filter(|&v| *v != 0).product()
// }

fn part1(input: &str, seconds: isize, wide: isize, tall: isize) -> usize {
    let robots = input
        .lines()
        .map(|line| {
            line.parse::<Robot>()
                .map(|mut r| {
                    r.p_x = if r.v_x < 0 {
                        (r.p_x + (seconds * (wide + r.v_x))) % wide
                    } else if r.v_x > 0 {
                        ((seconds * r.v_x) + r.p_x) % wide
                    } else {
                        r.p_x
                    };
                    r.p_y = if r.v_y < 0 {
                        (r.p_y + (seconds * (tall + r.v_y))) % tall
                    } else if r.v_x > 0 {
                        (r.p_y + (seconds * r.v_y)) % tall
                    } else {
                        r.p_y
                    };
                    r
                })
                .unwrap()
        })
        .collect::<Vec<_>>();
    let middle_x = (wide / 2) + 1;
    let middle_y = (tall / 2) + 1;
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::Robot;

    #[test]
    fn test_parse_robot() {
        assert_eq!(
            "p=0,4 v=3,-3".parse::<Robot>().unwrap(),
            Robot {
                p_x: 0,
                p_y: 4,
                v_x: 3,
                v_y: -3
            }
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(TEST_INPUT, 100, 11, 7), 12);
    }

    #[test]
    fn test_part2() {
        todo!()
    }

    const TEST_INPUT: &str = r"15887950
16495136
527345
704524
1553684
12683156
11100544
12249484
7753432
5908254";
}
