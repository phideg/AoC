#![feature(iter_array_chunks)]

use std::collections::HashSet;

#[derive(Debug, PartialEq)]
struct Cave {
    obstacles: HashSet<(usize, usize)>,
    height: usize,
    begin: usize,
    end: usize,
}

impl Cave {
    fn new() -> Self {
        Self {
            obstacles: HashSet::new(),
            height: 0,
            begin: usize::MAX,
            end: 0,
        }
    }
}

fn add_path_segment(p1: (usize, usize), p2: (usize, usize), map: &mut HashSet<(usize, usize)>) {
    assert!(p1.0.abs_diff(p2.0) == 0 || p1.1.abs_diff(p2.1) == 0);
    let x1 = p1.0.min(p2.0);
    let x2 = p1.0.max(p2.0);
    (x1..=x2).for_each(|x| {
        map.insert((x, p1.1));
    });
    let y1 = p1.1.min(p2.1);
    let y2 = p1.1.max(p2.1);
    (y1..=y2).for_each(|y| {
        map.insert((p1.0, y));
    });
}

fn decode_input(input: &str) -> Cave {
    let mut cave = Cave::new();
    input
        .split_terminator('\n')
        .filter(|line| !line.is_empty())
        .for_each(|l| {
            let mut p1 = Option::None;
            l.split_whitespace()
                .filter(|&t| t != "->")
                .map(|t| {
                    t.split_terminator(',')
                        .map(|v| v.parse().unwrap())
                        .array_chunks()
                        .next()
                        .unwrap()
                })
                .for_each(|[x, y]| {
                    let p2 = (x, y);
                    if let Some(p1) = p1 {
                        add_path_segment(p1, p2, &mut cave.obstacles);
                    }
                    cave.begin = p2.0.min(cave.begin);
                    cave.end = p2.0.max(cave.end);
                    cave.height = p2.1.max(cave.height);
                    p1 = Some(p2);
                });
        });
    cave
}

fn pour_sand(start: usize) {}

fn part1(input: &Cave) -> usize {
    0
}

// fn part2(input: &mut Vec<Vec<(usize, usize)>>) -> usize {}

fn main() {
    let mut input = decode_input(INPUT);
    println!("Part 1: {}", part1(&mut input));
    // println!("Part 2: {}", part2(&mut input));
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::Cave;

    #[test]
    fn test_decode() {
        let expected = Cave {
            obstacles: HashSet::from([(498, 4), (498, 5), (498, 6), (497, 6), (496, 6)]),
            height: 6,
            begin: 496,
            end: 498,
        };
        assert_eq!(expected, super::decode_input("498,4 -> 498,6 -> 496,6"));
    }

    #[test]
    fn test_part1() {
        let mut input = super::decode_input(TEST);
        assert_eq!(13_usize, super::part1(&mut input));
    }

    // #[test]
    // fn test_part2() {
    //     let mut input = super::decode_input(TEST);
    //     assert_eq!(140_usize, super::part2(&mut input));
    // }

    const TEST: &str = r#"
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
"#;
}

