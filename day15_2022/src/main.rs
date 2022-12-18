use rayon::prelude::*;
use std::cmp::{max, min};

use regex::Regex;

fn manhatten_distance(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    i32::abs(p1.0 - p2.0) + i32::abs(p1.1 - p2.1)
}

fn decode_input(input: &str) -> (Vec<[i32; 5]>, i32, i32) {
    let re = Regex::new(
        "Sensor at x=(-?[0-9]+), y=(-?[0-9]+): closest beacon is at x=(-?[0-9]+), y=(-?[0-9]+)",
    )
    .unwrap();
    let mut min_x = 0;
    let mut max_x = 0;
    (
        input
            .split_terminator('\n')
            .filter(|l| !l.is_empty())
            .map(|l| {
                re.captures_iter(l)
                    .map(|cap| {
                        let mut v = [
                            cap[1].parse().unwrap(),
                            cap[2].parse().unwrap(),
                            cap[3].parse().unwrap(),
                            cap[4].parse().unwrap(),
                            0,
                        ];
                        // calculate manhatten distance
                        v[4] = manhatten_distance((v[0], v[1]), (v[2], v[3]));
                        // the x dimension of the field is the outermost sensors plus their measured distance
                        min_x = min(min_x, v[0] - v[4]);
                        max_x = max(max_x, v[0] + v[4]);
                        v
                    })
                    .last()
                    .unwrap()
            })
            .collect::<Vec<_>>(),
        min_x,
        max_x,
    )
}

fn part1(row: i32, input: &[[i32; 5]], min_x: i32, max_x: i32) -> usize {
    // we scan the search row starting from the fields outer
    // x dimensions. For each point on that row we check
    // if it is in the shadow of a sensor. In case the point
    // is a beacon itself we have to ignore the point!
    (min_x..=max_x)
        .filter(|&x| {
            input.iter().any(|s| {
                (x, row) != (s[2], s[3]) && manhatten_distance((x, row), (s[0], s[1])) <= s[4]
            })
        })
        .count()
}

fn part2(input: &[[i32; 5]], min_xy: i32, max_xy: i32) -> usize {
    // This time we search within a window of our field. We search
    // for the coordinates that are not in reach of any sensor.
    // Therefore the the beacon itself should not be ignored this time!
    let row_len = (max_xy - min_xy) as usize;
    let mut field = vec![false; row_len * row_len];
    input.iter().for_each(|s| {
        for x in 0..=s[4] {
            for y in s[4]..=x {
                let pos = ((s[1] + y) as usize * row_len) + (s[0] + x) as usize;
                if pos < field.len() {
                    field[pos] = true;
                }
                if s[1] >= y && s[0] >= x {
                    let pos = ((s[1] - y) as usize * row_len) + (s[0] - x) as usize;
                    if pos < field.len() {
                        field[pos] = true;
                    }
                }
            }
        }
    });
    dbg!(&field);
    if let Some((i, _)) = field.par_iter().enumerate().find_first(|&(_, f)| !f) {
        ((i / row_len) as usize * 4000000) + i % row_len
    } else {
        0
    }
    //     let mut x = min_xy;
    //     let mut y = min_xy;
    //     let mut found = false;
    //     while y <= max_xy {
    //         while x <= max_xy {
    //             let delta = input
    //                 .par_iter()
    //                 .map(|s| s[4] - manhatten_distance((x, y), (s[0], s[1])))
    //                 .find_first(|&d| d >= 0);
    //             if let Some(delta) = delta {
    //                 x += delta + 1;
    //                 continue;
    //             } else {
    //                 found = true;
    //                 break;
    //             }
    //         }
    //         if found {
    //             return (x * 4000000 + y) as usize;
    //         } else {
    //             x = min_xy;
    //             y += 1;
    //         }
    //     }
    //     0
}

fn main() {
    let (sensors, min_x, max_x) = decode_input(INPUT);
    println!("part1 {}", part1(2000000, &sensors, min_x, max_x));
    println!("part2 {}", part2(&sensors, 0, 4000000));
}

#[cfg(test)]
mod test {
    use crate::{decode_input, part1, part2};

