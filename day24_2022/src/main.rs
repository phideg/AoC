#![feature(let_chains)]
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
enum Blizzard {
    Left,
    Right,
    Top,
    Down,
}

impl Display for Blizzard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Blizzard::Left => "<",
                Blizzard::Right => ">",
                Blizzard::Top => "^",
                Blizzard::Down => "v",
            }
        )
    }
}

type BlizzardMap = HashMap<(usize, usize), Blizzard>;

#[derive(Debug)]
struct Valley {
    blizzards: BlizzardMap,
    tracks: HashMap<(usize, usize), usize>,
    entry: (usize, usize),
    exit: (usize, usize),
    rows: usize,
    cols: usize,
    minutes: usize,
}

impl Valley {
    fn new(rows: usize, cols: usize, entry: (usize, usize), exit: (usize, usize)) -> Self {
        Self {
            blizzards: BlizzardMap::new(),
            tracks: HashMap::new(),
            entry,
            exit,
            rows,
            cols,
            minutes: 0,
        }
    }

    fn insert_blizzard(&mut self, pos: (usize, usize), blizzard: Blizzard) {
        self.blizzards.insert(pos, blizzard);
    }

    fn increment_minute(&mut self) {
        self.minutes += 1;
    }

    fn get_blizzards_at(&self, pos: (usize, usize), minute: usize) -> HashSet<Blizzard> {
        let mut blizzards = HashSet::new();
        let cols_remainder = minute % self.cols;
        let rows_remainder = minute % self.rows;
        // search for blizzard from left
        let orig_left_col = if pos.1 >= cols_remainder {
            pos.1 - cols_remainder
        } else {
            self.cols - (cols_remainder - pos.1)
        };
        if let Some(blizzard) = self.blizzards.get(&(pos.0, orig_left_col)) && *blizzard == Blizzard::Right{
            blizzards.insert(blizzard.clone());
        }
        // search for blizzards from right
        let orig_right_col = if pos.1 + cols_remainder < self.cols {
            pos.1 + cols_remainder
        } else {
            cols_remainder - (self.cols - pos.1)
        };
        if let Some(blizzard) = self.blizzards.get(&(pos.0, orig_right_col)) && *blizzard == Blizzard::Left {
            blizzards.insert(blizzard.clone());
        }
        // search for blizzard from top
        let orig_top_row = if pos.0 >= rows_remainder {
            pos.0 - rows_remainder
        } else {
            self.rows - (rows_remainder - pos.0)
        };
        if let Some(blizzard) = self.blizzards.get(&(orig_top_row, pos.1)) && *blizzard == Blizzard::Down {
            blizzards.insert(blizzard.clone());
        }
        // search for blizzard from bottom
        let orig_down_row = if pos.0 + rows_remainder < self.rows {
            pos.0 + rows_remainder
        } else {
            rows_remainder - (self.rows - pos.0)
        };
        if let Some(blizzard) = self.blizzards.get(&(orig_down_row, pos.1)) && *blizzard == Blizzard::Top {
            blizzards.insert(blizzard.clone());
        }
        blizzards
    }

    fn try_move(&mut self) -> usize {
        let mut new_tracks = HashMap::new();
        for (pos, &minute) in self.tracks.iter() {
            if *pos == self.exit {
                return minute;
            }
            for delta in [(-1, 0), (1, 0), (0, 0), (0, 1), (0, -1)] {
                let Some(row) = pos.0.checked_add_signed(delta.0) else { continue; };
                let Some(col) = pos.1.checked_add_signed(delta.1) else { continue; };
                if !(row < self.rows && col < self.cols) {
                    continue;
                }
                if self.get_blizzards_at((row, col), self.minutes).len() == 0 {
                    new_tracks.insert((row, col), self.minutes);
                }
            }
        }
        // check if new track can be started
        if self.get_blizzards_at(self.entry, self.minutes).len() == 0 {
            new_tracks.insert(self.entry, self.minutes);
        }
        self.tracks = new_tracks;
        return 0;
    }

    fn reset_tracks(&mut self) {
        self.tracks.clear();
    }

    fn swap_entry_and_exit(&mut self) {
        let entry = self.entry;
        self.entry = self.exit;
        self.exit = entry;
    }
}

impl Display for Valley {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:#<1$}", "", self.cols + 2)?;
        for r in 0..self.rows {
            write!(f, "#")?;
            for c in 0..self.cols {
                let blizzards = self.get_blizzards_at((r, c), self.minutes);
                match blizzards.len() {
                    0 => {
                        if self.tracks.iter().any(|(t, _)| *t == (r, c)) {
                            write!(f, "E")?
                        } else {
                            write!(f, ".")?
                        }
                    }
                    1 => blizzards.iter().next().unwrap().fmt(f)?,
                    n => write!(f, "{}", n)?,
                }
            }
            write!(f, "#")?;
            writeln!(f)?;
        }
        writeln!(f, "{:#<1$}", "", self.cols + 2)?;
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
        valley_cols,
        (0, lines[0].chars().position(|c| c == '.').unwrap() - 1),
        (
            lines.len() - 3,
            lines[lines.len() - 1]
                .chars()
                .position(|c| c == '.')
                .unwrap()
                - 1,
        ),
    );
    lines
        .iter()
        .skip(1)
        .take(valley_rows)
        .enumerate()
        .for_each(|(r, row)| {
            row.as_bytes()
                .iter()
                .skip(1)
                .take(valley_cols)
                .enumerate()
                .filter(|(_, &b)| b != b'.')
                .for_each(|(c, col)| match col {
                    b'<' => valley.insert_blizzard((r, c), Blizzard::Left),
                    b'>' => valley.insert_blizzard((r, c), Blizzard::Right),
                    b'^' => valley.insert_blizzard((r, c), Blizzard::Top),
                    b'v' => valley.insert_blizzard((r, c), Blizzard::Down),
                    _ => panic!("unexpected input {}", String::from_utf8_lossy(&[*col])),
                });
        });
    valley
}

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
#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
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
