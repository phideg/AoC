use std::collections::HashSet;

struct Grid {
    cells: Vec<u8>,
    width: usize,
}

impl Grid {
    fn new() -> Self {
        Grid {
            cells: Vec::new(),
            width: 0,
        }
    }

    fn start(&self) -> (usize, usize) {
        let pos = self.cells.iter().position(|c| *c == b'S').unwrap();
        (pos / self.width, pos % self.width)
    }

    fn end(&self) -> (usize, usize) {
        let pos = self.cells.iter().position(|c| *c == b'E').unwrap();
        (pos / self.width, pos % self.width)
    }

    fn at(&self, row: usize, col: usize) -> u8 {
        self.cells[(self.width * row) + col]
    }

    fn height(&self) -> usize {
        self.cells.len() / self.width
    }
}

fn decode_input(input: &str) -> Grid {
    input
        .split_terminator('\n')
        .filter(|l| !l.is_empty())
        .fold(Grid::new(), |mut acc, l| {
            acc.cells.extend(l.bytes());
            acc.width = l.len();
            acc
        })
}

fn part1(input: &Grid) -> usize {
    let mut tracks = HashSet::from([(input.start(), 0)]);
    loop {
        let mut new_tracks = HashSet::new();
        for ((row, col), steps) in tracks {
            for (dy, dx) in [(-1, 0), (1, 0), (0, 1), (0, -1)] {}
        }
    }
    0
}

fn main() {
    decode_input(INPUT);
}

#[cfg(test)]
mod test {

    #[test]
    fn test_part1() {
        assert_eq!(31, part1(decode_input(TEST)));
    }

    const TEST: &str = r#"
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
"#;
}

const INPUT: &str = "";
