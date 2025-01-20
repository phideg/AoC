use std::{collections::HashSet, str::FromStr};

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
    fn find_free_horizontal(&self, row: usize, direction: isize) -> Option<usize> {
        let mut curr_pos = self.robot_position.0 as isize;
        // apply delta until we find a free space or a wall
        loop {
            curr_pos = curr_pos + direction;
            let obstacle = self
                .map
                .iter()
                .nth(row)
                .map(|line| line.bytes().nth(curr_pos as usize).unwrap())
                .unwrap();
            match obstacle {
                b'#' => return None,
                b'.' => return Some(curr_pos as usize),
                b'O' | b'[' | b']' => continue,
                c => {
                    dbg!(&self);
                    dbg!(c);
                    unreachable!()
                }
            }
        }
    }

    fn move_horizontal(&mut self, direction: isize) {
        let row = self.robot_position.1;
        if let Some(mut new_position) = self.find_free_horizontal(row, direction) {
            // shift objects until we reach the robot
            loop {
                let next = (new_position as isize - direction) as usize;
                let entry = self.map[row].as_bytes()[next];
                unsafe {
                    self.map[row].as_bytes_mut()[new_position] = entry;
                }
                if next == self.robot_position.0 {
                    unsafe {
                        self.map[row].as_bytes_mut()[self.robot_position.0] = b'.';
                    }
                    self.robot_position =
                        ((self.robot_position.0 as isize + direction) as usize, row);
                    return;
                }
                new_position = next;
            }
        }
    }

    fn find_free_vertical(&self, pos: (usize, usize), direction: isize) -> HashSet<(usize, usize)> {
        let mut positions = HashSet::new();
        let next_row = (pos.1 as isize + direction) as usize;
        match self.map[next_row].bytes().nth(pos.0) {
            Some(b'O') => {
                let next = self.find_free_vertical((pos.0, next_row), direction);
                if next.is_empty() {
                    return HashSet::new();
                }
                positions.extend(next);
                positions.insert((pos.0, next_row));
            }
            Some(b']') => {
                let new_left = self.find_free_vertical((pos.0 - 1, next_row), direction);
                let new_right = self.find_free_vertical((pos.0, next_row), direction);
                if new_right.is_empty() || new_left.is_empty() {
                    return HashSet::new();
                }
                positions.extend(new_left);
                positions.extend(new_right);
                positions.insert((pos.0, next_row));
            }
            Some(b'[') => {
                let new_left = self.find_free_vertical((pos.0, next_row), direction);
                let new_right = self.find_free_vertical((pos.0 + 1, next_row), direction);
                if new_right.is_empty() || new_left.is_empty() {
                    return HashSet::new();
                }
                positions.extend(new_left);
                positions.extend(new_right);
                positions.insert((pos.0, next_row));
            }
            Some(b'.') => {
                positions.insert((pos.0, next_row));
            }
            Some(b'#') => { /* we hit the wall -> stop! */ }
            c => {
                dbg!(&self);
                dbg!(c);
                unreachable!()
            }
        }
        positions
    }

    fn move_vertical(&mut self, direction: isize) {
        let mut new_positions: Vec<(usize, usize)> = self
            .find_free_vertical(self.robot_position, direction)
            .into_iter()
            .collect();
        if !new_positions.is_empty() {
            if direction.is_positive() {
                new_positions.sort_by(|a, b| b.1.cmp(&a.1));
            } else {
                new_positions.sort_by(|a, b| a.1.cmp(&b.1));
            }
            // now that all target locations have been found shift boxes and robot
            // if no move is possible new_positions is empty
            for new_pos in new_positions {
                unsafe {
                    self.map[new_pos.1].as_bytes_mut()[new_pos.0] =
                        self.map[(new_pos.1 as isize - direction) as usize].as_bytes()[new_pos.0];
                    self.map[(new_pos.1 as isize - direction) as usize].as_bytes_mut()[new_pos.0] =
                        b'.';
                }
            }
            // finally update position of robot, too!
            self.robot_position = (
                self.robot_position.0,
                (self.robot_position.1 as isize + direction) as usize,
            );
        }
    }

    fn exec_moves(&mut self) {
        //dbg!(&self.map);
        for i in 0..self.robot_moves.len() {
            //dbg!(&self.robot_moves[i]);
            match self.robot_moves[i] {
                RobotMove::Down => self.move_vertical(1),
                RobotMove::Up => self.move_vertical(-1),
                RobotMove::Left => self.move_horizontal(-1),
                RobotMove::Right => self.move_horizontal(1),
            }
            //dbg!(&self.map);
        }
    }

    fn sum_boxes(&self, box_symbol: u8) -> usize {
        self.map
            .iter()
            .enumerate()
            .skip(1)
            .take(self.map.len() - 2)
            .map(|(row_index, row)| {
                row.bytes()
                    .enumerate()
                    .filter(|(_, c)| *c == box_symbol)
                    .map(|(col_index, _)| 100 * row_index + col_index)
                    .sum::<usize>()
            })
            .sum()
    }
}

fn main() {
    let input = include_str!("../puzzle_input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn transform_input(input: &str) -> String {
    input
        .replace("O", "[]")
        .replace("#", "##")
        .replace(".", "..")
        .replace("@", "@.")
}

fn part1(input: &str) -> usize {
    let mut warehouse = input.parse::<Warehouse>().unwrap();
    warehouse.exec_moves();
    warehouse.sum_boxes(b'O')
}

fn part2(input: &str) -> usize {
    let input = transform_input(input);
    let mut warehouse = input.parse::<Warehouse>().unwrap();
    warehouse.exec_moves();
    warehouse.sum_boxes(b'[')
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
    fn test_transform_input() {
        assert_eq!(
            super::transform_input(TEST_INPUT_LARGE),
            EXPECTED_INPUT_PART2
        );
    }

    #[test]
    fn test_part2_large() {
        assert_eq!(super::part2(TEST_INPUT_LARGE), 9021);
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

    const EXPECTED_INPUT_PART2: &str = r"####################
##....[]....[]..[]##
##............[]..##
##..[][]....[]..[]##
##....[]@.....[]..##
##[]##....[]......##
##[]....[]....[]..##
##..[][]..[]..[][]##
##........[]......##
####################

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
