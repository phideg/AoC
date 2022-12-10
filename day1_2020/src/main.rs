use std::{env, fs, io};

fn calc_expenses(expenses: &Vec<u32>, n: usize) {
    let input_len = expenses.len();
    let mut set: Vec<usize> = vec![0; n];
    for i in 1..n {
        set[i] = i * (input_len / n as usize);
    }

    let split_index = expenses.len() / 2;
    if split_index > 2 {
        for expense in expenses.split_at(expenses.len() / 2).0.iter().enumerate() {
            let delta = 2020 - *expense.1;
            if let Ok(found_at) = expenses.binary_search(&delta) {
                if found_at != expense.0 {
                    println!(
                        "Part1: found expense {} * {} = {}",
                        &expense.1,
                        &delta,
                        expense.1 * &delta
                    );
                    return;
                }
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Expected text file and number as input");
        return Ok(());
    }

    let file = fs::File::open(&args[1])?;
    let lines = io::BufRead::lines(io::BufReader::new(file));
    let n = args[2]
        .parse::<usize>()
        .expect(&format!("{} is not a number", &args[2]));

    // only keep expenses < 2020
    let expenses_up_to_2020 = lines
        .into_iter()
        .filter_map(|line| {
            if let Ok(line) = line {
                if let Ok(num) = line.parse::<u32>() {
                    if num <= 2020 {
                        return Some(num);
                    }
                }
            }
            None
        })
        .collect::<Vec<u32>>();

    if n < 2 || n > expenses_up_to_2020.len() {
        return Ok(());
    }

    calc_expenses(&expenses_up_to_2020, n);

    Ok(())
}
