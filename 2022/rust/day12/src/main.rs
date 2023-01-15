use pathfinding::prelude::bfs;

#[derive(Clone, Debug, Hash, Eq, PartialEq, PartialOrd)]
struct Node {
    y: usize,
    x: usize,
}

impl Node {
    fn new(y: usize, x: usize) -> Self {
        Self { y, x }
    }
}

struct Grid {
    cells: Vec<u8>,
    width: usize,
    start: Node,
    goal: Node,
}

impl Grid {
    fn new(cells: Vec<u8>, width: usize) -> Self {
        let pos = cells.iter().position(|c| *c == b'S').unwrap();
        let start = Node::new(pos / width, pos % width);
        let pos = cells.iter().position(|c| *c == b'E').unwrap();
        let goal = Node::new(pos / width, pos % width);
        Grid {
            cells,
            width,
            start,
            goal,
        }
    }

    fn at(&self, row: usize, col: usize) -> u8 {
        self.cells[(self.width * row) + col]
    }

    fn height(&self) -> usize {
        self.cells.len() / self.width
    }

    fn manhatten_distance(&self, node: &Node) -> usize {
        self.goal.y.abs_diff(node.y) + self.goal.x.abs_diff(node.x)
    }

    fn neighbors(&self, node: &Node) -> Vec<Node> {
        let cur_grid_val = self.at(node.y, node.x);
        let mut neighbors = vec![];
        for (dy, dx) in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
            let Some(y) = node.y.checked_add_signed(dy) else { continue; };
            let Some(x) = node.x.checked_add_signed(dx) else { continue; };
            if y < self.height() && x < self.width {
                let new_grid_value = self.at(y, x);
                if cur_grid_val == new_grid_value
                    || cur_grid_val + 1 == new_grid_value
                    || cur_grid_val - 1 == new_grid_value
                    || ((cur_grid_val + 1 == b'z' || cur_grid_val == b'z')
                        && new_grid_value == b'E')
                    || (cur_grid_val == b'S' && new_grid_value == b'a')
                {
                    println!("{}", String::from_utf8_lossy(&[new_grid_value]));
                    neighbors.push(Node::new(y, x));
                }
            }
        }
        neighbors
    }
}

fn decode_input(input: &str) -> Grid {
    let (cells, width) = input.split_terminator('\n').filter(|l| !l.is_empty()).fold(
        (Vec::new(), 0),
        |mut acc, l| {
            acc.0.extend(l.bytes());
            acc.1 = l.len();
            acc
        },
    );
    Grid::new(cells, width)
}

fn find_path_to_goal(input: &Grid) -> Option<Vec<Node>> {
    let start_node = input.start.clone();
    bfs(&start_node, |n| input.neighbors(n), |n| *n == input.goal)
}

fn part1(input: &Grid) -> usize {
    let res = find_path_to_goal(input).unwrap();
    res.iter().for_each(|e| {
        print!("{}", String::from_utf8_lossy(&[input.at(e.y, e.x)]));
    });
    println!();
    res.len() - 1
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
