#![feature(exclusive_range_pattern)]
#![feature(slice_as_chunks)]
#![feature(let_chains)]

use std::{borrow::Cow, cmp::Ordering};

#[derive(Debug, PartialEq, Clone)]
enum Element {
    ListStart,
    Value(i32),
    ListEnd,
}

fn map_line(line: &str) -> Vec<Element> {
    let mut number_start = None;
    line.as_bytes()
        .iter()
        .enumerate()
        .fold(vec![], |mut acc, (pos, char)| {
            match char {
                b'[' => {
                    acc.push(Element::ListStart);
                }
                b']' | b',' => {
                    if let Some(start_pos) = number_start {
                        acc.push(Element::Value(line[start_pos..pos].parse().unwrap()));
                        number_start = None;
                    }
                    if *char == b']' {
                        acc.push(Element::ListEnd);
                    }
                }
                b'0'..=b'9' => {
                    if number_start.is_none() {
                        number_start = Some(pos);
                    }
                }
                _ => panic!("invalid input {char}"),
            }
            acc
        })
}

fn decode_input(input: &str) -> Vec<Vec<Element>> {
    let input = input
        .split_terminator('\n')
        .filter(|line| !line.is_empty())
        .map(map_line)
        .collect::<Vec<Vec<Element>>>();
    input
}

fn cmp_lists(mut left: Cow<[Element]>, mut right: Cow<[Element]>) -> Ordering {
    let mut order = Ordering::Equal;
    let mut li = 0;
    let mut ri = 0;
    loop {
        let left_element = if li < left.len() {
            Some(left[li].clone())
        } else {
            None
        };
        let right_element = if ri < right.len() {
            Some(right[ri].clone())
        } else {
            None
        };
        match (left_element, right_element) {
            (None, None) => {
                break;
            }
            (Some(Element::ListStart), Some(Element::ListStart)) => {},
            (Some(Element::ListEnd), Some(Element::ListEnd)) => {},
            (Some(Element::Value(l_val)), Some(Element::Value(r_val))) => {
                if l_val < r_val {
                    order = Ordering::Less;
                    break;
               } else if l_val > r_val {
                    order = Ordering::Greater;
                    break;
                }
            }
            (Some(Element::ListStart), Some(Element::Value(_))) => {
                li += 1;
                right.to_mut().insert(ri+1, Element::ListEnd);
                continue;
            }
            (Some(Element::Value(_)), Some(Element::ListStart)) => {
                ri += 1;
                left.to_mut().insert(li+1, Element::ListEnd);
                continue;
            }
            (Some(Element::ListEnd), Some(Element::ListStart)) |
                // [[[],7,5,6,[]]
                // [[[[4]]]]
            (Some(Element::ListEnd), Some(Element::Value(_))) => {
                order = Ordering::Less;
                break;
            }
            (Some(Element::ListStart), Some(Element::ListEnd)) |
            (Some(Element::Value(_)), Some(Element::ListEnd)) |
            (Some(Element::Value(_)), None) => {
                order = Ordering::Greater;
                break;
            }
            (le, re) => {
                dbg!(le);
                dbg!(re);
                unreachable!()
            },
        }
        li += 1;
        ri += 1;
    }
    order
}

#[allow(dead_code)]
fn print_lists(input: &[Vec<Element>]) {
    input.iter().for_each(|(v)| {
        v.iter().for_each(|e| {
            match e {
                Element::ListStart => print!("["),
                Element::ListEnd => print!("],"),
                Element::Value(val) => print!("{val},"),
            };
        });
        println!();
    });
}

fn part1(input: &[Vec<Element>]) -> usize {
    let mut left = 0;
    let mut right = 1;
    let mut current_pair = 1;
    let mut pair_ok_count = 0;
    while right < input.len() {
        if cmp_lists(Cow::from(&input[left]), Cow::from(&input[right])) == Ordering::Less {
            pair_ok_count += current_pair;
        }
        left += 2;
        right += 2;
        current_pair += 1;
    }
    pair_ok_count
}

fn part2(input: &mut Vec<Vec<Element>>) -> usize {
    let divider1 = vec![
        Element::ListStart,
        Element::ListStart,
        Element::Value(2),
        Element::ListEnd,
        Element::ListEnd,
    ];
    let divider2 = vec![
        Element::ListStart,
        Element::ListStart,
        Element::Value(6),
        Element::ListEnd,
        Element::ListEnd,
    ];
    input.push(divider1.clone());
    input.push(divider2.clone());
    input.sort_unstable_by(|l, r| cmp_lists(Cow::from(l), Cow::from(r)));
    // print_lists(input);
    input.iter().enumerate().fold(1, |acc, (i, l)| {
        if l == &divider1 || l == &divider2 {
            acc * (i + 1)
        } else {
            acc
        }
    })
}

fn main() {
    let mut input = decode_input(INPUT);
    println!("Part 1: {}", part1(&mut input));
    println!("Part 2: {}", part2(&mut input));
}

#[cfg(test)]
mod test {

    #[test]
    fn test_part1() {
        let mut input = super::decode_input(TEST);
        assert_eq!(13_usize, super::part1(&mut input));
    }

    #[test]
    fn test_part1_special() {
        let mut input = super::decode_input("\n[[8,[[7]]]]\n[[[[8]]]]");
        assert_eq!(0_usize, super::part1(&mut input));
    }

    #[test]
    fn test_part1_special2() {
        let mut input = super::decode_input("\n[[[[1],9],[[],0,3,5,4],[7,10,[]],2],[[[3],9,6,1],[],[[],[8,3,7,1]],7]]\n[[[9,3,[4,2]],4,6]]");
        assert_eq!(1_usize, super::part1(&mut input));
    }

    #[test]
    fn test_part2() {
        let mut input = super::decode_input(TEST);
        assert_eq!(140_usize, super::part2(&mut input));
    }

    const TEST: &str = r#"
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
"#;
}

const INPUT: &str = r#"
[[[8,[4,3,4]],[[4,8,4,0,4],[3],10,[10,10,5,5,7]],10],[2,[],6,9]]
[[[6],[],[[]],[5,4,6,9]],[[[9,0,10],[7,2,7]],8,9,0,[[2,9,1,3,5],1,[],10,[]]],[9]]

[[],[[6,8,0,5]]]
[[3],[3,[]],[[4]]]

