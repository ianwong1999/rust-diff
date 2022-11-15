mod diff;

use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::ops::Range;

pub struct Config {
    file1_path: String,
    file2_path: String,
}

fn read_file_by_lines(file_name: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let file = File::open(file_name)?;
    let buf = BufReader::new(file);
    Ok(buf.lines().map(|l| l.unwrap()).collect())
}

fn range_to_string(r: &Range<usize>) -> String {
    if r.start + 1 >= r.end {
        format!("{}", r.end)
    } else {
        format!("{},{}", r.start + 1, r.end)
    }
}

fn print_diff(lhs: &Vec<String>, rhs: &Vec<String>, x: Range<usize>, y: Range<usize>) {
    let change_command = if x.start == x.end {
        "a"
    } else if y.start == y.end {
        "d"
    } else {
        "c"
    };

    println!(
        "{}{}{}",
        range_to_string(&x),
        change_command,
        range_to_string(&y),
    );

    for i in x {
        println!("<{}", lhs[i]);
    }

    if change_command == "c" {
        println!("---");
    }

    for i in y {
        println!(">{}", rhs[i]);
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let lhs = read_file_by_lines(&config.file1_path)?;
    let rhs = read_file_by_lines(&config.file2_path)?;

    let trace = diff::diff(&lhs, &rhs).trace;

    let mut x = 0;
    let mut y = 0;

    for (trace_x, trace_y) in trace {
        if (trace_x, trace_y) == (x, y) {
            x += 1;
            y += 1;
            continue;
        }

        dbg!((x, y, trace_x, trace_y));

        print_diff(&lhs, &rhs, x..trace_x, y..trace_y);

        x = trace_x + 1;
        y = trace_y + 1;
    }

    if (x, y) != (lhs.len(), rhs.len()) {
        print_diff(&lhs, &rhs, x..lhs.len(), y..rhs.len());
    }

    Ok(())
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let file1_path = args[1].clone();
        let file2_path = args[2].clone();

        Ok(Config {
            file1_path,
            file2_path,
        })
    }
}
