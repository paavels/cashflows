extern crate clap;

use clap:: {App, Arg, SubCommand};

fn main() {
    println!("create cf");

    let matches = App::new("Create cashflow")
                        .version("0.1")
                        .author("Pavels Nikolajevs <paavels@gmail.com>")
                        .about("Creates cashflow from payment pattern and reserve file")
                        .arg(Arg::with_name("PAYMENT_PATTERN")
                            .required(true)
                            .index(1)
                            .value_name("PAYMENT_PATTERN")
                            .help("Filename for payment pattern in CSV format. Delimiter: semicolon, required columns: OriginLength, DevelopmentPeriod, Pattern. Pattern by default cumulative."))
                        .arg(Arg::with_name("RESERVE")
                            .required(true)
                            .index(2)
                            .value_name("RESERVE")
                            .help("Filename for amounts file in CSV format. Delimiter: semicolon, required columns: BookingMonth, AccidentPeriod, Amount. Pattern will be applied onto this file."))
                        .arg(Arg::with_name("CASHFLOW")
                            .required(true)
                            .index(3)
                            .value_name("CASHFLOW")
                            .help("Filename for resulting output cashflow."))
                        .get_matches();

    let payment_pattern_file = matches.value_of("PAYMENT_PATTERN").unwrap();
    let reserve_file = matches.value_of("RESERVE").unwrap();
    let cashflow_file = matches.value_of("CASHFLOW").unwrap();
    println!("Value for PAYMENT_PATTERN: {}", payment_pattern_file);

}