[[8,[10]],[1,10,[1,[4,3,1,6],[],[5]]],[3,[1]],[1,8,2,[3,4]]]
[[1]]

[[9,[],[]],[7,0,[6,4,7],7],[[[0,1,3]],2,6,5],[10,[9,10,9]]]
[[10,[[2],[0]],7,5,[[3,6,10,1,3],[10,6],0,[3,7]]],[[1],[],[[8,8,10,8],[],[6,5,8,3]],[[5,10,7],[1,2,4]]],[[8,2,0],[3,[]]],[],[[0]]]

[[[[3]],9]]
[[],[2],[]]

[[9,[10]],[[],[],5,3,8],[]]
[[[[9,5],[1,5,9,1,8],1]],[[5,[]],[[4,7,0],[3,10],[4,3,8]],[8,[9,0],7,[2,6,6,7],[10,2,5,4,10]],4,5],[10,[0,9],2],[2,[[0,3,2,5,6]],[],0,[5]],[6,3,3,[]]]

[[[[5],[0,7,4,1,10],[3,10],[3,2,10,10]],[[7,5,10,6,9]],2,[[8,7,0],[1,6]]]]
[[[7,7],1,[7,2,1],[1,7,[10,10,4]],10]]

[[],[7,0,6,[1],[3,6,3,1]]]
[[7,10,[0],[],[]],[10,1,1,5,2],[],[0,[],5],[[[2,0,0,1,1],3],[0,[8,0,3,8,6],[1,1,1,8],[2,6,4,6,10],10],[[],10,[0,0,0,3,10]]]]

[[9,[[],7,5,[5,10],10],[4,2]],[4,2,3,[[0,9,3],[],2,[4]],2]]
[[4,3],[[0,[]],[6,8,[],0,[0]],4]]

[3,3,9,0,9]
[3,3,9,0]

[[[6,3,4],0,[[],8,1,[7,5],[6,2,1]],8],[8,10,6,[6,6,8,4],[8,[4,3]]],[10,[10,[9,1],[6,3,0,6,4]],3,[],[[0,1,0],3,2,6,1]],[[0,10,[8,5,2]]],[0,[]]]
[[[[3,9],[1,6],[]],0,[],[[10,8,2,5,1],[8],[1,1],[5]]]]

[[[[],8],[],[[10,8]]],[1,5,[[9,7],7,[],[3],10],10],[[8,10,[10,3,5],10,[6,10,6]],[]],[],[3,[[8,6,2,2,8],[7,5,10],8,[0,8]]]]
[[6,8]]

[[3],[],[[[],8,[10,3,1],9,[2,2,4,4]],[0],5,[8,[],5,10,4]],[5,4,[],[6],7]]
[[],[[],[[7,3,0,5],7,8,1,[9,7]]],[[[7,10,8]],[[2,1]],[8,[6,1,8,1,6],7,5,[1]]]]

[[[2,4,8,[]],[]]]
[[5,3,5,10],[[]],[7,0,10,10,8],[9,1,8,[10,3],9],[[]]]

[[[7,[],10,3,7],1,[6,[4,4,4,8],9],2],[],[[9,[],[2]],[5,1,[7,1,0],[]]],[[[9],3,4,1],1],[[5],1,7,2]]
[[7,[[8,10,4,7],[6,4,2,1],[9,7,6,8,6]],6],[[[10]],0]]

[[[[6,3,4,6,1],[5],6,[3,6,7,3],[2]]],[],[5,[0,[4,2,8,1,10],5,4],7],[[3,9,[3,6,0,4],[2,9]],7,1,[[0,9]],[4,[0,3,3,4,0]]]]
[[8,5,[9,0],8,3],[2,6],[],[10,[0],8,1,10]]

[[8,5,0],[9,0,7],[0,0,7]]
[[[10,[8,9]],[[],[2,6,3]],[[6,8,5],[3,1,2,2,1],2,[9,7,3,8],[0,10]],3,[9]],[[],[[0,8,9,7,7],[7,10],[8,0],5],5],[[[],5,9]],[[4,[1,8],3],[5,[4,8]]],[[[4],[8,0,5],4,[10,7,10],[]]]]

[[1,[5,[],[10,3,7],7,9],4],[[[10,5,9,10,1]]],[6]]
[[[[],1,10,[10,10]],[8,1,[0],5],[]],[]]

[[8,6]]
[[0],[[[8,3,5],4,[]],9]]

[[1,0,[[2,5,5]],2]]
[[1,1,1],[0,1,7,10]]

[[],[[[1,8,6,10],10,2],8,[]]]
[[7],[[0,8,5],10],[[1],4,2],[7]]

[[[],5,0,[]],[1,3,9,3],[6,[[6,6]],[[8,1],[3,8],0]],[]]
[[6,7,10,[]],[4,[7,10,[3,3,2,4,2],[9,4,3,1],[5,8,0]],[[0,9,10],9,[2],[5,10],5]],[10,3,8,0,[10,6]]]

[[[]]]
[[0,[]],[9],[],[[[1,3,6]],[[1]],6,0]]

[[[],[]],[1,[7,[7,10,0,10],[7,6],4,[5,8,7,3]],9,[[9,2,3,5]],[[1,5,1],0]],[5,[[]],[[2,9,2,0,10],5,4]]]
[[[],[4,[0,0,4,4,2],1],[6,[],[7,3,1,10,6]]],[[9,4,[9,9,8,5],9]],[[0,[3],[4,4],[4],[]],10,[3,[3,3],[6,2,3,2,1],[]],10,7]]

[[1,1]]
[[4,[]],[[[1,0,0,3]],4,[[7,1,0,10,3]],[],[[7,2,2,0]]],[[[8]],2,7],[[[6,8,3,3,6],[1,9,0,8,9],[6,7,0]]],[]]

[[3,[3,9,[4,7,5,8]],4,[],2],[[[3,0],[5,5,4,7]]],[[[7,4,1,9,5],5],[[0,5,1,5,9],0],[[10,9,10,10]],5,8],[[[1,1],[],[]],[[],[3,5,0,2]]],[[4,[2],5,10],[],1,[],[2,[8,1,0],[]]]]
[[3,0,[9],[[],2,[7,5],[9,2,9,3],5],5],[6,8,0,1,8]]

