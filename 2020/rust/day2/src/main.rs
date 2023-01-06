use std::{env, fs, io};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Expected one text file as input");
        return Ok(());
    }

    let file = fs::File::open(&args[1])?;
    let lines = io::BufRead::lines(io::BufReader::new(file));

    let mut expenses_up_to_2020 = lines
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

    expenses_up_to_2020.sort();

    let split_index = expenses_up_to_2020.len() / 2;
    dbg!(&split_index);
    if split_index > 2 {
        for expense in expenses_up_to_2020
            .split_at(expenses_up_to_2020.len() / 2)
            .0
        {
            let delta = 2020 - expense;
            if expenses_up_to_2020.binary_search(&delta).is_ok() {
                println!(
                    "found expense {} * {} = {}",
                    expense,
                    &delta,
                    expense * delta
                );
                return Ok(());
            }
        }
    }

    Ok(())
}
