use std::str::FromStr;

#[derive(Debug)]
enum RobotMove {
    Up,
    Down,
    Left,
    Right,
}

impl From<u8> for RobotMove {
    fn from(value: u8) -> Self {
        match value {
            b'^' => RobotMove::Up,
            b'v' => RobotMove::Down,
            b'<' => RobotMove::Left,
            b'>' => RobotMove::Right,
            _ => panic!("Invalid Input"),
        }
    }
}

#[derive(Debug, Default)]
struct Warehouse {
    map: Vec<String>,
    robot_position: (usize, usize),
    robot_moves: Vec<RobotMove>,
}

#[derive(Debug)]
struct WarehouseParseError;

impl FromStr for Warehouse {
    type Err = WarehouseParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.lines()
            .filter(|line| !line.is_empty())
            .fold(Warehouse::default(), |mut w, line| {
                match line.as_bytes()[0] {
                    b'#' => {
                        if let Some(x) = line.bytes().position(|c| c == b'@') {
                            assert!(w.robot_position == (0, 0));
                            w.robot_position = (x, w.map.len());
                        }
                        w.map.push(line.into());
                    }
                    b'^' | b'v' | b'<' | b'>' => w.robot_moves.extend(
                        line.bytes()
                            .filter(|&c| matches!(c, b'^' | b'v' | b'<' | b'>'))
                            .map(|c| c.into()),
                    ),
                    _ => {}
                }
                w
            }))
    }
}

impl Warehouse {
    fn find_free(&self, direction: (isize, isize)) -> Option<(usize, usize)> {
        let mut curr_pos = (
            self.robot_position.0 as isize,
            self.robot_position.1 as isize,
        );
        // apply delta until we find a free space or a wall
        loop {
            curr_pos = (curr_pos.0 + direction.0, curr_pos.1 + direction.1);
            let obstacle = self
                .map
                .iter()
                .nth(curr_pos.0 as usize)
                .map(|line| line.bytes().nth(curr_pos.1 as usize).unwrap())
                .unwrap();
            match obstacle {
                b'#' => return None,
                b'.' => return Some((curr_pos.0 as usize, curr_pos.1 as usize)),
                b'O' => continue,
                _ => {
                    dbg!(&self);
                    unreachable!()
                }
            }
        }
    }

    fn move_robot(&mut self, mut new_position: (usize, usize), direction: (isize, isize)) {
        // shift objects until we reach the robot
        loop {
            let next = (
                (new_position.0 as isize - direction.0) as usize,
                (new_position.1 as isize - direction.1) as usize,
            );
            let entry = self.map[next.0].as_bytes()[next.1];
            unsafe {
                self.map[new_position.0].as_bytes_mut()[new_position.1] = entry;
            }
            if next == self.robot_position {
                unsafe {
                    self.map[self.robot_position.0].as_bytes_mut()[self.robot_position.1] = b'.';
                }
                self.robot_position = (
                    (self.robot_position.0 as isize + direction.0) as usize,
                    (self.robot_position.1 as isize + direction.1) as usize,
                );
                return;
            }
            new_position = next;
        }
    }

    fn exec_moves(&mut self) {
        for i in 0..self.robot_moves.len() {
            let delta = match self.robot_moves[i] {
                RobotMove::Down => (1, 0),
                RobotMove::Up => (-1, 0),
                RobotMove::Left => (0, -1),
                RobotMove::Right => (0, 1),
            };
            if let Some(new_position) = self.find_free(delta) {
                self.move_robot(new_position, delta);
            }
        }
    }

    fn sum_boxes(&self) -> usize {
        self.map
            .iter()
            .enumerate()
            .skip(1)
            .take(self.map.len() - 2)
            .map(|(row_index, row)| {
                row.bytes()
                    .enumerate()
                    .filter(|(_, c)| *c == b'O')
                    .map(|(col_index, _)| 100 * row_index + col_index)
                    .sum::<usize>()
            })
            .sum()
    }
}

fn main() {
    let input = include_str!("../puzzle_input");
    println!("Part 1: {}", part1(input));
    // println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut warehouse = input.parse::<Warehouse>().unwrap();
    warehouse.exec_moves();
    warehouse.sum_boxes()
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_parse() {
        let warehouse: super::Warehouse = TEST_INPUT.parse().unwrap();
        dbg!(&warehouse);
    }

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(TEST_INPUT), 2028);
    }

    #[test]
    fn test_part1_large() {
        assert_eq!(super::part1(TEST_INPUT_LARGE), 10092);
    }

    #[test]
    fn test_part2() {
        todo!()
    }

    const TEST_INPUT: &str = r"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    const TEST_INPUT_LARGE: &str = r"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
}