[[[5,[8,6,2,3]],[[3]],9,2],[],[[],9,[5]]]
[[10,[8,9,[],9,0],5,10],[1,9,9,[[4,0,4,10,8],10,[9,5,8,1,4]],[[10,7]]],[[],8,[0],[[2,8],4,[2,0,3]],1],[10,10,8,3,6]]

[[3,5,1,[2]],[],[],[5]]
[[2],[8,9,10,[9,[],6,4,[5,7]]],[3],[1],[]]

[[5,4,10,[],[[8],[2,5,6,0,1],[5,8,6,8],8]],[5],[3,5,5,2],[[8,10,[4,3],0],[[4,9],8],9,5,[8,[0,6,6,5,8],[]]],[]]
[[[[1,9,2],6,6,9],[[],[6,8,1],[9,8,6,9]],2],[[[8,9],4,9,[2,7,7,10],[]],2,5,7],[7,[[7,3,1],[7,1,0],[]]],[8,[],[2,9,10,[]],3],[6,3,[0,1,8],9]]

[[8,[8,[9,10,5,9]],[[6],9,[5,3,7,8,2]],1,3],[8,5],[[[4,4,7,5],10],0,[6,[0],[],10,4],3]]
[[[[4,0,0]],8,10],[[3,1,6],[5,4],0,[[3,2,4,4,1]]]]

[[],[[[6,6,7],1,[],[1,7]]],[7]]
[[[1,[9,4,10],[]],0,[[],[],[],4],5]]

[[[2,[6,4],[7,9,9,0]],10]]
[[4],[],[[[1,1,8],[],[0,7],[6,6,6]],1,7],[[[6,10,2],2,[7,5,4],0,[9]],0,5,5],[4]]

[[],[7],[0,[7,8,1,[],[3,8,0,3,9]],[6,[10,4,1]],[0,10,2,1]],[]]
[[],[4,8,2,[2],4],[],[[[7,7,5],[8,2,10],[4,7],9],6],[[[7],[],8],[],[[2,5],5,[9,4,1,1,2],1],[]]]

[[[0,3,[9,7,3,4],[],[0,10]],[]],[[8,6,8],1,6],[6,[[],[8]],0,[10]],[[],[[2,10],2,[7,5,9,4],[6]]],[]]
[[9,2,6,[[2,8,8],[10,9,0,10,1],2],[[0,1,8]]],[6,[[10,9],6],0,[1,6],[]],[[9,4,[],[6,3,1,3],[5,6,7,7,8]]]]

[[[0,5],[[6,2,3]]],[[3,[3,5,6]],6,[8,6,10,10]]]
[[],[6,2,1],[3,[]]]

[[5,[3]],[6,[0,[],6],9],[[9,9,6,[4,2,2]]],[[[3],[9,4,3,1]]],[1,[3,[4,8,9,10],6,9],[[],3,[6,9,8],[]]]]
[[[0,2,3,[8],[4,5]],0],[[[4,0,7,5,3],4,3,10],[6,[2,4,4,1],[8,9],10],[3],4,[]],[0,6,10,10,[6,1]],[[7,[8,4,1,8]],10,[[6,7,6,9],9,1],[],6],[1,10,10]]

[[5,8],[6,[[10,6,10,1,0],2,6,[1,10,2],4],3,0],[7],[4,10,[10,[2,2,3,1],6,[8,3,0,2],[9,10,9,5,9]]],[]]
[[9,1,[[3,9,2,1,4],3,2],[[],[2,5,7,9,7],9,0,[7,8]]],[]]

[[4,[[5],[8,1,2],[0]]],[[[2,9,3],7,[1,6,2,2]],2],[],[[3],[[10,10],[7,6,7]]],[[[6,4]],[0]]]
[[[9,[],8,[5,4,5],[7,8,9]],0,8,9]]

[[[0,10,[6,7],[8]],[5,10,[6],4],7,1,2],[],[],[],[10,[],2,8,4]]
[[[4]],[],[[],5,7,10],[5,2]]

[[[[1,9],[3],[5,5,4,4],[2],5]],[2,[[],[0,9,4,4,5],[10,10]],[1,[],[10],4]],[[8,[5,9,8,3,1],3,4,3],[[0,3,0,3,8],[0],[2,3,9,1,2],[0,6,1,9]],[9]],[[[],2,[7],3],[[7],0,[7,9,1,4]],3,[[9,4,6,4,4],[1,8],9,[4]],[[]]],[4]]
[[2,[],[10],6,3]]

[[4]]
[[[4,[4,0,5,7],2],[],[9,[1]],2,10],[[0,8,7],8,7,10],[],[3]]

[[7],[],[5,[[0],[],[4,5,2,3,0],[6,4],[5,9,2,4,1]]],[]]
[[],[9,[8,[5,7,7,3],3,[7,0,3,10,0],7]]]

[[[6],[8,7,5,[4,4,9,0,9],[]],[[0,10,6,6]],1,[]],[[[],[]]],[3,2],[]]
[[[]],[[[2,3,0,7],8],1,0],[0,10,0,[3,4,5,[4]],9],[0],[[[3,7],1,2,10],[[4,10],0,5,1],[]]]

[[[[4,0,5,6],[7,4],9,1]]]
[[9,[[6,3,6,0],0,[1,6,0,0],[8,0,0]]],[2,5,[],[[7],8,[8,8],4,1]],[[],[[4,7,8],[],[9,4],4,[]]],[[[10,5,3,9,2],7,3],[7,[9,0,8,10],[9,6]],[5],8,[[],6,[10],[7],2]],[]]

[[[4,[1,10],[6,3,9],5],9],[]]
[[10,[],3,[],[[0,1,10],[3,2,1,3,3],[10,4]]],[]]

[[[],[[9,9],4,[9,7,2]]],[3,8,0],[10,[3,[10],3]]]
[[8,[2,[2,6],1]],[[6,6]],[[[0],4,5],[[3,9],1]],[[[5,7,8],0,[]],[[9,5]],4,[[2,10,0,5,9]],[[],6,9,[],[3,10,10,2]]]]

[[],[[[5],[4,3,2,10],[],0]],[9,[],[7,10,[6,5,8,5],9,7]],[[7,0,[7,3]]],[]]
[[7,[],3,[[],[4,0,0],5,[2,5]],9],[[6,[9],[],[],[4,10]],[[3],[0,4,6],4],[[5,2,2,5],7,[10,8,4,10,3]],[2,[0,7,3,4,7],1,[3,2,8,2]]],[6,5,[[10,5,6,9],6,[8],[1,9,10],7]]]

