extern crate clap;
extern crate serde;
extern crate chrono;
extern crate csv;

use clap::{App, Arg};
use chrono::prelude::*;
use serde::Deserialize;
//use std::io;
use std::error::Error;
use std::process;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct DevPattern {
    #[serde(rename = "OriginLength")]
    origin_lenght: u32,
    #[serde(rename = "DevelopmentPeriod")]
    development_period: u32,
    #[serde(rename = "Pattern")]
    pattern: f64
}

fn create_pattern_matrix(incremental_pattern: Vec<f64>) -> Vec<Vec<f64>> {
    let mut result: Vec<Vec<f64>> = Vec::new();

    let ln = incremental_pattern.len();

    for i in 0..ln {
        let mut patt: Vec<f64> = Vec::new();
        let mut sum: f64 = 0.0_f64;
        for j in i..ln {
            sum = sum + incremental_pattern[j];
        }
        for j in i..ln {
            patt.push(if sum != 0.0_f64 { incremental_pattern[j] / sum } else { 0.0_f64 });
        }
        result.push(patt);
    }

    return result;
}

fn convert_to_increment_pattern(cumulative_pattern: Vec<f64>) -> Vec<f64> {
    let mut result: Vec<f64> = Vec::new();

    let mut prev_val: f64 = 0.0_f64;
    for val in cumulative_pattern {
        let incr = val - prev_val;
        prev_val = val;
        result.push(incr);
    }

    return result;
}

fn read_payment_pattern(filename: &str, delimiter: u8) -> Result<Vec<f64>, Box<dyn Error>> {
    let mut result: Vec<f64> = Vec::new();

    //::from_reader(io::stdin());
    let mut rdr = csv::ReaderBuilder::new().delimiter(delimiter).from_path(Path::new(filename))?;

    let mut dev: u32 = 0;
    let mut patt: f64 = 0.0_f64;

    //let mut records = rdr.records()
    //   .collect::<Result<Vec<csv::StringRecord>, csv::Error>>()?;
    //records.sort_by(|r1, r2| r1[0].cmp(&r2[0]));

    for row in rdr.deserialize() {
        let record: DevPattern = row?;

        dev = dev + 1;
        assert!(record.development_period == dev, "Invalid development period received");

        patt = record.pattern;

        result.push(patt);
    }

    assert!(dev > 0, "No development points received");

    if patt != 1.0_f64 {
        result.push(1.0_f64);
    }

    Ok(result)
}

fn create_cashflow(amounts_filename: &str, delimiter: u8, cashflow_filename: &str) -> Result<(), Box<dyn Error>> {
    //, cashflow_matrix: Vec<Vec<f64>>
    let mut rdr = csv::ReaderBuilder::new().delimiter(delimiter).from_path(Path::new(amounts_filename))?;

    let headers = rdr.headers()?;
    let column_count: usize = headers.len();
    let mut amount_column_idx: usize = usize::max_value();
    let mut origin_month_column_idx: usize = usize::max_value();
    let mut occurence_month_column_idx: usize = usize::max_value();

    for (idx, col) in headers.iter().enumerate() {
        println!("c = {} {}", idx, col);

        match col {
            "OriginMonth" => { origin_month_column_idx = idx },
            "OccurenceMonth" => { occurence_month_column_idx = idx },
            "Amount" => { amount_column_idx = idx }
            _ => {}
        }
    }

    assert!(amount_column_idx != usize::max_value(), "Amount column not found in file");
    assert!(origin_month_column_idx != usize::max_value(), "OriginMonth column not found in file");
    assert!(occurence_month_column_idx != usize::max_value(), "OccurenceMonth column not found in file");

    for row in rdr.records() {
        let mut amount: f64 = 0.0_f64;
        let mut origin_month: Date<FixedOffset>;
        let mut occurence_month: Date<FixedOffset>;
        for(idx, val) in row?.iter().enumerate() {
            if idx == amount_column_idx { amount = val.replace(",", ".").parse::<f64>()?; }
            if idx == origin_month_column_idx { origin_month = DateTime::parse_from_str(val, "%Y-%m-%d")?.date(); }
            if idx == occurence_month_column_idx { occurence_month = DateTime::parse_from_str(val, "%Y-%m-%d")?.date(); }
        }

        println!("amount = {}", amount);
    }

    Ok(())
}

fn main() {
    let matches = App::new("Create cashflow")
                        .version("0.1")
                        .author("Pavels Nikolajevs <paavels@gmail.com>")
                        .about("Creates cashflow from cashflow pattern and amounts file")
                        .arg(Arg::with_name("CASHFLOW_PATTERN")
                            .required(true)
                            .index(1)
                            .value_name("CASHFLOW_PATTERN")
                            .help("Filename for payment pattern in CSV format. Delimiter: semicolon, required columns: OriginLength, DevelopmentPeriod, Pattern. Pattern by default cumulative."))
                        .arg(Arg::with_name("AMOUNTS")
                            .required(true)
                            .index(2)
                            .value_name("AMOUNTS")
                            .help("Filename for amounts file in CSV format. Delimiter: semicolon, required columns: BookingMonth, AccidentPeriod, Amount. Pattern will be applied onto this file."))
                        .arg(Arg::with_name("CASHFLOW")
                            .required(true)
                            .index(3)
                            .value_name("CASHFLOW")
                            .help("Filename for resulting output cashflow."))
                        .arg(Arg::with_name("delimiter")
                               .short("d")
                               .long("delimiter")
                               .multiple(false)
                               .takes_value(true)
                               .default_value(";")
                               .help("Sets the CSV delimiter"))
                        .get_matches();

    let parm_cashflow_pattern_file = matches.value_of("CASHFLOW_PATTERN").unwrap();
    let parm_amounts_file = matches.value_of("AMOUNTS").unwrap();
    let parm_cashflow_file = matches.value_of("CASHFLOW").unwrap();
    
    let parm_delimiter = matches.value_of("delimiter").unwrap();
    let delimiter: u8 = parm_delimiter.as_bytes()[0];

    let cumulative_cashflow_pattern: Vec<f64>;
    match read_payment_pattern(parm_cashflow_pattern_file, delimiter) {
        Ok(v) => { cumulative_cashflow_pattern = v; }
        Err(err) => {
            println!("Error reading cashflow pattern file: {}", err);
            process::exit(1);
        }
    }

    let cashflow_pattern: Vec<f64> = convert_to_increment_pattern(cumulative_cashflow_pattern);
    let cashflow_matrix = create_pattern_matrix(cashflow_pattern);

    match create_cashflow(parm_amounts_file, delimiter, parm_cashflow_file) {
        Ok(_) => { },
        Err(err) => {
            println!("Error reading cashflow pattern file: {}", err);
            process::exit(1);
        }
    }

    println!("result = {:?}", cashflow_matrix);

}
