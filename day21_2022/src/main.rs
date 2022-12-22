enum Operation {
    Add(&'static str, &'static str),
    Sub(&'static str, &'static str),
    Mul(&'static str, &'static str),
    Div(&'static str, &'static str),
    Val(usize),
}

fn decode_input(input: &'static str) -> Vec<(&'static str, Operation)> {
    input
        .split_terminator('\n')
        .filter(|l| !l.is_empty())
        .map(|l| l.split_whitespace().collect::<Vec<_>>())
        .map(|v| {
            if v.len() == 2 {
                (v[0], Operation::Val(v[1].parse().unwrap()))
            } else {
                (
                    v[0],
                    match v[2] {
                        "+" => Operation::Add(v[1], v[3]),
                        "-" => Operation::Sub(v[1], v[3]),
                        "*" => Operation::Add(v[1], v[3]),
                        "/" => Operation::Div(v[1], v[3]),
                        _ => panic!("unexpected input"),
                    },
                )
            }
        })
        .collect::<Vec<_>>()
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use crate::decode_input;

    #[test]
    fn test_part1() {
        let input = decode_input(TEST);
        assert!(input.len() > 0);
    }

    const TEST: &str = r#"
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
"#;
}