[[[[3,2,2],[0,8],[3,2],6,9],[1,[8,10,6],[8,9,5,0,5]]],[1,7,[[4,3],[0,7],[8,6,5],6],[6,6,[2],[8],[7]],[]],[[4,[4,9,3,10,7],[5,0]],3,2,[6,3,0,[]],[[7,7,3],[0,0]]],[[]],[]]
[[7],[],[[9,5,[5,7],0],[[2],0,[2,10],0,4]],[7,0,[[8,5,5,7]]],[]]

[[[9,[5,0,7],10,[6,1,10],[6]],[[1,10,0,0,4],2,[2],6,4]],[[],[[10,5],[7]]]]
[[8,[[2,5],7],[[4,4,2],4],6,[]],[[],7,10],[[[4,3,9,6,0],[2]],8]]

[[8],[[],9,3],[9,9,[[2,5,7,0,3],10,2]]]
[[0,[],[[5,2,2,1],[4],[6],4,[5,4,10]],4,3],[4,9],[[[0,8,1,0,0]],[[6,6,2],[4,4,0],[],[6]],6]]

[[[2,[],7,7,9]],[10,[[8,10,7],[7,9,2,0],7,[5],[3]],3],[3,6,8]]
[[1,[[4],[7,5,3,1],[9,8,8,2],10],[],[10],3]]

[[],[9]]
[[2,4,10],[[],8,10,[7,3,5],[8,[10,2,3],4,2]],[1,9,[0,4,[3],[6]],[[8,10,1,4],[],4],[2]],[[]]]

[[[[],0],3],[[2,3]],[[9,10,[3,7,0],3,[9]],8,7,10],[[[7,3,1]]]]
[[4,8],[9,9],[3,1,[[10,0,2,10],6],[],[2,3,[6,0,9],9]],[3,5,3,[[6,9],[2,9,1]],[[6,4,9,3,4],0,[7],[2,7],[9]]]]

[[5,[8,[5,4,7,8],[1,1,6,5,10]],[7]],[[2]],[[0,[1,8,8,10,3]],[],1],[[2,9,3],10,[0,8,[7,3,0,1]],1]]
[[[[4,0,5]],9],[[10,4,7,7],4],[0,[[7,9,6,9],9,10,0],[8,1],0,[8,[4,1],10,[2,9,10,7]]],[[],[6,[1],[5],5,7],[[5,8],9],10,2],[[8,9,1,[4,6]],[],[[],[10,4,2],4,4],[9,[2,4]],3]]

[7,10,0,6]
[7,10,0,6,1]

[[8,[[0,4,2,1]],[8,3],[[],3]],[4,[[]],[6]]]
[[[7,[1,10,1]],[[6,5,9,1],[3,3],[6,2,3,6,2]],[9,[9],8,6,1]],[[[7],[1],[0,7,2,7],7,5],0,1],[3,[[1,7],2,[8,5,6,10,2],7,[9,6,7,9]]],[7,3,1,[4,1],3],[[]]]

[[[0,1],10,9]]
[[2,[[10,0,2],3,[1,1,4],[1]],7,[[],[2,6,10,3,6],[3]]],[[6,[],[0,8,0,1,8]],9,3,10],[10,3,7,9]]

[[[1,4,[2,2]],[9,[8,6,10],5],[9]],[[[0,4,3]],8,[[9,9,0],[1,2,10]],6,8],[]]
[[10,0,[],[],3],[9,[2,10,5,[1,4,3]],4,1]]

[[3,10,[3,[9,2],[3]],5]]
[[4,[8,[0],[5,1,5]],0,[5,[5,8,2,3,9],[]]]]

[[],[[6,[1,7,9],[],10],7],[],[[3,[],[9,9,6,0,4]],[[7,5,7,2,10],0,[6,10,3],[6,2],[6,6,1,1]],[[1,3,9],[4,7,6,7],6,3,[4]]],[[7,[3]],7]]
[[[[10],[9,9,10,1,0],[10,6,9,8,4],4]],[1],[6,[[10,9,6,9,10],4],[[6,7,2,1],[],[4,1,4,1],[]],[5,1],[[2,5],9]],[],[]]

[[],[[[1,9,9],9,[5,5,5]],[],9,8]]
[[[10,5,[2,9,2,1,7]]],[],[[3,8,9,[],[9,9,5]],7,[]],[[7]]]

[[[[]],3,0,8,[10,[10,6,9,8,3],7,5]],[[[6],5,[9,1,2],[3,4,0,4]],9],[10,[],[2,2,[9,10,5,6]]]]
[[2,3]]

[[1,9,[8,[4],[4,2,0,0],[10,7,1,10],[8,10,10]]],[6,3,8,2,9],[7,[1,1,[4,3],[]],[],9,4]]
[[0,10,7,[],5]]

[[[[4,8,1,4],[]],10,0],[],[],[3]]
[[1,[[3,8],[3,4,6,6],[],[6,10]],5,[4,[6,5,10]],9],[7,[],[[2],[],4],3,2],[5,5],[4]]

[[[[0],3],5,1,6],[[],[[4],9,4,[9]],7,[2,4,[3,0,4,5,5]]],[2],[6,[6,8,[],3],[],6,[[5,8,0,6],10,[1,5,9,4]]],[]]
[[4,10,[8,[],[0,5,10,2,6],8],4],[],[[9,10,3],7]]

[[[6,[5,3],[3],1,[2,4,9,4]]],[],[[[1,2,2],5,1]],[8,[8,[7,2,7],[9],7,8],2,[0,1],6]]
[[[[6,9,0,0]],3,[5],9,[[8,1,5,4,10],6,[]]],[3],[[[1,5,1,2],8,1],[8],[3,[4,4,8,3,3],[0],[7,7,0,3],9]]]

[[7],[1,1,0],[[2],[[1,0,6,8,6],[3,4,8,10,9],2,[6]],10],[[[8,8,9,0],[],3],[1],[9,1,[6,0]]]]
[[],[2,5,2,4,5],[1,6],[[2,[],[6,4,2],9],4,[5,[5,4,1,3,8],0],[2,[9],1,[],6]],[]]

