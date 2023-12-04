use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn answer1_1(reader: BufReader<File>) -> io::Result<()> {
    let mut sum = 0;
    for line in reader.lines() {
        let value = line.unwrap();
        let list_ar: Vec<&str> = value.matches(char::is_numeric).collect();
        let num_to_sum = format!("{}{}", list_ar.first().unwrap(), list_ar.last().unwrap());
        sum = sum + num_to_sum.parse::<i32>().unwrap();
    }
    println!("sum: {}", sum);

    Ok(())
}

fn answer1_2(reader: BufReader<File>) -> io::Result<()> {
    let mut sum = 0;
    for line in reader.lines() {
        let value = line.unwrap();

        let j: Vec<String> = vec![
            "1", "2", "3", "4", "5", "6", "7", "8", "9", 
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
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

        fn to_numeric_string(val: String) -> String {
            if val == "one" {
                return "1".to_owned();
            } else if val == "two" {
                return "2".to_owned();
            } else if val == "three" {
                return "3".to_owned();
            } else if val == "four" {
                return "4".to_owned();
            } else if val == "five" {
                return "5".to_owned();
            } else if val == "six" {
                return "6".to_owned();
            } else if val == "seven" {
                return "7".to_owned();
            } else if val == "eight" {
                return "8".to_owned();
            } else if val == "nine" {
                return "9".to_owned();
            } else  {
                return val;
            } 
        }
        
        let to_sum = format!("{}{}", to_numeric_string(store_first), to_numeric_string(store_last));
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
