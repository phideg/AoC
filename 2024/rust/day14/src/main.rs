use std::{cmp::Ordering, str::FromStr};

#[derive(Debug)]
struct RobotParseError;

#[derive(Default, Debug, PartialEq)]
struct Robot {
    p_x: usize,
    p_y: usize,
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

impl Robot {
    fn update_pos(&mut self, seconds: usize, wide: usize, tall: usize) {
        let r = self;
        r.p_x = match r.v_x.cmp(&0) {
            Ordering::Less => (r.p_x + (seconds * (wide - r.v_x.unsigned_abs()))) % wide,
            Ordering::Greater => ((seconds * r.v_x as usize) + r.p_x) % wide,
            Ordering::Equal => r.p_x,
        };
        r.p_y = match r.v_y.cmp(&0) {
            Ordering::Less => (r.p_y + (seconds * (tall - r.v_y.unsigned_abs()))) % tall,
            Ordering::Greater => (r.p_y + (seconds * r.v_y as usize)) % tall,
            Ordering::Equal => r.p_y,
        };
    }
}

fn print_robots(robots: &[Robot], width: usize, height: usize) {
    let mut field = vec![vec![0_usize; width]; height];
    robots.iter().for_each(|r| field[r.p_y][r.p_x] += 1);
    field.iter().for_each(|line| {
        line.iter().for_each(|p| {
            if p == &0 {
                print!(".");
            } else {
                print!("{p}");
            }
        });
        println!();
    });
}

fn get_quadrants(robots: &[Robot], width: usize, height: usize) -> [usize; 4] {
    // sum quadrants
    let middle_x = width / 2;
    let middle_y = height / 2;
    robots.iter().fold([0_usize, 0, 0, 0], |mut acc, r| {
        match (r.p_x.cmp(&middle_x), r.p_y.cmp(&middle_y)) {
            (Ordering::Less, Ordering::Less) => acc[0] += 1,
            (Ordering::Greater, Ordering::Less) => acc[1] += 1,
            (Ordering::Less, Ordering::Greater) => acc[2] += 1,
            (Ordering::Greater, Ordering::Greater) => acc[3] += 1,
            _ => {}
        }
        acc
    })
}

fn get_robots(input: &str, seconds: usize, width: usize, height: usize) -> Vec<Robot> {
    input
        .lines()
        .map(|line| {
            line.parse::<Robot>()
                .map(|mut r| {
                    r.update_pos(seconds, width, height);
                    r
                })
                .unwrap()
        })
        .collect::<Vec<_>>()
}

fn part1(input: &str, seconds: usize, width: usize, height: usize) -> usize {
    let robots = get_robots(input, seconds, width, height);
    print_robots(&robots, width, height);
    let quadrants = get_quadrants(&robots, width, height);
    quadrants.iter().product()
}

fn part2(input: &str, width: usize, height: usize) -> usize {
    println!("{input}, {width}, {height}");
    //// while watching the output of a loop it seemed obvious
    //// that there is pattern reoccurring every 101 seconds
    //// starting from 77 onwards
    // let mut i = 77;
    // loop {
    //     let robots = get_robots(input, i, width, height);
    //     print_robots(&robots, width, height);
    //     println!("{i}");
    //     sleep(time::Duration::from_millis(100));
    //     i += 101;
    // };
    // at following point in time a tree was visible
    7753
}

fn main() {
    let width = 101;
    let height = 103;
    let input = include_str!("../puzzle_input");
    println!("Part 1: {}", part1(input, 100, width, height));
    let part2 = part2(input, width, height);
    println!("Part 2: {part2}");
    // print the easteregg
    let robots = get_robots(input, part2, width, height);
    print_robots(&robots, width, height);
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
    fn test_update_pos() {
        let mut robot = "p=2,4 v=2,-3".parse::<Robot>().unwrap();
        robot.update_pos(5, 11, 7);
        assert_eq!(
            robot,
            Robot {
                p_x: 1,
                p_y: 3,
                v_x: 2,
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

    const TEST_INPUT: &str = r"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
}