[[3,6,8,[]],[]]
[[[[8,7],4,[]],3,[3,[0],[5,4]],7],[[],[4,6,[6,4,9,4,2],0],10,[8,8,2,0]],[[[3,0,6,10],[8,9]],[[3,7],[7,9,6,5,9]],[]]]

[[[8,2,[9,5,1,2,2],[0,10,0,5,7],[1,8]],8],[4,[],[7,[],1,[],0],[[5],3],7],[1,[[],[4,2,8,1]],8,4]]
[[7],[[5,6]],[6,[2,8,[],2,[6,3]]],[[[1,1],2],6,[[6,10,7],1,[8]],[[],10,2]],[]]

[[],[8]]
[[],[[[9],6],0,8,[3,[],9,[],1],[1,6]]]

[[[8,[6,0,9,10]],[[6,1,8,3,1],[9,4,10,1,5],[2,7,5,3,4],[2]],10,7,[5,[8,3]]],[4,10,[[4],[]]]]
[[[[5,8,0,9,7],6,5],[],1,[]],[1,[],[[0,7],[1,10],[3],[0,0,10,7],0],[[9,8,1],[]],[[6,1,10,6],[4,9,4],[8],0]],[2,9,[4],1]]

[[9,[8,2,0,[7,10,9,7]]],[8,[[0,10,9],0],[8,[7,0,0,2]]],[],[[]]]
[[2,[[3,2,7]],[0,[6,6,0,4,5]],[10,[3,6]],[]],[0,10,[],[7],[[7],5]]]

[[9,[[3],6,[10,4],6,[9]]]]
[[[[5,1,2,10,3],[7,7,1,8]],3],[9,[],[[3,7,6]],6],[[[4,8,10,3],0]],[[8],[[1,4]],[5,[6,7,7,3]],7,9]]

[[[10,[8,1,2],2,1],2,[3,[6,2],[10],[7,10],4]]]
[[[9,8],7,[[0,6,9],[8,1],[8]],[7,[],0]],[9,0,[[1,2],4,8,[],[]],[1],9],[[]],[]]

[[2,3,[9],[[8,1,1,10,4],[2,3,9,2,7],1,4]],[],[[2,6,6,[4,5,2,8,3],9],0,[[1,9],6,[6,6,0,5,10]],9],[[5,[],7,7,4],[5],6]]
[[[4,0,[3,6,3,5],0],4,[],[7,[4,1,3,3],5,[4,4,4,2],[10,8]],1],[]]

[[],[0,[[],[0],1,5]],[5,[5,1,[5,6,8,4,0],7,[4]],[0,[]],0],[8,1,2,10],[2,[9,[7,8],3],[9,10,3,[10,3,8]]]]
[[[1,0,8,5],4],[],[]]

[[1,[10,[7],[],[7,7,7,1],6]],[8,[4,[0,7]]],[[]],[[],[[6,4,2,8,6],7,[9,0,3,6]],9]]
[[],[10,2,[],3,0]]

[[6,[[2,10,10]]],[[[],[]]],[1,9,2,3,[[5,3,9],6]],[10,[[5],9,[8,0,4],[8,6,4,6]]]]
[[],[[6,[2,2],9,0],7,[[7,4,2,1,4],6,[0,10,5,10]],1,[[7,9,5,7,1],5,[],4,10]]]

[[9,6],[[9,[7,0,1,7,7],[]],[1,[7],[9,3,9,3]],0,[[]]],[9,6],[]]
[[[[],0,[3,2,1,2,1]],10,[5]]]

[[],[7],[[6,4,0,7,[6,4,0]],2,1,[[7,3],8,[2,2,10],7],5],[]]
[[],[7,8,[[10,5],[10,0,10,4,5],4,[10,3,6,4],[]],8],[],[3],[2,8]]

[[[[5],8,[],9],[[1,5,8,9],[8,2,5,2,10]],[],8,4],[1,4,9],[],[[4,7,3,0]],[[8],[3,[8,10,8,9],4,1],[[9]],[5,1,[],[7,5,2]],[5]]]
[[6,7,[3],[]],[[[1],[10,1],5,4],3,[[0,8,10]],10]]

[[2,3,[0,[0,1,10,1,8]],[[6,0,4,10],[3],6,3],10]]
[[7,5,[[5,5],9,[4,3],[4,8,7,8],5],8,[3,3,7,[]]],[1,[[6,9],[4,0,4,2,3],1,[1]],3,[],[6,5]],[[8]],[[[0,8],[0,8,4,5],2]],[4,[[],[7,9],[6,0,7]],10]]

[[[6],[[9,9,2,6],2,7,1,1],[],[7,[9,4],[7,5,10,3,1],[2,1,5,7],8]],[[3],[[2,8,1,10,2],1,[4,6],[5]],[2,[0,5,1,5,2]],[]]]
[[3,7,[]],[8,8],[[6,3,5,[6]],10,[8],10],[[3,[3,10],10,[],[3,8]],0,[0,2]],[1,6]]

[[7,1,[10,[],[8]],[5,2,[1],[0,0,1,4]],3],[5,[0,0],10,[7,5,[],3]],[[[0,5,5],4],10,[9,[4],[10,3,7,1]],[[8,10,1,10,1],2,4,[9,3],[2,9,0]]],[[[0],[3,8,9,5],0,[4,2]],8,9,[[4,0],4,[1]]],[[4]]]
[[[[0],2]],[[9,[5],[6,4,0],[8,7,5,5,5],3],[[8,3],[],10,10]],[3,6,8],[9,[3,[2]],2,[10,[3,5,2,10],8,3],1]]

[[9,5,2],[],[5,[[4,6],5,3,3,7]],[[7,5,[],[5,10]],[],[],[[8,9,1],1]]]
[[0],[[[]],7,1,2,4],[[[1,2,10,1,8],[7,10,9]],[],[[]],0],[10,9]]

[[1,[[10],2,5,8]],[],[],[5,[],6],[]]
[[[[3,3,8],4,4],9,[7]],[]]

[[[4,[0,0],6,4],3,[[3,8]]],[[[1,3,7,0]]],[[[]],1,[[10,1,3,8],[5,8],3,[6,5,4,4]]],[[0,1,3,[8,6],[1,0,10]],4,[],[[6,8,6,4],[4,0,8],[],0,7]],[[[],7,[],8],9,[],3]]
[[[6,[4],10,[1,2,6,10,6]]],[[[6,5],6,[2],1]],[[[6]],[1],[[9,5,5,2],8,5,8],[1,[]],[]],[7,[[2,8,0,4,5],4,7,[7,3,4,5,7],[]],4],[[7,2,5,5,5],[[6,1,5,5,10],[4,10,6,3,8],[2,8,6,0,1]],[[1,1],4]]]

