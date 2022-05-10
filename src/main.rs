use chrono::{DateTime, NaiveDateTime};
use clap::{Arg, Command};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

fn insertion_sort(sort_by: &mut [i64]) {
    for index in 1..sort_by.len() {
        let current = sort_by[index];
        let mut swap_index = index as i64 - 1;
        while swap_index >= 0 && current <= sort_by[swap_index as usize] {
            sort_by[swap_index as usize + 1] = sort_by[swap_index as usize];
            swap_index -= 1;
        }
        sort_by[(swap_index + 1) as usize] = current;
    }
}

fn main() -> std::io::Result<()> {
    let matches = Command::new("Insertion sorter")
        .version("0.1.0")
        .author("John Marsden <jmmarsde@ncsu.edu>")
        .about("Quickly sorts log files from a large dataset")
        .arg(
            Arg::new("input")
                .index(1)
                .required(true)
                .help("The log file to sort"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .takes_value(true)
                .help("The file to output the sorted log file to"),
        )
        .get_matches();

    let input_name = matches.value_of("input").unwrap();
    let input_path = Path::new(input_name);
    let input_file = BufReader::new(File::open(input_path)?);

    let output_path = format!(
        "{}_output.{}",
        input_path.file_stem().unwrap().to_str().unwrap(),
        input_path.extension().unwrap().to_str().unwrap()
    );
    let output_name = matches
        .value_of("output")
        .unwrap_or_else(|| output_path.as_str());

    let mut dates: Vec<i64> = Vec::new();
    let logs_from_file = input_file.lines().flatten();

    for log in logs_from_file {
        dates.push(
            DateTime::parse_from_rfc3339(&log[0..25])
                .unwrap()
                .timestamp(),
        );
    }

    insertion_sort(dates.as_mut_slice());

    let mut output_file = BufWriter::new(File::create(output_name)?);
    for date in dates {
        writeln!(output_file, "{}", NaiveDateTime::from_timestamp(date, 0))?;
    }
    Ok(())
}
