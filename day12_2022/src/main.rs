fn decode_input(input: &str) -> Vec<Vec<char>> {
    input.split_terminator('\n').filter(|l| !l.is_empty())
        .map(|l| l.spli)
}

// fn part1()

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod test {

    const TEST: &str = r#"
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
    "#;

    #[test]
    fn test_part1() {

    }
}