[[0]]
[[[[],[7,3],[6,7,9]]]]

[[],[5,4,7],[[[6,1,10],2,[5,7,9,5],[]],[0,[1,10],[8,4,5,0,7],[9],[10,0,5,5,9]],[[4],[],[],[6]],[7],1]]
[[[[],0],[2],2],[],[[8,[0,5],7,[7,6,0,6],[3,0,3]],1,10,9],[9,8,[6],7],[]]

[[9,[0,[4,8],[7,4,2,6,6]]],[[9,[6,5,3,6],10,5,4]]]
[[[2,[4],[7],5],[[3,2,5,10,9],3,[10,5,8,4],3]],[]]

[[],[],[9,[[8,7,6,6],7,0],[4],5],[[[5,7,4,0],1]]]
[[[1,6],5,[],[1,[10]]],[0,[[9,8,10,8,10],0,7],5,[[5,3,8,10],[7,9,2,0,0],8],[0,[6,0],4,[0,7]]],[[7,[5],3,6]]]

[[1],[9,[[2,9]],[[2,1,1]],9,[]],[9,[8,[8,2,6,3,1]],1,0]]
[[[5,1,0,0],[5,7,5,0,[]],3,8]]

[[[],8,8,6],[[5,[],[9,8,2,9]]],[],[3],[3]]
[[4,1,[2,5]]]

[[[[7,8,8,3],7,10],[[7,0,4,4],0],10]]
[[8,5],[4],[10],[5,7,[0,[8,7,2,3]],[]]]

[[10]]
[[[[2],8]],[],[],[1,3,[[3,4],[],[1,1,2,9]],[8,0,[9,0],3,[3,5,6]],1],[[8]]]

[[[[5],[4,6],[0,8,10,0,7]]],[[3,[6],10,2],[],5],[],[]]
[[[[2,1,2,9,9]],[[9],[1,4,4],8,[4,5,4,6],7],[[7,9,5],[5,2,0,4]],5],[[9],[[4,2],[1,7,5,9]],4,10,[2]],[2,6,2,7,[[9],[0],1,10,[3]]],[2,1,2,10]]

[[[[5,0]],3,[8,2,10],[[7,6,2,9],9,5,[7,2,8,10,9]],0],[[[3,7],7,[]],6],[5],[1,3,3,[[3,9,8],[1,3,7,6],1,[8,0,8],8],[[2,3],[6,1],[8],8,4]]]
[[[8,[1,8,3,9,6]],2,[10],[9,[0,0],[],0]],[[[4,0,8],0,6,8,[1,4]]],[7,2,[0,6,[4,8,0]],1,5],[2],[[[2,3]],9]]

[[3,[5],8]]
[[5],[[7,[7,2,8],[5,2,6,7],9,7],0,[],5],[[],2,3,[[8,9,2,0,8]]]]

[[[[10,10],[6,10,10]],[],[3]],[10,1,[[],6,[3,9,0,1],[4,9],5]],[]]
[[[0,[8]],[5,0,[3,8,6,5,1]],10,[9,[3,9,9,7,3],2,[1,0]]],[3,3,[[4,10]],3,[[0,7,4],5,[],[6,5,5,3,2]]],[[6,3,[],4,[10,1,0,0,1]],8],[[],[],8],[6,[7,[]],[]]]

[[[4,0,[6,10,8,1,2]],[],7,1,[6,[0],8,[6,8,8,10,5]]],[[2],1],[[],4],[0,[9,[10,9],1,9]]]
[[5,10],[[],7],[[6,[1,9,10,0,9],3,[]],5,[[7,7,3,2,7],[]],[],7]]

[[2,1,9,3]]
[[[]],[8,[[2],3,[10],[]]]]

[[[9,7],[3,[]],4],[10,[9],[],[[5,7,0],[4,9,5,5,5],[5,2,0,5],3],[[],5,[1,6,0,2],[],[]]],[[],[0],[[6]],10,4]]
[[9,10,[6,7,8],5]]

[[[[9,5,4,8],[],[4,3,8,4,4],2],[1,3,5],[[],1],[8],[]],[5],[0,8,4,[9,4,3,5,[3,8,10,4,6]]],[[3,1],10,7,6],[2]]
[[4,2,[[1,8,2,7],1,4,10,5]],[5,[[5],2,[0,6,6,5],1,[7]],0,7,[[7,3,3]]],[[5,8,[0]],5,[],5],[[]],[[],[3,4,[5,8,0],[3,8,10]],5,1,3]]

[[8,4,8,9],[[[7,6,6,3],6,2,5,[5,7,6]],[],0],[[[5,7,10],0],[[5,4],3],7,2],[]]
[[5],[[4,[10,6],0,6,[5,8,5,1,1]],9,5,7,[7,4,6,10,7]],[1,[4]],[3,[1],5,[3,[2,8,10],4,8]],[9,[[5,0],1,[4,0,6],[7,0]]]]

[[[[10,8,8,5,2],3,[0,1,8,7,9],6],7,1],[],[[1,8,1],[4,[8],[6,9],5]],[[4],[[1,9],[9,2],[4],[10,10],[10,1]],9,2],[]]
[[[5,[]]],[[[3],[10,1,0,1,7],[9,10,7]],10,5,6],[],[[8,[2,5,7,7],8]],[7,[[4],[8,8,6],9],[[10,6,8]],1,[]]]

[[],[[[1,6,1,5],[2,5,5,3],[],0,5],[],[5],8]]
[[[[8,3,3,3,1],[3,8,0,8,10],[6,10],[]],[8,[9],[4,3,9,8],[]],3,[],[[8,0]]]]

[[[[0,7,2,9]],[5,[5,4,7],9,[9,8,2,4]],10,[[7,3,2],4,[8,3,0,10,1],[1,5]],7],[1,10,1],[]]
[[[3],[10,[],[1,0,5]],[[9,8,8,8]]],[[7,9,2,1,[]],[4],[3,[10,0,3,0],7,[6]],10,[4,6,[2]]],[[[0],[],[10],1],6]]

