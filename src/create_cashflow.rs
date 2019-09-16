extern crate clap;
extern crate serde;
extern crate csv;

use clap::{App, Arg};
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

fn read_payment_pattern(filename: &str, delimiter: u8) -> Result<(), Box<dyn Error>> {

    //::from_reader(io::stdin());
    let mut rdr = csv::ReaderBuilder::new().delimiter(delimiter).from_path(Path::new(filename))?;

    let mut cumulative_pattern: Vec<f64> = Vec::new();
    let mut dev: u32 = 0;
    let mut patt: f64 = 0.0_f64;

    //let mut records = rdr.records()
    //   .collect::<Result<Vec<csv::StringRecord>, csv::Error>>()?;
    //records.sort_by(|r1, r2| r1[0].cmp(&r2[0]));

    for result in rdr.deserialize() {
        let record: DevPattern = result?;

        dev = dev + 1;
        println!("x = {} {}", record.development_period, dev);
        assert!(record.development_period == dev, "Invalid development period received");

        patt = record.pattern;

        cumulative_pattern.push(patt);
    }

    assert!(dev > 0, "No development points received");

    if patt != 1.0_f64 {
        cumulative_pattern.push(1.0_f64);
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

    if let Err(err) = read_payment_pattern(parm_cashflow_pattern_file, delimiter) {
        println!("Error reading cashflow pattern file: {}", err);
        process::exit(1);
    }

}
