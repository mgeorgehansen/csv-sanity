extern crate csv_sanity;

extern crate serde_json;
#[macro_use]
extern crate log;
extern crate regex;
#[macro_use]
extern crate clap;

use csv_sanity::cli::{
    self,
    Cli,
};

use std::fs::File;
use std::path::Path;
use log::{
    LogRecord,
    LogLevel,
    LogMetadata,
    LogLevelFilter,
    SetLoggerError
};
use clap::{
    App,
    Arg
};

struct ConsoleLogger {
    log_level: LogLevel
}

impl log::Log for ConsoleLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= self.log_level
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args())
        }
    }
}

fn init_logging() -> Result<(), SetLoggerError> {
    log::set_logger(|max_log_level| {
       max_log_level.set(LogLevelFilter::Info);
       Box::new(ConsoleLogger { log_level: LogLevel::Info })
   })
}

fn main() {
    init_logging().unwrap();

    let matches = App::new("Convert CSV")
        .version(crate_version!())
        .author("M. George Hansen <technopolitica@gmail.com>")
        .about("Apply a set of transformations to the records in a CSV file, attempting to read a much valid information from the file as possible.")
        .arg(Arg::with_name("INPUT_FILE")
            .help("CSV file to process")
            .required(true)
            .index(1))
        .arg(Arg::with_name("output")
            .help("File to output the transformed CSV records. Defaults to ./output.csv")
            .short("o")
            .long("output")
            .takes_value(true))
        .arg(Arg::with_name("error_output")
            .help("File to output errors in CSV format. Defaults to ./errors.csv")
            .short("e")
            .long("error_output")
            .takes_value(true))
        .arg(Arg::with_name("ruleset")
            .help("JSON file containing the ruleset to apply. Defaults to ./ruleset.json")
            .short("r")
            .long("ruleset")
            .takes_value(true))
        .get_matches();

    let ruleset_file_path = Path::new(matches.value_of("ruleset").unwrap_or("ruleset.json"));
    let ruleset_file = match File::open(ruleset_file_path) {
        Ok(f) => f,
        Err(e) => exit_with_error(&format!("unable to read ruleset file {}: {}", ruleset_file_path.display(), e))
    };
    let ruleset = match serde_json::from_reader(ruleset_file) {
        Ok(r) => r,
        Err(e) => {
            exit_with_error(&format!("failed to parse ruleset from {}: {}", ruleset_file_path.display(), e));
        }
    };

    let cli_app = Cli::new_with_options(ruleset, cli::Options {
        csv_options: cli::CsvOptions {
            delimiter: b'\t',
            .. Default::default()
        },
        .. Default::default()
    });

    // NOTE: Required arguments are validated by clap, so we should be safe to use expect here.
    let input_file_name = matches.value_of("INPUT_FILE").expect("INPUT_FILE argument could not be found!");
    let output_file_name = matches.value_of("output_file").unwrap_or("output.csv");
    let error_file_name = matches.value_of("error_file").unwrap_or("errors.csv");
    cli_app.run(input_file_name, output_file_name, error_file_name);
}

fn exit_with_error(error_msg: &str) -> !
{
    error!("{}", error_msg);
    std::process::exit(1);
}
