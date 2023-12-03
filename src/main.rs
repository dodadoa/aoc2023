use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn answer1_1(reader: BufReader<File>) -> io::Result<()> {
    let mut sum = 0;
    for line in reader.lines() {
        let value = line.unwrap();
        let list_ar = value.split("").collect::<Vec<&str>>();
        let list_str_num = list_ar
            .into_iter()
            .filter(|x| x.parse::<i32>().is_ok())
            .collect::<Vec<&str>>();
        let first = list_str_num.first().unwrap();
        let last = if list_str_num.len() > 1 {
            list_str_num.last().unwrap()
        } else {
            first
        };

        let num_to_sum = format!("{}{}", first, last);
        sum = sum + num_to_sum.parse::<i32>().unwrap();
    }
    println!("sum: {}", sum);

    Ok(())
}

fn answer1_2(reader: BufReader<File>) -> io::Result<()> {
    let mut sum = 0;
    for line in reader.lines() {
        let value = line.unwrap();

        let list_ar = value.split("").collect::<Vec<&str>>();
        let list_str_num = list_ar
            .into_iter()
            .filter(|x| x.parse::<i32>().is_ok())
            .collect::<Vec<&str>>();

        println!("{}", value);

        let j: Vec<String> = vec![
            "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five",
            "six", "seven", "eight", "nine",
        ]
        .into_iter()
        .map(|x| x.to_owned())
        .collect();

        let mut first_cal = 10000;
        let mut store_first = "".to_owned();
        for to_find in j.iter() {
            let first = value.find(to_find);
            if let Some(val) = first {
                if val < first_cal {
                    first_cal = val;
                    store_first = to_find.to_owned();
                }
            }
        }

        let mut last_cal = 0;
        let mut store_last = "".to_owned();
        for to_find in j.iter() {
            let last = value.rfind(to_find);
            if let Some(val) = last {
                if val >= last_cal {
                    last_cal = val;
                    store_last = to_find.to_owned();
                }
            }
        }

        if store_first == "one" {
            store_first = "1".to_owned();
        } else if store_first == "two" {
            store_first = "2".to_owned();
        } else if store_first == "three" {
            store_first = "3".to_owned();
        } else if store_first == "four" {
            store_first = "4".to_owned();
        } else if store_first == "five" {
            store_first = "5".to_owned();
        } else if store_first == "six" {
            store_first = "6".to_owned();
        } else if store_first == "seven" {
            store_first = "7".to_owned();
        } else if store_first == "eight" {
            store_first = "8".to_owned();
        } else if store_first == "nine" {
            store_first = "9".to_owned();
        } else if store_first == "zero" {
            store_first = "0".to_owned();
        }

        if store_last == "one" {
            store_last = "1".to_owned();
        } else if store_last == "two" {
            store_last = "2".to_owned();
        } else if store_last == "three" {
            store_last = "3".to_owned();
        } else if store_last == "four" {
            store_last = "4".to_owned();
        } else if store_last == "five" {
            store_last = "5".to_owned();
        } else if store_last == "six" {
            store_last = "6".to_owned();
        } else if store_last == "seven" {
            store_last = "7".to_owned();
        } else if store_last == "eight" {
            store_last = "8".to_owned();
        } else if store_last == "nine" {
            store_last = "9".to_owned();
        } else if store_last == "zero" {
            store_last = "0".to_owned();
        }

        println!("first: {}, last: {}", store_first, store_last);

        let to_sum = format!("{}{}", store_first, store_last);
        sum = sum + to_sum.parse::<i32>().unwrap();
    }
    println!("sum: {}", sum);

    Ok(())
}

fn main() -> io::Result<()> {
    let file_path = "./input/input1_2.txt";
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let _ = answer1_2(reader);

    Ok(())
}
