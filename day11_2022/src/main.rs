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
"#;