    #[test]
    fn test_part1_one_sensor() {
        let (input, min_x, max_x) =
            decode_input("Sensor at x=8, y=7: closest beacon is at x=2, y=10");
        assert_eq!(12, part1(10, &input, min_x, max_x));
    }

    #[test]
    fn test_part1() {
        let (input, min_x, max_x) = decode_input(TEST);
        assert_eq!(26, part1(10, &input, min_x, max_x));
    }

    #[test]
    fn test_part2() {
        let (input, _, _) = decode_input(TEST);
        assert_eq!(56000011, part2(&input, 0, 20));
    }

    const TEST: &str = r#"
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
"#;
}

const INPUT: &str = r#"
Sensor at x=220580, y=684270: closest beacon is at x=436611, y=263737
Sensor at x=3329538, y=3016377: closest beacon is at x=3355914, y=2862466
Sensor at x=2605308, y=2023938: closest beacon is at x=2197530, y=2271330
Sensor at x=1810202, y=3423309: closest beacon is at x=1829362, y=3182862
Sensor at x=480296, y=3999646: closest beacon is at x=1694700, y=4178942
Sensor at x=46556, y=1283362: closest beacon is at x=-91140, y=1441882
Sensor at x=3741660, y=3959257: closest beacon is at x=3537901, y=3368697
Sensor at x=3399994, y=700264: closest beacon is at x=3748004, y=2000000
Sensor at x=1531981, y=3801761: closest beacon is at x=1694700, y=4178942
Sensor at x=193367, y=2712458: closest beacon is at x=-91140, y=1441882
Sensor at x=3199067, y=2194575: closest beacon is at x=3748004, y=2000000
Sensor at x=1878117, y=2578817: closest beacon is at x=2197530, y=2271330
Sensor at x=2439089, y=3168242: closest beacon is at x=1829362, y=3182862
Sensor at x=273443, y=171076: closest beacon is at x=436611, y=263737
Sensor at x=3680413, y=2477027: closest beacon is at x=3748004, y=2000000
Sensor at x=3620241, y=2904998: closest beacon is at x=3355914, y=2862466
Sensor at x=1728351, y=2895399: closest beacon is at x=1829362, y=3182862
Sensor at x=1894207, y=1168355: closest beacon is at x=2197530, y=2271330
Sensor at x=856867, y=3271314: closest beacon is at x=1829362, y=3182862
Sensor at x=3056788, y=2626224: closest beacon is at x=3355914, y=2862466
Sensor at x=3598024, y=3322247: closest beacon is at x=3537901, y=3368697
Sensor at x=1662543, y=3128823: closest beacon is at x=1829362, y=3182862
Sensor at x=3992558, y=1933059: closest beacon is at x=3748004, y=2000000
Sensor at x=1844282, y=2994285: closest beacon is at x=1829362, y=3182862
Sensor at x=3604375, y=3668021: closest beacon is at x=3537901, y=3368697
Sensor at x=2569893, y=3911832: closest beacon is at x=1694700, y=4178942
Sensor at x=117970, y=37503: closest beacon is at x=436611, y=263737
Sensor at x=3951385, y=3125577: closest beacon is at x=3537901, y=3368697
Sensor at x=2482373, y=2648092: closest beacon is at x=2197530, y=2271330
Sensor at x=915040, y=1835970: closest beacon is at x=-91140, y=1441882
Sensor at x=3047883, y=3301452: closest beacon is at x=3537901, y=3368697
Sensor at x=117432, y=1503889: closest beacon is at x=-91140, y=1441882
Sensor at x=1136011, y=261705: closest beacon is at x=436611, y=263737
Sensor at x=2343111, y=66183: closest beacon is at x=2081841, y=-807749
Sensor at x=608229, y=955721: closest beacon is at x=436611, y=263737
Sensor at x=1189379, y=3999750: closest beacon is at x=1694700, y=4178942
Sensor at x=766640, y=26597: closest beacon is at x=436611, y=263737
Sensor at x=3891093, y=2110588: closest beacon is at x=3748004, y=2000000
"#;
