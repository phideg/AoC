#![feature(iter_array_chunks)]
fn parse_stacks(lines: &str) -> Vec<Vec<char>> {
    let mut stack_lines = lines.split_terminator('\n').rev();
    let stack_header = stack_lines.next().unwrap();
    let no_of_stacks: usize = stack_header.split_whitespace().last().unwrap().parse().unwrap();
    let mut stacks = vec![Vec::new(); no_of_stacks];
    stack_lines.for_each(|l| {
        let mut chunks = l.chars().array_chunks();
        let mut i = 0;
        while let Some([_,e,_,_]) = chunks.next() {
            if !e.is_whitespace() {
                stacks[i].push(e);
            }
            i += 1;
        }
        if let Some(v) = chunks.into_remainder().unwrap().nth_back(1) {
            stacks[i].push(v);
        }
    });
    stacks
}

fn parse_moves(lines: &str) -> Vec<(usize, usize, usize)> {
    lines.split_terminator('\n')
    .map(|l| l.split_whitespace().collect::<Vec<&str>>())
    .map(|v| (v[1].parse().unwrap(), v[3].parse().unwrap(), v[5].parse().unwrap()))
    .collect()
}

fn part1(mut stacks: Vec<Vec<char>>, moves: &[(usize, usize, usize)]) {
    moves.iter().for_each(|(no, from, to)| {
        for _ in 0..*no {
            let mv = stacks[*from - 1].pop().unwrap();
            stacks[*to - 1].push(mv);
        }
    });
    println!("{}", stacks.iter().fold(String::new(), |mut acc, stack| { acc.push(*stack.last().unwrap()); acc }))
}

fn part2(mut stacks: Vec<Vec<char>>, moves: &[(usize, usize, usize)]) {
    moves.iter().for_each(|(no, from, to)| {
        let from_len = stacks[*from-1].len();
        let p = stacks[*from - 1].split_off(from_len - *no);
        stacks[*to - 1].extend_from_slice(&p);
    });
    println!("{}", stacks.iter().fold(String::new(), |mut acc, stack| { acc.push(*stack.last().unwrap()); acc }))
}

fn main() {
    let end_stack = INPUT.find("\n\n").unwrap();
    let stacks = parse_stacks(&INPUT[..end_stack]);
    let moves = parse_moves(&INPUT[end_stack+2..]);
    part1(stacks.clone(), &moves);
    part2(stacks, &moves);
}

const TEST: &str = r#"
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#;

const INPUT: &str = r#"
        [M]     [B]             [N]
[T]     [H]     [V] [Q]         [H]
[Q]     [N]     [H] [W] [T]     [Q]
[V]     [P] [F] [Q] [P] [C]     [R]
[C]     [D] [T] [N] [N] [L] [S] [J]
[D] [V] [W] [R] [M] [G] [R] [N] [D]
[S] [F] [Q] [Q] [F] [F] [F] [Z] [S]
[N] [M] [F] [D] [R] [C] [W] [T] [M]
 1   2   3   4   5   6   7   8   9