[[8,[[]],0,0,9],[0,0,3,[],[[9,7,2],[3,7,7,2],[7,6,4],6,5]]]
[[],[6,8,[4,7],[2,10,[4],[1],[6,1,5]],7],[8]]

[[3,[[1,0,8,5],5,[],[4],[]],5,[3,6,10,[],7],9],[4,[6,[4,4,5]],[]]]
[[0],[0,[5,4,5,8,6],3],[10,6,[]],[5,5,10,6,10],[10,[0]]]

[[]]
[[],[2],[8,7]]

[[],[[],7,[5],2,3]]
[[[5,[9,5,10],6,[10,6],4]],[[[4,9,10,9,1],[8,2,4],5,[10,10,5],7],[2,9,2,6]]]

[[],[]]
[[3,2,[[3,3],10,7],3],[],[[],[]],[[[8,10,10,4,8],9,[4,8,3,0,4]],[8]]]

[[[0,[9,3,10,2,4],4]]]
[[],[2],[[9,6,5,[3,5,8,4],[9,9,3,6]],[8,[4,1,3],[]],0,0],[[7,2,[3,9,6,0],3,[1,3,10,0,3]],10,[[7,0,4,10],9,3],[[7],[7,6,6,4,10],[10,5,2,5],[4,1,10,8]]],[9,[[3]],[1,[9,6,10],[6,2]],[0,7]]]

[[3,[[],9,6,5,6],[]],[[[6],7],6],[5],[4,[4,0,[5]]]]
[[],[3,[],[8,[10,4,7]],[[10,3,4,1,10],4,5,8]],[[[6,4,2,8],[],5]],[5,0]]

[[4,[[],[]],[]],[]]
[[5,6,0],[[2,0],[[8]],8,9],[1,6,2]]

[[[10,[],3,[8,1],[2,0,1,3,7]],3,[[1,6,4,2],[8,9,3,7],[1],[4,6,1,6]],7,[[0,1,7],[7]]],[[4,[4,0,6,1],7],2,5,10,[[7,10,4,3],1,5,[],5]],[[[2,6],[6],[6],[9,6,1],9],[[3,6,2,4],9,4,4],9,5]]
[[6,[],8]]

[[],[[[],[9,10,3,0],4,10]]]
[[3,8,1],[],[9,6,2,1],[10,2,3,[[],1],[[1]]]]

[[10,[],[[],1,8,3,[9]],[[],2]]]
[[[[9,10]],1],[[7,6,6,[2,2,0],[3,1,10,0]],6,[]],[9,[[3,9,8,5],[4],1],[[8,10,8],4,[2,1,1]],6]]

[[],[[]],[[7],7,6,0,[]],[[[1,0,6],3,10,6],[[3,2],[5,6,7],3,[],[2,4,0,2]]],[]]
[[[[4,4,3,7],[2,5,3],8,10],0],[],[6],[[[1,9],[],[4,6,5,4],[10,2,10,9,2],3],6,[3,7,[3,8],[7]]]]

[[9,5],[0],[[4,[],[]],[8,[]]],[8,[9]]]
[[[[3,8,6,0],[9,9,9,3,0],1,9,6],9,[0,1,8],[6,[7,8]]],[5,9,4],[[[5,8,2]],[1,[],1,7,6],9,[7,7,2]],[],[]]

[[[[5,4,8,7,4],8,2,7,10],4,[0,6,[6,9,2,6]]],[],[[1,[5]]],[5,6,9,1]]
[[],[6,4],[[[3,0,2]],[8,1,[6,7,5,5],2],[7,5,6],[],10],[4,[[],6,[]]]]

[[[[],3],9],[[0,4,[2,2,2]]],[],[[2,9,6,[9,9,2]],2],[[6,4,6,5],6,6]]
[[],[[[],[],4,2],0,7],[[[9,7,6,10],9]]]

[[7,[[8,2,6],[7,6,10,8],[4,6],[0,7,3,1],7],7,[[7],[],[9,0,5,2],[8]],[4,[9,8,10]]],[[10],[1,8,0,[]],[7,[8],8,7],[[3,2,1,3],9],[[10,0,8,10,7],0,[2,0,4,1]]],[[3,[6,0,2],[1,10],7],5,7,1,[]]]
[[5,0],[8,[[10],[]],[9,10],[7,[9,4,0,5,0],6,4,[3]],[6]],[],[[0,[],[3,1,1],[],[9,9]]],[]]

[[0],[0,[7,7,[3,5,2],3,[0,1,9,8]],[[10],[10,4,5,5,6]],[[4,9,4,8],2,[3],[7,5,2,7]]]]
[[[[9,4,1],[4,10]]],[2,[4],[],[[],[3,2,5],7,4],[6]]]

[[7,[[],8,[8,2,8,2],[3,1,3,10]]],[5,[],2,5,7],[2,[[]]],[],[[[],[2,6,8]],9,[2,2,3],[3]]]
[[4],[[]]]

[[[0,4],[6,1],8],[],[2],[]]
[[],[[[2,4],[8],0]]]

[[3,[7,[6],2,4]],[7,[[]]],[6,7,8,6,0],[],[]]
[[],[9,0,1,[2,6,[3],[8,4,8,0,8],[2,1,4,7,1]]]]

[[[0],0,0,[]],[],[[3,8,7],4,[[8,6,5],[1,9,8]],[],2]]
[[3],[5,1,[1,[4,7,3,5,7],[7,1,5,3],[9,4,3]]],[1,[3,[4,7,10,8],[6,6,4,3,7],[4,1,7],8]],[7,[6,[0,2,8,10,10]]]]

[[9,[],[]],[[9,[4,2,10,5,0],0],1,10,[1,[2,0,5,2],[6,1],[2],4],5],[4,[10,[1,6,6,9,10],[5],[5,2]],2,[[4,4,6]]],[1,[],10,7],[10,[4,4,[0,6,10,6,0],4],1,3,[3,9,[5]]]]
[[[[2],[],7,[6],[9,8,5,5]],[[3,6],5,9,[7,3,6],[]],10],[10,[[8],[4,7,10,4,3]]]]

[[4],[1]]
[[[],7,[[],[9,4]]],[[6,[7,1,0],[7,0,3],5],7,4,7,9],[[3,[4,10]],8,9,1,[1,[0,9,0,9]]]]

[[4,[[8]]]]
[[2],[[[]]]]

