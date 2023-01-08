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
    let mut tracks = vec![vec![input.start()]];
    let mut steps = 0_usize;
    loop {
        let mut new_tracks = Vec::new();
        let mut new_positions = HashSet::new();
        println!("Tracks: {}", tracks.len());
        for track in tracks {
            let &(row, col) = track.last().unwrap();
            let cur_grid_val = input.at(row, col);
            if cur_grid_val == b'E' {
                return steps;
            }
            for (dy, dx) in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
                let Some(y) = row.checked_add_signed(dy) else { continue; };
                let Some(x) = col.checked_add_signed(dx) else { continue; };
                if y < input.height()
                    && x < input.width
                    && new_positions.insert((y, x))
                    && !track.contains(&(y, x))
                {
                    let new_grid_value = input.at(y, x);
                    if cur_grid_val == new_grid_value
                        || cur_grid_val + 1 == new_grid_value
                        || ((cur_grid_val + 1 == b'z' || cur_grid_val == b'z')
                            && new_grid_value == b'E')
                        || (cur_grid_val == b'S' && new_grid_value == b'a')
                    {
                        let mut new_track = track.clone();
                        new_track.push((y, x));
                        new_tracks.push(new_track);
                    }
                }
            }
        }
        assert!(!new_tracks.is_empty());
        steps += 1;
        new_tracks.iter().for_each(|v| {
            v.iter().for_each(|c| print!("{c:?}"));
            println!();
        });
        tracks = new_tracks;
    }
}

fn main() {
    println!("Part 1: {}", part1(&decode_input(INPUT)));
}

#[cfg(test)]
mod test {
    use crate::{decode_input, part1};

    #[test]
    fn test_part1() {
        assert_eq!(31, part1(&decode_input(TEST)));
    }

    const TEST: &str = r#"
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
"#;
}

