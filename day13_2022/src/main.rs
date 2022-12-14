enum Element {
    Scalar(i32),
    List(Vec<Element>),
}

fn decode_input(input: &str) -> Vec<[Element, 2]> {
    input.split_terminator('\n')
    .filter(|line| !line.is_empty()).map(|line| {
        let mut slice = line[..];
        let mut list = Element::List(vec![]);
        let number = "";
        while slice.len() > 0 {
            match slice[0] {
                b'[' => { if let List(elements) = list {
                    elements.push(Element::List(vec![]));
                } else {
                    panic!("Invalid input");
                }}
                ,
                b'0' .. b'9' => {
                    let end_index = slice.as_bytes().position(|&r| r == b',' || r == ']');

                }
            }
        }
    }).collect()
}

fn part1(input: &[(&str, &str)]) {
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