[[8,8,3],[]]
[[4,10,[10,0],9,4],[[],[[9,5,8,8,7],3,[3,8,2]],[[10],[7,8,2],[3],[2,8,7]]],[],[1,4,[9,8,5],6],[[8,1,[9,7,6],[8,10,10,0,9]],[],8]]

[[2,[[1,10,8,10],3,10,0,[0,10,5]],3],[3,[[8],[3,0,3,7]]]]
[[],[[[2,10,3,7,9],6,[3,10,4],[7]]],[],[10,[2,[]],1,8,1]]

[[3],[5,3,2,[[7],1,[5,9,0,9,4],[4,8,1,1],7]],[[1],2,[1,[10,6,7,10],9],5,7],[],[]]
[[[0,[],10],[8,[],2]],[[[0,4,8,2],10,1,8],[5,2,3,[8,4,8,5,4],[0,5]],[7,8,3],[6,1,[10,8]]],[6,[],2],[]]

[[],[7,[[1],7,0],[6,3,[9,9]],4]]
[[],[4,[],[[6,6,3],9]]]

[[9,[5,[9,8,2],9],0,9],[[[9,4,2]],2,2],[5,[],0,[[0,8,10,6],2],[[2,7],[9,5],9]]]
[[7,[[6,4,2,4,3],[9,3,7]],2,1,9]]

[[3]]
[[6,[[6,0,8],[3,3,5,8],[]],[[4],[10,1,7,9],7,[]],8],[2,2],[[[3,0,7],[2,1,10,1,9],4,9]],[4],[9]]

[[[],7,5,6,[]],[9,[[3,2],0,[1,3,9,8,5]],3,10,3],[7,8,[7],[8],2],[[],[[7],4]]]
[[[[6,2],3,7,[6,5,3,0],5],2,0,[[5]]],[6,4,3],[],[]]

[[7,[[1,3,0,1,3],[3,1,1,2,2]],5,10,[0,10,2,5]]]
[[[],1,1,[]],[],[[],[1],[2],0,[[0,7,7,2,2],[2,6,8,1]]]]

[[5,10,7,9]]
[[3],[9,8,[[9,8,0],9,[7],5,[]],7],[[[2,6,10,7],1]],[3,[5,[7,1,10,0,10],6,9],10,4],[[4,8,[7,4,8,4,4],[],[3]],7,6]]

[[[[1],[4,6]],[1,3,5]],[],[8,0],[[5,1,6,2],[],[[],1,[4,9]],[3,[10,9,5,5],[0,0,0]],4],[3]]
[[9],[[[4,3,6,0,3],3,[],[0,10,3,4],5],5,3,[[7]]],[]]

[[5,3,5]]
[[],[4,[[1],[7],4,[],2],[],[[2,4,4,3,3],0,3,8]],[5,[],[[],5,1,4,2]]]

[[2,[[6],1],1],[[[7,5,5],7,9,9],6,[1,4,10]]]
[[[[0,8,8,1],[8,8,10],[5],[8,8,10,2],2],9,5,[7,1],3],[4,[4,[9,1,1,3,10]]],[1,[],[2,[9,0,4,3,10]],[5]],[[],10,7]]

[[[0,[1,4,6,4,10],10],[1],6,2],[10,0,7,[4,[9],4,8,5],[10,[9,6,7,5,2],[1,6,6,2]]],[[2,[1,4],[6,5,3],4],[[6,7,7,8],[9,2,8,4]],9,7]]
[[[5,[2,0,0,8],[8,5]],[[3],9,[9,0,0,0],[1],9],[2],[[],2,1,9]],[],[[9,5,7],6,[6],[10,[2,9,4],[],[]],9],[2]]

[[5,[0,[10,4,0,1,5],[8,4],9,[6]],[[4,5,10,5,7],[],[2,5,9,7],[5,3,2,6],[6,3,1]],6,[]],[0],[2]]
[[[],0,6],[[7,[5],[7],[7]],[[0,5]],[[8,0,10],[0,2,4,8,2]],10,[[]]],[[],[5],[[],10,1,4]],[6,[[2,5],[1],8,5],[[],[0,4,0,10],[10,6]],[2,8,[],10],10],[8,[10,[7,3,2],[10,3]],1,1]]

[[[],[],[[]],0],[0,10,[9],[[0,5,1]]]]
[]

[[[10,[4,3,9],[7]],0,[9,3,0,7],9],[[]],[[[9,2,7],[6,10,8,9,10],10,8,[9,1,2,9,1]],[[10,4],[8,9,4]],5,[[9,1,6,2,10],[0,6,0,2],[4,7,8,2],[5,10,5,10,8]],[[0,3,1,9,1],[2,0,4],[7,9],[0,5,1,0,6]]],[5,5],[9]]
[[[[6],2,[3,0],[0,5,5],7],[4,[0,9,9,0]],[10,[6,7]],9,[1,[1,0,7,2],[4,2,6,7,3],[5,9,3]]],[],[[[10],[10],[]],[[9,1,9]],[1,10,[]],5],[2,5,[7,[3,8],[]]],[]]

[[[[4,4,2,3],[8,4,0,4,2],[],[9,1,7]],[],[[6,7,8,4,6],4,8]],[8,[],7],[]]
[[5,5,[[3,10,1],4,8,[4,1,10,0,1]],8,[[9,2,1,6],8,[9,7,8,3,8],[3,6,10,1]]],[[[5,0,10,6],[0],[7,2,0],[1,1,1,2,3]],[0,[0,7,10,4],8,[10,9],[]]],[10],[[[10,7],[0,1,4],[0,6,5,3]]],[]]

[[[[10,6,4,9,9],[3,3,9,5,4],[9,6,1,7,6]],8,[]],[5,[[],[0,6],[9,8,6],7],10,[8]],[3,7,[6]]]
[[8],[],[[0,[2,4],[7]],[[8,3],9,[6]],[4,1],[6,2,10,1],[7,7,[2,1,2,4]]]]

[[[[9,5],9,[8,1],[0,3,6,9],[6]],[[0,10,2,2],6]],[[[9,5,5]]],[6],[3,[],9,[[5,4,8,6,6],[10,6,6],[],9],1],[3,0,[[],9,[4],2]]]
[[5,[],[[]]],[[10,[],[8,8,6,9],[1,4],[5,2,7,9,6]],2,9],[],[[],2,5,[5,6,1,8],0]]
"#;