const INPUT: &str = r#"
abacccaaaacccccccccccaaaaaacccccaaaaaaccccaaacccccccccccccccccccccccccccccccccccccccccccaaaaa
abaaccaaaacccccccccccaaaaaaccccccaaaaaaaaaaaaaccccccccccccccccccccccccccccccccccccccccccaaaaa
abaaccaaaacccccccccccaaaaacccccaaaaaaaaaaaaaaaccccccccccccccccccccccccccccccccccccccccccaaaaa
abccccccccccccccccccccaaaaacccaaaaaaaaaaaaaaaacccccccccccccccccccccccccccaaaccccccccccccaaaaa
abccccccccccccccccccccaacaacccaaaaaaaaccaaaaaccccccccccccccccccccccccccccaaaccccccccccccaccaa
abcccccccccccccaacccaaaccccccaaaaaaaaaccaaaaaccccccccccccccccccccccccccccccacccccccccccccccca
abcccccccccccaaaaaaccaaaccacccccaaaaaaacccccccccccccccccccccccccciiiicccccccddddddccccccccccc
abcccccccccccaaaaaaccaaaaaaaccccaaaaaacccccaacccccccaaaccccccccciiiiiiiicccdddddddddacaaccccc
abccccccccccccaaaaaaaaaaaaacccccaaaaaaacaaaacccccccaaaacccccccchhiiiiiiiiicddddddddddaaaccccc
abcccccccccccaaaaaaaaaaaaaacccccccaaacccaaaaaacccccaaaaccccccchhhipppppiiiijjjjjjjddddaaccccc
abcccccccccccaaaaaaaaaaaaaaccccccccccccccaaaaaccccccaaaccccccchhhpppppppiijjjjjjjjjddeeaccccc
abcccccccccccccccccaaaaaaaacccccccccccccaaaaaccccccccccccccccchhppppppppppjjqqqjjjjjeeeaacccc
abccccccccccccccccccaaaaaaaacccccccccccccccaacccccccccccccccchhhpppuuuupppqqqqqqqjjjeeeaacccc
abcccccccccccccccccccaacccacccccccccccccccccccccccccccccccccchhhopuuuuuuppqqqqqqqjjjeeecccccc
abacccccccccccccaaacaaaccccccccccccccccccccccccccccaaccccccchhhhoouuuuuuuqvvvvvqqqjkeeecccccc
abaccccccccccccaaaaaacccccaaccccccccccccccccccccccaaaccccccchhhooouuuxxxuvvvvvvqqqkkeeecccccc
abaccccccccccccaaaaaacccaaaaaaccccccccccccccccccaaaaaaaaccchhhhooouuxxxxuvyyyvvqqqkkeeecccccc
abcccccccccccccaaaaacccaaaaaaaccccccccccccccccccaaaaaaaaccjjhooooouuxxxxyyyyyvvqqqkkeeecccccc
abccccccccccccccaaaaaacaaaaaaaccccccccaaaccccccccaaaaaaccjjjooootuuuxxxxyyyyyvvqqkkkeeecccccc
abccccccccccccccaaaaaaaaaaaaacccccccccaaaacccccccaaaaaacjjjooootttuxxxxxyyyyvvrrrkkkeeecccccc
SbccccccccccccccccccaaaaaaaaacccccccccaaaacccccccaaaaaacjjjoootttxxxEzzzzyyvvvrrrkkkfffcccccc
abcccccccccccaaacccccaaaaaaacaaaccccccaaaccccccccaaccaacjjjoootttxxxxxyyyyyyvvvrrkkkfffcccccc
abcccccccccaaaaaacccaaaaaacccaaacacccaacccccccccccccccccjjjoootttxxxxyxyyyyyywvvrrkkkfffccccc
abcccccccccaaaaaacccaaaaaaaaaaaaaaaccaaacaaacccccaacccccjjjnnnttttxxxxyyyyyyywwwrrkkkfffccccc
abcaacacccccaaaaacccaaacaaaaaaaaaaaccaaaaaaacccccaacaaacjjjnnnntttttxxyywwwwwwwwrrrlkfffccccc
abcaaaaccccaaaaacccccccccaacaaaaaaccccaaaaaacccccaaaaacccjjjnnnnnttttwwywwwwwwwrrrrllfffccccc
abaaaaaccccaaaaaccccccaaaaaccaaaaacaaaaaaaaccccaaaaaaccccjjjjinnnntttwwwwwsssrrrrrllllffccccc
abaaaaaaccccccccccccccaaaaacaaaaaacaaaaaaaaacccaaaaaaacccciiiiinnnntswwwwssssrrrrrlllfffccccc
abacaaaaccccccccccccccaaaaaacaaccccaaaaaaaaaaccccaaaaaaccccciiiinnnssswwsssssllllllllfffccccc
abccaaccccccccccccccccaaaaaaccccccccccaaacaaaccccaaccaacccccciiiinnsssssssmmllllllllfffaacccc
abccccccccccccccccccccaaaaaaccccccccccaaaccccccccaaccccccccccciiinnmsssssmmmmlllllgggffaacccc
abcccccccccccccccaccccccaaacccccccccccaaccccccccccccccccccccccciiimmmsssmmmmmgggggggggaaacccc
abcccccccccaaaaaaaaccccccccccccccccccccccccccccaaaaaccccccccccciiimmmmmmmmmgggggggggaaacccccc
abccccccccccaaaaaaccccccccccccccccccaacccccccccaaaaacccccccccccciiimmmmmmmhhggggcaaaaaaaccccc
abccccccccccaaaaaacccccccccccccccccaacccccccccaaaaaacccccccccccciihhmmmmhhhhgccccccccaacccccc
abccccaacaaaaaaaaaaccccccccccccccccaaaccccccccaaaaaaccccccccccccchhhhhhhhhhhaaccccccccccccccc
abccccaaaaaaaaaaaaaaccccccccccaaccaaaaccccccccaaaaaacccaaacccccccchhhhhhhhaaaaccccccccccccccc
abcccaaaaaaaaaaaaaaaccccccccaaaaaacaaaacacaccccaaaccccaaaacccccccccchhhhccccaaccccccccccaaaca
abcccaaaaaacacaaacccccccccccaaaaaaaaaaaaaaacccccccccccaaaacccccccccccaaaccccccccccccccccaaaaa
abcccccaaaacccaaaccccccccccaaaaaaaaaaaaaaaaccccccccccccaaacccccccccccaaacccccccccccccccccaaaa
abcccccaacccccaacccccccccccaaaaaaaaaaaaaccccccccccccccccccccccccccccccccccccccccccccccccaaaaa
"#;