const INPUT: &str = r#"
532,137 -> 532,130 -> 532,137 -> 534,137 -> 534,127 -> 534,137 -> 536,137 -> 536,135 -> 536,137 -> 538,137 -> 538,127 -> 538,137 -> 540,137 -> 540,132 -> 540,137 -> 542,137 -> 542,135 -> 542,137 -> 544,137 -> 544,127 -> 544,137 -> 546,137 -> 546,131 -> 546,137 -> 548,137 -> 548,134 -> 548,137 -> 550,137 -> 550,133 -> 550,137
549,140 -> 549,143 -> 541,143 -> 541,147 -> 555,147 -> 555,143 -> 554,143 -> 554,140
519,98 -> 523,98
524,111 -> 524,109 -> 524,111 -> 526,111 -> 526,103 -> 526,111 -> 528,111 -> 528,110 -> 528,111 -> 530,111 -> 530,108 -> 530,111 -> 532,111 -> 532,108 -> 532,111 -> 534,111 -> 534,107 -> 534,111
511,50 -> 511,51 -> 525,51 -> 525,50
506,46 -> 506,43 -> 506,46 -> 508,46 -> 508,37 -> 508,46 -> 510,46 -> 510,40 -> 510,46 -> 512,46 -> 512,45 -> 512,46 -> 514,46 -> 514,38 -> 514,46
524,111 -> 524,109 -> 524,111 -> 526,111 -> 526,103 -> 526,111 -> 528,111 -> 528,110 -> 528,111 -> 530,111 -> 530,108 -> 530,111 -> 532,111 -> 532,108 -> 532,111 -> 534,111 -> 534,107 -> 534,111
555,150 -> 555,154 -> 550,154 -> 550,157 -> 561,157 -> 561,154 -> 557,154 -> 557,150
506,46 -> 506,43 -> 506,46 -> 508,46 -> 508,37 -> 508,46 -> 510,46 -> 510,40 -> 510,46 -> 512,46 -> 512,45 -> 512,46 -> 514,46 -> 514,38 -> 514,46
520,56 -> 525,56
524,111 -> 524,109 -> 524,111 -> 526,111 -> 526,103 -> 526,111 -> 528,111 -> 528,110 -> 528,111 -> 530,111 -> 530,108 -> 530,111 -> 532,111 -> 532,108 -> 532,111 -> 534,111 -> 534,107 -> 534,111
517,58 -> 522,58
513,94 -> 517,94
532,137 -> 532,130 -> 532,137 -> 534,137 -> 534,127 -> 534,137 -> 536,137 -> 536,135 -> 536,137 -> 538,137 -> 538,127 -> 538,137 -> 540,137 -> 540,132 -> 540,137 -> 542,137 -> 542,135 -> 542,137 -> 544,137 -> 544,127 -> 544,137 -> 546,137 -> 546,131 -> 546,137 -> 548,137 -> 548,134 -> 548,137 -> 550,137 -> 550,133 -> 550,137
498,70 -> 503,70
506,46 -> 506,43 -> 506,46 -> 508,46 -> 508,37 -> 508,46 -> 510,46 -> 510,40 -> 510,46 -> 512,46 -> 512,45 -> 512,46 -> 514,46 -> 514,38 -> 514,46
519,94 -> 523,94
515,30 -> 520,30
532,137 -> 532,130 -> 532,137 -> 534,137 -> 534,127 -> 534,137 -> 536,137 -> 536,135 -> 536,137 -> 538,137 -> 538,127 -> 538,137 -> 540,137 -> 540,132 -> 540,137 -> 542,137 -> 542,135 -> 542,137 -> 544,137 -> 544,127 -> 544,137 -> 546,137 -> 546,131 -> 546,137 -> 548,137 -> 548,134 -> 548,137 -> 550,137 -> 550,133 -> 550,137
524,58 -> 529,58
509,89 -> 509,80 -> 509,89 -> 511,89 -> 511,84 -> 511,89 -> 513,89 -> 513,88 -> 513,89 -> 515,89 -> 515,84 -> 515,89 -> 517,89 -> 517,88 -> 517,89
507,98 -> 511,98
549,140 -> 549,143 -> 541,143 -> 541,147 -> 555,147 -> 555,143 -> 554,143 -> 554,140
527,56 -> 532,56
532,137 -> 532,130 -> 532,137 -> 534,137 -> 534,127 -> 534,137 -> 536,137 -> 536,135 -> 536,137 -> 538,137 -> 538,127 -> 538,137 -> 540,137 -> 540,132 -> 540,137 -> 542,137 -> 542,135 -> 542,137 -> 544,137 -> 544,127 -> 544,137 -> 546,137 -> 546,131 -> 546,137 -> 548,137 -> 548,134 -> 548,137 -> 550,137 -> 550,133 -> 550,137
532,137 -> 532,130 -> 532,137 -> 534,137 -> 534,127 -> 534,137 -> 536,137 -> 536,135 -> 536,137 -> 538,137 -> 538,127 -> 538,137 -> 540,137 -> 540,132 -> 540,137 -> 542,137 -> 542,135 -> 542,137 -> 544,137 -> 544,127 -> 544,137 -> 546,137 -> 546,131 -> 546,137 -> 548,137 -> 548,134 -> 548,137 -> 550,137 -> 550,133 -> 550,137
524,111 -> 524,109 -> 524,111 -> 526,111 -> 526,103 -> 526,111 -> 528,111 -> 528,110 -> 528,111 -> 530,111 -> 530,108 -> 530,111 -> 532,111 -> 532,108 -> 532,111 -> 534,111 -> 534,107 -> 534,111
532,137 -> 532,130 -> 532,137 -> 534,137 -> 534,127 -> 534,137 -> 536,137 -> 536,135 -> 536,137 -> 538,137 -> 538,127 -> 538,137 -> 540,137 -> 540,132 -> 540,137 -> 542,137 -> 542,135 -> 542,137 -> 544,137 -> 544,127 -> 544,137 -> 546,137 -> 546,131 -> 546,137 -> 548,137 -> 548,134 -> 548,137 -> 550,137 -> 550,133 -> 550,137
532,137 -> 532,130 -> 532,137 -> 534,137 -> 534,127 -> 534,137 -> 536,137 -> 536,135 -> 536,137 -> 538,137 -> 538,127 -> 538,137 -> 540,137 -> 540,132 -> 540,137 -> 542,137 -> 542,135 -> 542,137 -> 544,137 -> 544,127 -> 544,137 -> 546,137 -> 546,131 -> 546,137 -> 548,137 -> 548,134 -> 548,137 -> 550,137 -> 550,133 -> 550,137
522,96 -> 526,96
524,111 -> 524,109 -> 524,111 -> 526,111 -> 526,103 -> 526,111 -> 528,111 -> 528,110 -> 528,111 -> 530,111 -> 530,108 -> 530,111 -> 532,111 -> 532,108 -> 532,111 -> 534,111 -> 534,107 -> 534,111
549,140 -> 549,143 -> 541,143 -> 541,147 -> 555,147 -> 555,143 -> 554,143 -> 554,140
535,60 -> 540,60
506,46 -> 506,43 -> 506,46 -> 508,46 -> 508,37 -> 508,46 -> 510,46 -> 510,40 -> 510,46 -> 512,46 -> 512,45 -> 512,46 -> 514,46 -> 514,38 -> 514,46
511,50 -> 511,51 -> 525,51 -> 525,50
506,46 -> 506,43 -> 506,46 -> 508,46 -> 508,37 -> 508,46 -> 510,46 -> 510,40 -> 510,46 -> 512,46 -> 512,45 -> 512,46 -> 514,46 -> 514,38 -> 514,46
524,111 -> 524,109 -> 524,111 -> 526,111 -> 526,103 -> 526,111 -> 528,111 -> 528,110 -> 528,111 -> 530,111 -> 530,108 -> 530,111 -> 532,111 -> 532,108 -> 532,111 -> 534,111 -> 534,107 -> 534,111
532,137 -> 532,130 -> 532,137 -> 534,137 -> 534,127 -> 534,137 -> 536,137 -> 536,135 -> 536,137 -> 538,137 -> 538,127 -> 538,137 -> 540,137 -> 540,132 -> 540,137 -> 542,137 -> 542,135 -> 542,137 -> 544,137 -> 544,127 -> 544,137 -> 546,137 -> 546,131 -> 546,137 -> 548,137 -> 548,134 -> 548,137 -> 550,137 -> 550,133 -> 550,137
555,150 -> 555,154 -> 550,154 -> 550,157 -> 561,157 -> 561,154 -> 557,154 -> 557,150
533,114 -> 533,118 -> 525,118 -> 525,124 -> 541,124 -> 541,118 -> 538,118 -> 538,114
549,140 -> 549,143 -> 541,143 -> 541,147 -> 555,147 -> 555,143 -> 554,143 -> 554,140
532,137 -> 532,130 -> 532,137 -> 534,137 -> 534,127 -> 534,137 -> 536,137 -> 536,135 -> 536,137 -> 538,137 -> 538,127 -> 538,137 -> 540,137 -> 540,132 -> 540,137 -> 542,137 -> 542,135 -> 542,137 -> 544,137 -> 544,127 -> 544,137 -> 546,137 -> 546,131 -> 546,137 -> 548,137 -> 548,134 -> 548,137 -> 550,137 -> 550,133 -> 550,137
524,111 -> 524,109 -> 524,111 -> 526,111 -> 526,103 -> 526,111 -> 528,111 -> 528,110 -> 528,111 -> 530,111 -> 530,108 -> 530,111 -> 532,111 -> 532,108 -> 532,111 -> 534,111 -> 534,107 -> 534,111
509,89 -> 509,80 -> 509,89 -> 511,89 -> 511,84 -> 511,89 -> 513,89 -> 513,88 -> 513,89 -> 515,89 -> 515,84 -> 515,89 -> 517,89 -> 517,88 -> 517,89
532,137 -> 532,130 -> 532,137 -> 534,137 -> 534,127 -> 534,137 -> 536,137 -> 536,135 -> 536,137 -> 538,137 -> 538,127 -> 538,137 -> 540,137 -> 540,132 -> 540,137 -> 542,137 -> 542,135 -> 542,137 -> 544,137 -> 544,127 -> 544,137 -> 546,137 -> 546,131 -> 546,137 -> 548,137 -> 548,134 -> 548,137 -> 550,137 -> 550,133 -> 550,137
513,98 -> 517,98
532,137 -> 532,130 -> 532,137 -> 534,137 -> 534,127 -> 534,137 -> 536,137 -> 536,135 -> 536,137 -> 538,137 -> 538,127 -> 538,137 -> 540,137 -> 540,132 -> 540,137 -> 542,137 -> 542,135 -> 542,137 -> 544,137 -> 544,127 -> 544,137 -> 546,137 -> 546,131 -> 546,137 -> 548,137 -> 548,134 -> 548,137 -> 550,137 -> 550,133 -> 550,137
555,150 -> 555,154 -> 550,154 -> 550,157 -> 561,157 -> 561,154 -> 557,154 -> 557,150
505,33 -> 510,33
532,137 -> 532,130 -> 532,137 -> 534,137 -> 534,127 -> 534,137 -> 536,137 -> 536,135 -> 536,137 -> 538,137 -> 538,127 -> 538,137 -> 540,137 -> 540,132 -> 540,137 -> 542,137 -> 542,135 -> 542,137 -> 544,137 -> 544,127 -> 544,137 -> 546,137 -> 546,131 -> 546,137 -> 548,137 -> 548,134 -> 548,137 -> 550,137 -> 550,133 -> 550,137
498,13 -> 498,16 -> 492,16 -> 492,19 -> 506,19 -> 506,16 -> 501,16 -> 501,13
524,111 -> 524,109 -> 524,111 -> 526,111 -> 526,103 -> 526,111 -> 528,111 -> 528,110 -> 528,111 -> 530,111 -> 530,108 -> 530,111 -> 532,111 -> 532,108 -> 532,111 -> 534,111 -> 534,107 -> 534,111
532,137 -> 532,130 -> 532,137 -> 534,137 -> 534,127 -> 534,137 -> 536,137 -> 536,135 -> 536,137 -> 538,137 -> 538,127 -> 538,137 -> 540,137 -> 540,132 -> 540,137 -> 542,137 -> 542,135 -> 542,137 -> 544,137 -> 544,127 -> 544,137 -> 546,137 -> 546,131 -> 546,137 -> 548,137 -> 548,134 -> 548,137 -> 550,137 -> 550,133 -> 550,137
533,114 -> 533,118 -> 525,118 -> 525,124 -> 541,124 -> 541,118 -> 538,118 -> 538,114
498,13 -> 498,16 -> 492,16 -> 492,19 -> 506,19 -> 506,16 -> 501,16 -> 501,13
532,137 -> 532,130 -> 532,137 -> 534,137 -> 534,127 -> 534,137 -> 536,137 -> 536,135 -> 536,137 -> 538,137 -> 538,127 -> 538,137 -> 540,137 -> 540,132 -> 540,137 -> 542,137 -> 542,135 -> 542,137 -> 544,137 -> 544,127 -> 544,137 -> 546,137 -> 546,131 -> 546,137 -> 548,137 -> 548,134 -> 548,137 -> 550,137 -> 550,133 -> 550,137
509,89 -> 509,80 -> 509,89 -> 511,89 -> 511,84 -> 511,89 -> 513,89 -> 513,88 -> 513,89 -> 515,89 -> 515,84 -> 515,89 -> 517,89 -> 517,88 -> 517,89
549,140 -> 549,143 -> 541,143 -> 541,147 -> 555,147 -> 555,143 -> 554,143 -> 554,140
532,137 -> 532,130 -> 532,137 -> 534,137 -> 534,127 -> 534,137 -> 536,137 -> 536,135 -> 536,137 -> 538,137 -> 538,127 -> 538,137 -> 540,137 -> 540,132 -> 540,137 -> 542,137 -> 542,135 -> 542,137 -> 544,137 -> 544,127 -> 544,137 -> 546,137 -> 546,131 -> 546,137 -> 548,137 -> 548,134 -> 548,137 -> 550,137 -> 550,133 -> 550,137
549,140 -> 549,143 -> 541,143 -> 541,147 -> 555,147 -> 555,143 -> 554,143 -> 554,140
509,89 -> 509,80 -> 509,89 -> 511,89 -> 511,84 -> 511,89 -> 513,89 -> 513,88 -> 513,89 -> 515,89 -> 515,84 -> 515,89 -> 517,89 -> 517,88 -> 517,89
533,114 -> 533,118 -> 525,118 -> 525,124 -> 541,124 -> 541,118 -> 538,118 -> 538,114
498,13 -> 498,16 -> 492,16 -> 492,19 -> 506,19 -> 506,16 -> 501,16 -> 501,13
509,89 -> 509,80 -> 509,89 -> 511,89 -> 511,84 -> 511,89 -> 513,89 -> 513,88 -> 513,89 -> 515,89 -> 515,84 -> 515,89 -> 517,89 -> 517,88 -> 517,89
509,89 -> 509,80 -> 509,89 -> 511,89 -> 511,84 -> 511,89 -> 513,89 -> 513,88 -> 513,89 -> 515,89 -> 515,84 -> 515,89 -> 517,89 -> 517,88 -> 517,89
524,111 -> 524,109 -> 524,111 -> 526,111 -> 526,103 -> 526,111 -> 528,111 -> 528,110 -> 528,111 -> 530,111 -> 530,108 -> 530,111 -> 532,111 -> 532,108 -> 532,111 -> 534,111 -> 534,107 -> 534,111
503,76 -> 513,76 -> 513,75
498,13 -> 498,16 -> 492,16 -> 492,19 -> 506,19 -> 506,16 -> 501,16 -> 501,13
532,137 -> 532,130 -> 532,137 -> 534,137 -> 534,127 -> 534,137 -> 536,137 -> 536,135 -> 536,137 -> 538,137 -> 538,127 -> 538,137 -> 540,137 -> 540,132 -> 540,137 -> 542,137 -> 542,135 -> 542,137 -> 544,137 -> 544,127 -> 544,137 -> 546,137 -> 546,131 -> 546,137 -> 548,137 -> 548,134 -> 548,137 -> 550,137 -> 550,133 -> 550,137
509,89 -> 509,80 -> 509,89 -> 511,89 -> 511,84 -> 511,89 -> 513,89 -> 513,88 -> 513,89 -> 515,89 -> 515,84 -> 515,89 -> 517,89 -> 517,88 -> 517,89
533,114 -> 533,118 -> 525,118 -> 525,124 -> 541,124 -> 541,118 -> 538,118 -> 538,114
521,60 -> 526,60
532,137 -> 532,130 -> 532,137 -> 534,137 -> 534,127 -> 534,137 -> 536,137 -> 536,135 -> 536,137 -> 538,137 -> 538,127 -> 538,137 -> 540,137 -> 540,132 -> 540,137 -> 542,137 -> 542,135 -> 542,137 -> 544,137 -> 544,127 -> 544,137 -> 546,137 -> 546,131 -> 546,137 -> 548,137 -> 548,134 -> 548,137 -> 550,137 -> 550,133 -> 550,137
532,137 -> 532,130 -> 532,137 -> 534,137 -> 534,127 -> 534,137 -> 536,137 -> 536,135 -> 536,137 -> 538,137 -> 538,127 -> 538,137 -> 540,137 -> 540,132 -> 540,137 -> 542,137 -> 542,135 -> 542,137 -> 544,137 -> 544,127 -> 544,137 -> 546,137 -> 546,131 -> 546,137 -> 548,137 -> 548,134 -> 548,137 -> 550,137 -> 550,133 -> 550,137
516,92 -> 520,92
498,13 -> 498,16 -> 492,16 -> 492,19 -> 506,19 -> 506,16 -> 501,16 -> 501,13
506,46 -> 506,43 -> 506,46 -> 508,46 -> 508,37 -> 508,46 -> 510,46 -> 510,40 -> 510,46 -> 512,46 -> 512,45 -> 512,46 -> 514,46 -> 514,38 -> 514,46
506,46 -> 506,43 -> 506,46 -> 508,46 -> 508,37 -> 508,46 -> 510,46 -> 510,40 -> 510,46 -> 512,46 -> 512,45 -> 512,46 -> 514,46 -> 514,38 -> 514,46
512,33 -> 517,33
532,137 -> 532,130 -> 532,137 -> 534,137 -> 534,127 -> 534,137 -> 536,137 -> 536,135 -> 536,137 -> 538,137 -> 538,127 -> 538,137 -> 540,137 -> 540,132 -> 540,137 -> 542,137 -> 542,135 -> 542,137 -> 544,137 -> 544,127 -> 544,137 -> 546,137 -> 546,131 -> 546,137 -> 548,137 -> 548,134 -> 548,137 -> 550,137 -> 550,133 -> 550,137
532,137 -> 532,130 -> 532,137 -> 534,137 -> 534,127 -> 534,137 -> 536,137 -> 536,135 -> 536,137 -> 538,137 -> 538,127 -> 538,137 -> 540,137 -> 540,132 -> 540,137 -> 542,137 -> 542,135 -> 542,137 -> 544,137 -> 544,127 -> 544,137 -> 546,137 -> 546,131 -> 546,137 -> 548,137 -> 548,134 -> 548,137 -> 550,137 -> 550,133 -> 550,137
528,60 -> 533,60
524,111 -> 524,109 -> 524,111 -> 526,111 -> 526,103 -> 526,111 -> 528,111 -> 528,110 -> 528,111 -> 530,111 -> 530,108 -> 530,111 -> 532,111 -> 532,108 -> 532,111 -> 534,111 -> 534,107 -> 534,111
524,111 -> 524,109 -> 524,111 -> 526,111 -> 526,103 -> 526,111 -> 528,111 -> 528,110 -> 528,111 -> 530,111 -> 530,108 -> 530,111 -> 532,111 -> 532,108 -> 532,111 -> 534,111 -> 534,107 -> 534,111
532,137 -> 532,130 -> 532,137 -> 534,137 -> 534,127 -> 534,137 -> 536,137 -> 536,135 -> 536,137 -> 538,137 -> 538,127 -> 538,137 -> 540,137 -> 540,132 -> 540,137 -> 542,137 -> 542,135 -> 542,137 -> 544,137 -> 544,127 -> 544,137 -> 546,137 -> 546,131 -> 546,137 -> 548,137 -> 548,134 -> 548,137 -> 550,137 -> 550,133 -> 550,137
498,13 -> 498,16 -> 492,16 -> 492,19 -> 506,19 -> 506,16 -> 501,16 -> 501,13
503,76 -> 513,76 -> 513,75
532,137 -> 532,130 -> 532,137 -> 534,137 -> 534,127 -> 534,137 -> 536,137 -> 536,135 -> 536,137 -> 538,137 -> 538,127 -> 538,137 -> 540,137 -> 540,132 -> 540,137 -> 542,137 -> 542,135 -> 542,137 -> 544,137 -> 544,127 -> 544,137 -> 546,137 -> 546,131 -> 546,137 -> 548,137 -> 548,134 -> 548,137 -> 550,137 -> 550,133 -> 550,137
506,46 -> 506,43 -> 506,46 -> 508,46 -> 508,37 -> 508,46 -> 510,46 -> 510,40 -> 510,46 -> 512,46 -> 512,45 -> 512,46 -> 514,46 -> 514,38 -> 514,46
532,137 -> 532,130 -> 532,137 -> 534,137 -> 534,127 -> 534,137 -> 536,137 -> 536,135 -> 536,137 -> 538,137 -> 538,127 -> 538,137 -> 540,137 -> 540,132 -> 540,137 -> 542,137 -> 542,135 -> 542,137 -> 544,137 -> 544,127 -> 544,137 -> 546,137 -> 546,131 -> 546,137 -> 548,137 -> 548,134 -> 548,137 -> 550,137 -> 550,133 -> 550,137
509,89 -> 509,80 -> 509,89 -> 511,89 -> 511,84 -> 511,89 -> 513,89 -> 513,88 -> 513,89 -> 515,89 -> 515,84 -> 515,89 -> 517,89 -> 517,88 -> 517,89
498,24 -> 513,24
506,46 -> 506,43 -> 506,46 -> 508,46 -> 508,37 -> 508,46 -> 510,46 -> 510,40 -> 510,46 -> 512,46 -> 512,45 -> 512,46 -> 514,46 -> 514,38 -> 514,46
524,111 -> 524,109 -> 524,111 -> 526,111 -> 526,103 -> 526,111 -> 528,111 -> 528,110 -> 528,111 -> 530,111 -> 530,108 -> 530,111 -> 532,111 -> 532,108 -> 532,111 -> 534,111 -> 534,107 -> 534,111
525,98 -> 529,98
532,137 -> 532,130 -> 532,137 -> 534,137 -> 534,127 -> 534,137 -> 536,137 -> 536,135 -> 536,137 -> 538,137 -> 538,127 -> 538,137 -> 540,137 -> 540,132 -> 540,137 -> 542,137 -> 542,135 -> 542,137 -> 544,137 -> 544,127 -> 544,137 -> 546,137 -> 546,131 -> 546,137 -> 548,137 -> 548,134 -> 548,137 -> 550,137 -> 550,133 -> 550,137
555,150 -> 555,154 -> 550,154 -> 550,157 -> 561,157 -> 561,154 -> 557,154 -> 557,150
511,27 -> 516,27
509,89 -> 509,80 -> 509,89 -> 511,89 -> 511,84 -> 511,89 -> 513,89 -> 513,88 -> 513,89 -> 515,89 -> 515,84 -> 515,89 -> 517,89 -> 517,88 -> 517,89
514,60 -> 519,60
532,137 -> 532,130 -> 532,137 -> 534,137 -> 534,127 -> 534,137 -> 536,137 -> 536,135 -> 536,137 -> 538,137 -> 538,127 -> 538,137 -> 540,137 -> 540,132 -> 540,137 -> 542,137 -> 542,135 -> 542,137 -> 544,137 -> 544,127 -> 544,137 -> 546,137 -> 546,131 -> 546,137 -> 548,137 -> 548,134 -> 548,137 -> 550,137 -> 550,133 -> 550,137
523,54 -> 528,54
511,50 -> 511,51 -> 525,51 -> 525,50
524,111 -> 524,109 -> 524,111 -> 526,111 -> 526,103 -> 526,111 -> 528,111 -> 528,110 -> 528,111 -> 530,111 -> 530,108 -> 530,111 -> 532,111 -> 532,108 -> 532,111 -> 534,111 -> 534,107 -> 534,111
549,140 -> 549,143 -> 541,143 -> 541,147 -> 555,147 -> 555,143 -> 554,143 -> 554,140
509,89 -> 509,80 -> 509,89 -> 511,89 -> 511,84 -> 511,89 -> 513,89 -> 513,88 -> 513,89 -> 515,89 -> 515,84 -> 515,89 -> 517,89 -> 517,88 -> 517,89
533,114 -> 533,118 -> 525,118 -> 525,124 -> 541,124 -> 541,118 -> 538,118 -> 538,114
506,46 -> 506,43 -> 506,46 -> 508,46 -> 508,37 -> 508,46 -> 510,46 -> 510,40 -> 510,46 -> 512,46 -> 512,45 -> 512,46 -> 514,46 -> 514,38 -> 514,46
508,30 -> 513,30
506,46 -> 506,43 -> 506,46 -> 508,46 -> 508,37 -> 508,46 -> 510,46 -> 510,40 -> 510,46 -> 512,46 -> 512,45 -> 512,46 -> 514,46 -> 514,38 -> 514,46
497,66 -> 502,66
532,137 -> 532,130 -> 532,137 -> 534,137 -> 534,127 -> 534,137 -> 536,137 -> 536,135 -> 536,137 -> 538,137 -> 538,127 -> 538,137 -> 540,137 -> 540,132 -> 540,137 -> 542,137 -> 542,135 -> 542,137 -> 544,137 -> 544,127 -> 544,137 -> 546,137 -> 546,131 -> 546,137 -> 548,137 -> 548,134 -> 548,137 -> 550,137 -> 550,133 -> 550,137
509,89 -> 509,80 -> 509,89 -> 511,89 -> 511,84 -> 511,89 -> 513,89 -> 513,88 -> 513,89 -> 515,89 -> 515,84 -> 515,89 -> 517,89 -> 517,88 -> 517,89
524,111 -> 524,109 -> 524,111 -> 526,111 -> 526,103 -> 526,111 -> 528,111 -> 528,110 -> 528,111 -> 530,111 -> 530,108 -> 530,111 -> 532,111 -> 532,108 -> 532,111 -> 534,111 -> 534,107 -> 534,111
532,137 -> 532,130 -> 532,137 -> 534,137 -> 534,127 -> 534,137 -> 536,137 -> 536,135 -> 536,137 -> 538,137 -> 538,127 -> 538,137 -> 540,137 -> 540,132 -> 540,137 -> 542,137 -> 542,135 -> 542,137 -> 544,137 -> 544,127 -> 544,137 -> 546,137 -> 546,131 -> 546,137 -> 548,137 -> 548,134 -> 548,137 -> 550,137 -> 550,133 -> 550,137
524,111 -> 524,109 -> 524,111 -> 526,111 -> 526,103 -> 526,111 -> 528,111 -> 528,110 -> 528,111 -> 530,111 -> 530,108 -> 530,111 -> 532,111 -> 532,108 -> 532,111 -> 534,111 -> 534,107 -> 534,111
531,58 -> 536,58
519,33 -> 524,33
555,150 -> 555,154 -> 550,154 -> 550,157 -> 561,157 -> 561,154 -> 557,154 -> 557,150
555,150 -> 555,154 -> 550,154 -> 550,157 -> 561,157 -> 561,154 -> 557,154 -> 557,150
501,63 -> 511,63
491,70 -> 496,70
506,46 -> 506,43 -> 506,46 -> 508,46 -> 508,37 -> 508,46 -> 510,46 -> 510,40 -> 510,46 -> 512,46 -> 512,45 -> 512,46 -> 514,46 -> 514,38 -> 514,46
516,96 -> 520,96
509,89 -> 509,80 -> 509,89 -> 511,89 -> 511,84 -> 511,89 -> 513,89 -> 513,88 -> 513,89 -> 515,89 -> 515,84 -> 515,89 -> 517,89 -> 517,88 -> 517,89
505,70 -> 510,70
532,137 -> 532,130 -> 532,137 -> 534,137 -> 534,127 -> 534,137 -> 536,137 -> 536,135 -> 536,137 -> 538,137 -> 538,127 -> 538,137 -> 540,137 -> 540,132 -> 540,137 -> 542,137 -> 542,135 -> 542,137 -> 544,137 -> 544,127 -> 544,137 -> 546,137 -> 546,131 -> 546,137 -> 548,137 -> 548,134 -> 548,137 -> 550,137 -> 550,133 -> 550,137
524,111 -> 524,109 -> 524,111 -> 526,111 -> 526,103 -> 526,111 -> 528,111 -> 528,110 -> 528,111 -> 530,111 -> 530,108 -> 530,111 -> 532,111 -> 532,108 -> 532,111 -> 534,111 -> 534,107 -> 534,111
501,68 -> 506,68
506,46 -> 506,43 -> 506,46 -> 508,46 -> 508,37 -> 508,46 -> 510,46 -> 510,40 -> 510,46 -> 512,46 -> 512,45 -> 512,46 -> 514,46 -> 514,38 -> 514,46
510,96 -> 514,96
533,114 -> 533,118 -> 525,118 -> 525,124 -> 541,124 -> 541,118 -> 538,118 -> 538,114
524,111 -> 524,109 -> 524,111 -> 526,111 -> 526,103 -> 526,111 -> 528,111 -> 528,110 -> 528,111 -> 530,111 -> 530,108 -> 530,111 -> 532,111 -> 532,108 -> 532,111 -> 534,111 -> 534,107 -> 534,111
498,13 -> 498,16 -> 492,16 -> 492,19 -> 506,19 -> 506,16 -> 501,16 -> 501,13
532,137 -> 532,130 -> 532,137 -> 534,137 -> 534,127 -> 534,137 -> 536,137 -> 536,135 -> 536,137 -> 538,137 -> 538,127 -> 538,137 -> 540,137 -> 540,132 -> 540,137 -> 542,137 -> 542,135 -> 542,137 -> 544,137 -> 544,127 -> 544,137 -> 546,137 -> 546,131 -> 546,137 -> 548,137 -> 548,134 -> 548,137 -> 550,137 -> 550,133 -> 550,137
555,150 -> 555,154 -> 550,154 -> 550,157 -> 561,157 -> 561,154 -> 557,154 -> 557,150
494,68 -> 499,68
533,114 -> 533,118 -> 525,118 -> 525,124 -> 541,124 -> 541,118 -> 538,118 -> 538,114
509,89 -> 509,80 -> 509,89 -> 511,89 -> 511,84 -> 511,89 -> 513,89 -> 513,88 -> 513,89 -> 515,89 -> 515,84 -> 515,89 -> 517,89 -> 517,88 -> 517,89
506,46 -> 506,43 -> 506,46 -> 508,46 -> 508,37 -> 508,46 -> 510,46 -> 510,40 -> 510,46 -> 512,46 -> 512,45 -> 512,46 -> 514,46 -> 514,38 -> 514,46
509,89 -> 509,80 -> 509,89 -> 511,89 -> 511,84 -> 511,89 -> 513,89 -> 513,88 -> 513,89 -> 515,89 -> 515,84 -> 515,89 -> 517,89 -> 517,88 -> 517,89
"#;
