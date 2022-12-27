use std::fmt::Display;

type Blizzard = Vec<Vec<u32>>;

#[derive(Debug)]
struct Valley {
    left: Blizzard,
    right: Blizzard,
    top: Blizzard,
    down: Blizzard,
    tracks: Vec<(usize, usize, usize)>,
    entry: (usize, usize),
    exit: (usize, usize),
}

impl Valley {
    fn new(rows: usize, entry: (usize, usize), exit: (usize, usize)) -> Self {
        Self {
            left: Vec::with_capacity(rows),
            right: Vec::with_capacity(rows),
            top: Vec::with_capacity(rows),
            down: Vec::with_capacity(rows),
            tracks: Vec::new(),
            entry,
            exit,
        }
    }

    fn push_blizzard_left(&mut self, blizzard: Vec<u32>) {
        self.left.push(blizzard);
    }
    fn push_blizzard_right(&mut self, blizzard: Vec<u32>) {
        self.right.push(blizzard);
    }
    fn push_blizzard_top(&mut self, blizzard: Vec<u32>) {
        self.top.push(blizzard);
    }
    fn push_blizzard_down(&mut self, blizzard: Vec<u32>) {
        self.down.push(blizzard);
    }
    fn increment_minute() {}
}

impl Display for Valley {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..self.left.len() {
            for c in 0..self.left[r].len() {
                match (
                    self.left[r][c],
                    self.right[r][c],
                    self.down[r][c],
                    self.top[r][c],
                ) {
                    (l, 0, 0, 0) => {
                        write!(
                            f,
                            "{}",
                            if l == 1 {
                                "<".to_string()
                            } else {
                                l.to_string()
                            }
                        )?;
                    }
                    (0, r, 0, 0) => {
                        write!(
                            f,
                            "{}",
                            if r == 1 {
                                ">".to_string()
                            } else {
                                r.to_string()
                            }
                        )?;
                    }
                    (0, 0, d, 0) => {
                        write!(
                            f,
                            "{}",
                            if d == 1 {
                                "v".to_string()
                            } else {
                                d.to_string()
                            }
                        )?;
                    }
                    (0, 0, 0, t) => {
                        write!(
                            f,
                            "{}",
                            if t == 1 {
                                "^".to_string()
                            } else {
                                t.to_string()
                            }
                        )?;
                    }
                    _ => {
                        write!(f, ".")?;
                    }
                }
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

fn decode_input(input: &str) -> Valley {
    let lines = input
        .split_terminator('\n')
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>();
    let valley_rows = lines.len() - 2;
    let valley_cols = lines[0].len() - 2;
    let mut valley = Valley::new(
        valley_rows,
        (lines[0].chars().position(|c| c == '.').unwrap(), 0),
        (
            lines[lines.len() - 1]
                .chars()
                .position(|c| c == ' ')
                .unwrap(),
            0,
        ),
    );
    lines.iter().skip(1).take(valley_rows).for_each(|row| {
        valley.push_blizzard_right(
            row.as_bytes()
                .iter()
                .skip(1)
                .take(valley_cols)
                .enumerate()
                .filter(|(_, &b)| b == b'>')
                .fold(vec![0; valley_cols], |mut acc, (i, _)| {
                    acc[i] += 1;
                    acc
                }),
        );
        valley.push_blizzard_left(
            row.as_bytes()
                .iter()
                .skip(1)
                .take(valley_cols)
                .enumerate()
                .filter(|(_, &b)| b == b'<')
                .fold(vec![0; valley_cols], |mut acc, (i, _)| {
                    acc[i] += 1;
                    acc
                }),
        );
        valley.push_blizzard_top(
            row.as_bytes()
                .iter()
                .skip(1)
                .take(valley_cols)
                .enumerate()
                .filter(|(_, &b)| b == b'^')
                .fold(vec![0; valley_cols], |mut acc, (i, _)| {
                    acc[i] += 1;
                    acc
                }),
        );
        valley.push_blizzard_down(
            row.as_bytes()
                .iter()
                .skip(1)
                .take(valley_cols)
                .enumerate()
                .filter(|(_, &b)| b == b'v')
                .fold(vec![0; valley_cols], |mut acc, (i, _)| {
                    acc[i] += 1;
                    acc
                }),
        );
    });
    println!("{valley}");
    valley
}

fn part1(input: &Valley) -> usize {
    0
}

fn main() {
    println!("Part 1: {}", part1(&decode_input(INPUT)));
}

#[cfg(test)]
mod test {
    use crate::{decode_input, part1};

    #[test]
    fn test_part1() {
        assert_eq!(3, part1(&decode_input(TEST)));
    }

    const TEST: &str = r#"
#.#####
#.....#
#>....#
#.....#
#...v.#
#.....#
#####.#    
"#;
}

const INPUT: &str = r#"
#.########################################################################################################################
#<<><>>v.<<<<.^><<<^^<><.v^vv>>^>.<.>v^<>>.<<v>v<><v>>^<v^^>.>^vv^<v>><^vv>>v><<.v<v<<^<<^v..>v<>v<<v<<><>>>^>>.v^>^vv>><#
#>^^<>.>v^^<.>>vv^v<v<.<v..v.><><><<v^<>^v>^^v<>>.vv<v>v<vvv>^<<>^<<<>v>>^^>^>^v<^vv^v<><>v^<<>^vvvvv<^>>>..>><>^^.>^^.v>#
#<v<>v<>.<<<v<^><<>vv<^>^>><v<v><^v^<^v.v^v<>.^>^<^vv<<>>^v<v<<>>^v^^v^v><<.>^^vv>>v>^>v.>>v^^>>><v^vv^v^v>^v><<>^<<v<<><#
#<<v^^<<><v^^v>v.><<^><^<^v<v^^^v^.<>>v.<<><^>vv^^<<>^>.v^<^<^<>v<v<v<v>vv^^>>v<.<^vv.><<.<^.v><^^<>><v<><^^^>.<^..<^<>^>#
#>>^>^<v><v^<v^v>vv<>>>>^v>>^<v.>^^<^<..>^vvv^v>^v>><^.>>^v^<v>>vv>>^^><.><<<^><vv><^^^<vv<v<^^<>>>vv>v>v<^^v^><<vv<.v.v<#
#>v<<.v<^><.>v^.<v<^>vvvvv<vv>>.>^.^>><^>v>>><^>v><>^<>>><v^vv<^^v^v>>>^v><^v.>^<v.<vv>^.>vv^v^^<<.<<>>^^<vv>..><vv.v^^<>#
#<^^^>^v<^><v^<>><^>^.^vvv<><vvvv^>^<>v^^vv<<>>^.v<>v>^<v<vv><<<^^>><^vvvv<>>>^>^^>^^<<^^>.vv^vv>v<<<.<^>v><^.<v<>v^.><^>#
#<>^>.>^<<><^v<><>^>>>.vv>^v<.><v^vv^<>v^v<^<>>><<..^>.v^v.vv<^<<<^.v>v.^^^>^><>><v^vv^<^v<>>>^.<<v^>>vv.^<<<^<<>.vv>^vv>#
#<>vvv>^v.^^>>^^^>.>v<>v^<<.<<<.vv.^^^^^v..v><^v^v<<>>vv<>>v^v^^.>>^<^<^^<^v>v<^>>^v<<^.><.vv>^^^^><<.^.v^v>v<>.v^v<v.<^<#
#>v^v>.v^<vv><<>v^>v<^<^>^<vvv.^.>>^^.^>^<>v<v^><^<^.^^v>^^.v.<^v>>^^>^<.^>^<.^..^<<<<^<<^>><.^<<^.<<^<<^v.<..v>v<vv^<.<<#
#>>.>^<>.>v>.^><^^v^v>.v^.>.v..>..<v.<>><v.vv<<vvv>^>>v^^^><<v><.^v>><^vv<>.>vv^v^^v.v^.v.^>^><.^<^^^>>>>^<<^^.vv><>>>>.>#
#>^>>^>vvvv<^>><^>><^v<<<<>v<.^v^^>vvvv^><<^v>^^>>v<^v><<>><>v^^>v>v^v>^..<<vv.>vv><^<<^v<>v<v^v<<<><<^>^^v.<^><v.><>..v>#
#>^^>v.^v><<^^<v^<>><v^<>>vv^>vvv>^.>>>><<v^vvv.<^>v^<>>.>>vv<v>>.<v>.<>v>..v><<<v.v>><>v<.v<<v.^v>>^><^<^<>^>><>v>.><>v>#
#>^^vv>.^.^<^><^>^.v<<<^>v^v<^vv>><vv<<<^v^<v>vvvv^v>^<..<<<..>v^.>.>^><v>^<^<^v>>^<v..<^vv..>^^vv>>v.<.^^^><>>><><^>^v.>#
#>^^>^<>^><><<v^<^^>^v<<^vv.v<^<.<<^v<<v>^^v.v>.^>v>.<><^v<^<^<vv^^v><.>v<<^<<>>vvv^v^.>>v.v>v.^v<>.<<<<<v.vv^>^<^v^^^^<.#
#><v<<<>^v<<^^v>^^v^.^<vvvv<>>><>>^<v>^v<.<>>^..<.v^v><.^<vv>v>v<^>v^.^.<>><<v.^^v>v^>..v.<v<><<^<^>>><^.^<>^>v><><>v.<^<#
#><>.<^<^^v^v><..<^>v><v>v^><.^><v>^^v^^vv^^>^<v^v><<^vv<^><><vv><vv^^^v^v^>v><>v^>><>v^<v^^>v>.>>>^>.v><>v<<<>>^<<<>vv^<#
#><<<<>^^<.^.<v^vvv<><>v<^<v<>v^^v^v.<<v.><<v^<v<<<><<v^v>v<v>>.>v<^>>><<<^v^>>v<<^vv<.^^>v>v.<^>^<^>^>v<>>>v^vv.>.v^>v..#
#>><v>.^^<>.^v^<^v^v^><^^>^>vvv<>>^><...>v^.>^.>^v>>>^>>.<<^^><^.^.v.^v<.<>^v<<>v<.v>^v>>><<.<v^<^<<.<v>v>>^^v.^v^v<<^<..#
#>>^v><<^>v^.><v^><>vv^^^^^.>^vv.^<<vv>><^^vv>>^^>.><^v>v^<>^.>v>...v<^v^>.v^<>^.>vv>vv>^><><>^<>><>v^^v^<v>>^>^<v<>^>v^<#
#.^>^.><>^<^^<v<^^v^v<<<^<<>v..^<^<.v<v<^.<<<<<>>.^.^><^.><v<.v<<v<^>^v>>v>v.^>^><v>v.>vv..<^>>>v^v<vv><^<<>><<<<>>.<<>v<#
#><>>vvv><>>^^<.<<<>>^><>>vv>.<<<>v>v<v.<vv><>.>>^>^<<v^v^>v<>>^v<v>>^.^<^^.>v>^^v^>.<^.v.<vv^^v<^v^>>^^v><^<v<>^<.<v>^<>#
#>>vv<v^vv^^^<vvv^<.v^.^>^<<^v^<v<^<>>>.vvv^<<v>v>><<^v.^vvv<.<^vv.v<^v^v<>>v.^>>v.<<>>><^.v<v.v<<^>v.<<^>^^^>^.>>^^v.<^>#
#<><>><^v^><^<.>^^v<><vv<<>v>^<vv.<^v>v^^^.<><v.v<><^^^^^^v^><>v^>>.v^<^<<v<><.>>.v^vv<.>.^^v^vvv^<^<<^^>v^v>v>.^<v>v<v<<#
#<v<^><><.>^<<^<<v><v>^<v.^<<<^v><^v>>>.vv^v<.>v><vv>>>.<^.<<^^<><>>..^^<>^v>v^v<vv.v^>>^v^>..>^<^vv.>>^<<.v>>^>>>^.v<<>>#
########################################################################################################################.#
"#;