move 1 from 8 to 7
move 1 from 2 to 7
move 6 from 9 to 8
move 1 from 9 to 1
move 1 from 9 to 1
move 3 from 3 to 6
move 3 from 3 to 9
move 1 from 9 to 2
move 5 from 7 to 9
move 9 from 1 to 6
move 3 from 4 to 9
move 2 from 9 to 2
move 1 from 4 to 2
move 1 from 3 to 9
move 8 from 9 to 4
move 14 from 6 to 7
move 1 from 3 to 2
move 5 from 4 to 2
move 5 from 5 to 7
move 4 from 2 to 1
move 2 from 4 to 9
move 1 from 4 to 3
move 3 from 5 to 7
move 1 from 8 to 6
move 2 from 8 to 7
move 2 from 1 to 2
move 1 from 9 to 7
move 2 from 1 to 3
move 5 from 6 to 5
move 4 from 5 to 7
move 3 from 8 to 4
move 20 from 7 to 1
move 11 from 7 to 5
move 1 from 6 to 9
move 3 from 9 to 2
move 12 from 1 to 9
move 2 from 8 to 3
move 4 from 2 to 8
move 8 from 2 to 1
move 4 from 8 to 9
move 1 from 2 to 5
move 12 from 9 to 7
move 4 from 4 to 9
move 4 from 9 to 5
move 13 from 5 to 4
move 4 from 4 to 7
move 1 from 7 to 9
move 2 from 9 to 5
move 9 from 1 to 2
move 1 from 8 to 3
move 5 from 4 to 2
move 1 from 3 to 6
move 7 from 2 to 8
move 6 from 1 to 6
move 6 from 8 to 7
move 6 from 2 to 1
move 3 from 9 to 3
move 7 from 3 to 7
move 4 from 4 to 9
move 1 from 8 to 9
move 1 from 3 to 9
move 1 from 2 to 4
move 1 from 9 to 6
move 5 from 1 to 9
move 1 from 4 to 9
move 2 from 9 to 1
move 8 from 6 to 7
move 4 from 9 to 7
move 2 from 5 to 2
move 2 from 1 to 9
move 14 from 7 to 4
move 22 from 7 to 2
move 2 from 7 to 4
move 3 from 7 to 5
move 9 from 4 to 7
move 6 from 2 to 4
move 8 from 4 to 3
move 14 from 2 to 9
move 2 from 3 to 9
move 3 from 2 to 9
move 4 from 4 to 2
move 1 from 4 to 5
move 1 from 1 to 4
move 5 from 7 to 8
move 1 from 1 to 3
move 4 from 5 to 2
move 6 from 3 to 9
move 1 from 3 to 4
move 4 from 8 to 9
move 2 from 4 to 6
move 4 from 5 to 3
move 1 from 7 to 6
move 1 from 8 to 5
move 3 from 3 to 1
move 33 from 9 to 5
move 5 from 2 to 1
move 1 from 3 to 5
move 1 from 7 to 6
move 18 from 5 to 1
move 1 from 2 to 8
move 6 from 5 to 4
move 1 from 8 to 7
move 2 from 4 to 1
move 4 from 1 to 2
move 19 from 1 to 2
move 4 from 6 to 8
move 4 from 1 to 8
move 14 from 2 to 9
move 5 from 2 to 4
move 1 from 8 to 2
move 8 from 2 to 5
move 5 from 8 to 4
move 4 from 9 to 7
move 1 from 8 to 1
move 16 from 5 to 4
move 15 from 4 to 5
move 1 from 9 to 5
move 5 from 7 to 6
move 2 from 7 to 6
move 1 from 1 to 9
move 7 from 6 to 7
move 1 from 8 to 5
move 1 from 1 to 9
move 12 from 5 to 7
move 7 from 5 to 9
move 12 from 7 to 2
move 1 from 7 to 4
move 7 from 4 to 7
move 2 from 9 to 4
move 5 from 4 to 9
move 8 from 2 to 3
move 4 from 2 to 4
move 9 from 4 to 8
move 6 from 3 to 5
move 8 from 7 to 3
move 1 from 4 to 3
move 7 from 8 to 9
move 4 from 5 to 4
move 6 from 3 to 1
move 4 from 3 to 4
move 1 from 3 to 6
move 6 from 4 to 9
move 1 from 6 to 5
move 17 from 9 to 4
move 3 from 7 to 3
move 1 from 7 to 9
move 2 from 5 to 3
move 2 from 1 to 3
move 2 from 8 to 9
move 1 from 5 to 1
move 14 from 4 to 5
move 2 from 3 to 2
move 1 from 7 to 6
move 10 from 9 to 4
move 12 from 9 to 4
move 9 from 4 to 5
move 1 from 2 to 9
move 13 from 5 to 9
move 2 from 5 to 1
move 1 from 2 to 9
move 3 from 4 to 2
move 12 from 4 to 7
move 8 from 5 to 7
move 1 from 1 to 9
move 1 from 6 to 4
move 1 from 5 to 4
move 1 from 4 to 8
move 5 from 3 to 4
move 10 from 9 to 6
move 3 from 6 to 2
move 7 from 6 to 5
move 6 from 5 to 4
move 1 from 8 to 5
move 1 from 1 to 4
move 2 from 7 to 2
move 5 from 4 to 9
move 2 from 5 to 8
move 1 from 1 to 3
move 2 from 1 to 7
move 6 from 7 to 9
move 9 from 9 to 8
move 1 from 1 to 3
move 4 from 2 to 7
move 11 from 7 to 3
move 11 from 8 to 6
move 7 from 3 to 1
move 4 from 7 to 2
move 3 from 2 to 9
move 8 from 1 to 5
move 2 from 7 to 5
move 2 from 2 to 9
move 2 from 3 to 9
move 11 from 4 to 7
move 7 from 9 to 5
move 6 from 6 to 5
move 2 from 2 to 9
move 1 from 2 to 3
move 6 from 9 to 4
move 3 from 9 to 1
move 4 from 3 to 5
move 6 from 7 to 1
move 2 from 6 to 3
move 2 from 9 to 2
move 3 from 3 to 2
move 3 from 6 to 8
move 2 from 7 to 5
move 20 from 5 to 6
move 8 from 5 to 1
move 1 from 5 to 9
move 2 from 8 to 4
move 1 from 8 to 7
move 16 from 1 to 8
move 8 from 8 to 9
move 4 from 2 to 4
move 1 from 1 to 5
move 1 from 5 to 4
move 3 from 8 to 4
move 14 from 4 to 6
move 5 from 8 to 7
move 6 from 7 to 8
move 29 from 6 to 2
move 3 from 9 to 8
move 21 from 2 to 3
move 1 from 8 to 3
move 6 from 9 to 4
move 8 from 3 to 5
move 7 from 8 to 4
move 7 from 3 to 9
move 3 from 7 to 2
move 12 from 4 to 8
move 2 from 3 to 1
move 2 from 9 to 1
move 1 from 6 to 7
move 1 from 7 to 6
move 1 from 6 to 3
move 3 from 1 to 8
move 2 from 4 to 1
move 4 from 6 to 1
move 5 from 2 to 7
move 1 from 1 to 2
move 5 from 1 to 2
move 2 from 8 to 1
move 1 from 4 to 5
move 9 from 8 to 4
move 3 from 7 to 9
move 7 from 5 to 7
move 2 from 5 to 9
move 4 from 9 to 2
move 3 from 3 to 2
move 5 from 2 to 7
move 2 from 8 to 2
move 2 from 7 to 3
move 1 from 8 to 6
move 2 from 1 to 2
move 1 from 6 to 7
move 1 from 8 to 1
move 12 from 7 to 1
move 5 from 2 to 7
move 7 from 4 to 2
move 2 from 4 to 1
move 5 from 3 to 8
move 7 from 1 to 9
move 4 from 7 to 1
move 7 from 1 to 5
move 12 from 9 to 2
move 27 from 2 to 4
move 3 from 8 to 9
move 6 from 2 to 5
move 6 from 1 to 8
move 1 from 7 to 6
move 9 from 5 to 2
move 3 from 9 to 2
move 13 from 4 to 5
move 10 from 2 to 7
move 1 from 9 to 8
move 11 from 5 to 7
move 1 from 8 to 7
move 1 from 2 to 6
move 13 from 4 to 3
move 23 from 7 to 4
move 1 from 6 to 9
move 1 from 2 to 4
move 7 from 3 to 5
move 1 from 9 to 8
move 19 from 4 to 1
move 2 from 4 to 1
move 1 from 7 to 6
move 1 from 4 to 5
move 1 from 5 to 7
move 11 from 5 to 1
move 2 from 5 to 4
move 2 from 6 to 9
move 3 from 8 to 2
move 2 from 8 to 1
move 3 from 2 to 1
move 1 from 9 to 5
move 6 from 1 to 3
move 1 from 9 to 7
move 2 from 7 to 5
move 2 from 8 to 6
move 1 from 3 to 2
move 2 from 8 to 5
move 1 from 2 to 1
move 3 from 4 to 1
move 3 from 5 to 1
move 2 from 5 to 1
move 2 from 6 to 9
move 1 from 9 to 6
move 1 from 4 to 5
move 1 from 9 to 8
move 1 from 8 to 6
move 8 from 1 to 6
move 7 from 1 to 8
move 9 from 1 to 6
move 1 from 5 to 3
move 3 from 8 to 4
move 11 from 3 to 4
move 1 from 3 to 6
move 10 from 6 to 8
move 13 from 1 to 6
move 3 from 4 to 5
move 7 from 8 to 6
move 3 from 8 to 5
move 6 from 5 to 3
move 22 from 6 to 9
move 4 from 3 to 6
move 4 from 9 to 5
move 1 from 1 to 5
move 2 from 3 to 4
move 2 from 1 to 5
move 1 from 9 to 2
move 5 from 8 to 3
move 2 from 9 to 2
move 11 from 6 to 9
move 3 from 2 to 7
move 1 from 6 to 7
move 12 from 9 to 8
move 4 from 7 to 1
move 12 from 4 to 8
move 2 from 4 to 7
move 1 from 1 to 8
move 1 from 5 to 1
move 19 from 8 to 4
move 4 from 5 to 1
move 1 from 7 to 4
move 1 from 7 to 1
move 3 from 3 to 4
move 2 from 8 to 4
move 1 from 5 to 7
move 1 from 7 to 9
move 8 from 1 to 8
move 1 from 1 to 4
move 1 from 3 to 9
move 1 from 3 to 5
move 1 from 5 to 2
move 7 from 8 to 7
move 16 from 4 to 7
move 1 from 7 to 4
move 3 from 8 to 2
move 14 from 7 to 4
move 1 from 5 to 8
move 5 from 7 to 5
move 16 from 4 to 5
move 3 from 5 to 4
move 3 from 2 to 1
move 1 from 7 to 9
move 11 from 4 to 2
move 3 from 8 to 6
move 2 from 1 to 8
move 1 from 4 to 9
move 18 from 5 to 1
move 1 from 8 to 7
move 3 from 7 to 9
move 18 from 9 to 3
move 3 from 6 to 9
move 7 from 1 to 6
move 1 from 8 to 4
move 1 from 4 to 9
move 3 from 6 to 4
move 5 from 9 to 2
move 2 from 4 to 7
move 7 from 2 to 8
move 1 from 7 to 3
move 2 from 6 to 8
move 1 from 9 to 5
move 1 from 6 to 8
move 1 from 4 to 8
move 1 from 5 to 3
move 1 from 7 to 5
move 8 from 8 to 7
move 10 from 2 to 6
move 1 from 9 to 3
move 6 from 6 to 2
move 5 from 6 to 2
move 7 from 2 to 7
move 12 from 1 to 6
move 2 from 2 to 1
move 1 from 2 to 5
move 4 from 7 to 6
move 12 from 3 to 1
move 2 from 7 to 2
move 9 from 3 to 8
move 1 from 2 to 6
move 1 from 5 to 4
move 9 from 6 to 5
move 1 from 7 to 6
move 1 from 4 to 9
move 9 from 6 to 7
move 7 from 8 to 3
move 6 from 3 to 1
move 4 from 8 to 3
move 5 from 3 to 1
move 1 from 9 to 8
move 2 from 8 to 9
move 5 from 5 to 7
move 14 from 7 to 8
move 1 from 9 to 4
move 2 from 2 to 1
move 3 from 5 to 3
move 2 from 3 to 1
move 1 from 4 to 6
move 6 from 8 to 6
move 6 from 8 to 3
move 3 from 6 to 1
move 2 from 8 to 9
move 19 from 1 to 6
move 3 from 9 to 3
move 6 from 3 to 4
move 6 from 6 to 2
move 4 from 3 to 9
move 1 from 7 to 9
move 2 from 5 to 7
move 5 from 9 to 6
move 6 from 7 to 2
move 11 from 2 to 5
move 2 from 7 to 4
move 4 from 4 to 3
move 2 from 4 to 8
move 12 from 1 to 2
move 1 from 8 to 2
move 8 from 5 to 7
move 2 from 4 to 9
move 2 from 7 to 1
move 4 from 2 to 3
move 1 from 8 to 6
move 1 from 1 to 5
move 2 from 9 to 1
move 2 from 7 to 3
move 2 from 5 to 2
move 1 from 5 to 7
move 2 from 7 to 8
move 1 from 5 to 7
move 5 from 3 to 4
move 3 from 1 to 7
move 1 from 2 to 4
move 15 from 6 to 1
move 4 from 4 to 1
move 4 from 2 to 3
move 8 from 3 to 2
move 5 from 2 to 4
move 1 from 8 to 6
move 1 from 8 to 9
move 1 from 3 to 1
move 3 from 7 to 3
move 5 from 7 to 6
move 4 from 2 to 9
move 6 from 2 to 6
move 4 from 9 to 6
move 12 from 1 to 5
move 6 from 4 to 1
move 1 from 3 to 6
move 4 from 5 to 8
move 7 from 5 to 3
move 3 from 8 to 2
move 1 from 2 to 3
move 1 from 9 to 5
move 1 from 4 to 5
move 1 from 8 to 5
move 8 from 6 to 9
move 10 from 1 to 4
move 3 from 6 to 1
move 9 from 3 to 6
move 1 from 3 to 8
move 1 from 2 to 4
move 6 from 9 to 1
move 1 from 1 to 4
move 10 from 1 to 6
move 1 from 8 to 6
move 13 from 6 to 7
move 1 from 2 to 1
move 1 from 9 to 6
move 9 from 7 to 5
move 1 from 9 to 4
move 3 from 7 to 1
move 3 from 5 to 6
move 10 from 4 to 7
move 5 from 6 to 5
move 3 from 4 to 5
move 13 from 6 to 9
move 7 from 5 to 3
move 6 from 3 to 2
move 5 from 6 to 4
move 4 from 2 to 8
"#;
