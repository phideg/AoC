#![feature(exclusive_range_pattern)]
#![feature(slice_as_chunks)]

#[derive(Debug)]
enum Element {
    List(i32),
    Value((i32, i32)),
}

fn map_line(line: &str) -> Vec<Element> {
    let mut list_number = 0;
    let mut number_start = None;
    let mut open_list_pos = 0;
    line.as_bytes()
        .iter()
        .enumerate()
        .fold(vec![], |mut acc, (pos, char)| {
            match char {
                b'[' => {
                    list_number += 1;
                    open_list_pos = acc.len();
                    acc.push(Element::List(list_number));
                }
                b']' | b',' => {
                    if let Some(start_pos) = number_start {
                        acc.push(Element::Value((
                            list_number,
                            line[start_pos..pos].parse().unwrap(),
                        )));
                        number_start = None;
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

fn part1(input: &mut [Vec<Element>]) {
    let mut l = 0;
    let mut r = 1;
    let mut current_pair = 1;
    let mut pair_ok_count = 0;
    while r < input.len() {
        let mut li = 0;
        let mut ri = 0;
        let mut is_pair_ok = true;
        let mut left_li_no = 0;
        let mut right_li_no = 0;
        while li < input[l].len() && ri < input[r].len() {
            match (input[l][li], input[r][ri]) {
                (Element::Value(l_level, l_val), Element::Value(r_level, r_val)) => {
                    debug_assert!(l_level == r_level);
                    if l_val > r_val {
                        is_pair_ok = false;
                        break;
                    }
                }
                (Element::List(l_level), Element::List(r_level)) => {
                }
                (Element::List(l_level), Element::Value(r_level, r_val)) => {
                    is_pair_ok = false;
                }
                _ => panic!(),
            }
            li += 1;
            ri += 1;
        }
        if is_pair_ok {
            pair_ok_count += current_pair;
        }
        l += 2;
        r += 2;
        current_pair += 1;
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {

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

    #[test]
    fn test_part1() {
        let input = super::decode_input(TEST);
        super::part1(&input);
    }
}
