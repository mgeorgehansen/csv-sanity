//! Command line interface.

use std::fs::File;
use std::path::Path;

use {
    Ruleset,
    TransformError,
    TransformedRecord,
};

use csv;

/// Configuration options for the `Cli`.
pub struct Options
{
    /// See `CsvOptions`.
    pub csv_options: CsvOptions,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            csv_options: Default::default(),
        }
    }
}

/// `Cli` configuration options specific to how to parse the CSV file.
///
/// `CsvOptions` implements `Default` with the following defaults:
///
/// ```
/// extern crate csv;
/// use csv_sanity::cli::CsvOptions;
/// use csv::RecordTerminator;
///
/// let defaults = CsvOptions {
///     delimiter: b',',
///     record_terminator: csv::RecordTerminator::CRLF,
///     quote: b'"',
///     escape: None,
///     double_quote: true,
/// };
/// assert_eq!(defaults, Default::default());
/// ```
pub struct CsvOptions
{
    /// Field delimeter to expect in the CSV file.
    ///
    /// Corresponds to the `csv::Reader.delimiter` method.
    pub delimiter: u8,
    /// Record terminator to expect in the CSV file.
    ///
    /// Corresponds to the `csv::Reader.record_terminator` method. See `csv::RecordTerminator`.
    pub record_terminator: csv::RecordTerminator,
    /// Field quotation character to expect in the CSV file.
    ///
    /// Corresponds to the `csv::Reader.quote` method.
    pub quote: u8,
    /// Escape character to expect in the CSV file.
    ///
    /// Corresponds to the `csv::Reader.escape` method.
    pub escape: Option<u8>,
    /// Whether two adjacent quote characters should be interpreted as an escaped quote character.
    ///
    /// Corresponds to the `csv::Reader.double_quote` method.
    pub double_quote: bool
}

impl Default for CsvOptions
{
    fn default() -> CsvOptions {
        CsvOptions {
            delimiter: b',',
            record_terminator: csv::RecordTerminator::CRLF,
            quote: b'"',
            escape: None,
            double_quote: true,
        }
    }
}

/// Command line interface for running a `Ruleset` against a CSV file.
pub struct Cli
{
    options: Options,
    ruleset: Ruleset,
}

impl Cli
{
    /// Construct a new `Cli` with default options.
    ///
    /// ```
    /// use csv_sanity::Ruleset;
    /// use csv_sanity::cli::{
    ///     Cli
    /// };
    ///
    /// let ruleset = Ruleset::new();
    /// let cli = Cli::new(ruleset);
    /// ```
    pub fn new(ruleset: Ruleset) -> Cli {
        Self::new_with_options(ruleset, Default::default())
    }

    /// Construct a new `Cli` with the specified options.
    ///
    /// ```
    /// use csv_sanity::Ruleset;
    /// use csv_sanity::cli::{
    ///     Cli,
    ///     Options,
    ///     CsvOptions
    /// };
    ///
    /// let ruleset = Ruleset::new();
    /// let cli = Cli::new_with_options(ruleset, Options {
    ///     csv_options: CsvOptions {
    ///         delimiter: b',',
    ///         .. Default::default()
    ///     },
    ///     .. Default::default()
    /// });
    /// ```
    pub fn new_with_options(ruleset: Ruleset, options: Options) -> Cli {
        Cli {
            options: options,
            ruleset: ruleset,
        }
    }

    pub fn run<I: AsRef<Path>, O: AsRef<Path>, E: AsRef<Path>>(&self, input_file_path: I, output_file_name: O, error_file_name: E) {
        let (mut reader, headers) = self.reader_from_file(input_file_path);

        let mut output_writer = csv::Writer::from_file(output_file_name).expect("Unable to open output file for writing");
        let mut output_headers = headers.clone();
        output_headers.insert(0, "Record Number".to_string());
        output_writer.encode(output_headers).expect("Unable to write to output file");

        let mut error_writer = csv::Writer::from_file(error_file_name).expect("Unable to open error file for writing");
        let error_headers = vec![
            "Record Number",
            "Field Name",
            "Field Value",
            "Reason",
        ];
        error_writer.encode(error_headers).expect("Unable to write to error file");

        for (record_n, record) in reader.records().enumerate() {
            let original_line_n = record_n + 2; // Plus one for headers and plus one for zero-indexing.
            let transformed_record: TransformedRecord = match record {
                Err(e) => {
                    let err = TransformError {
                        field_value: "".to_string(),
                        field_name: "".to_string(),
                        record_n: original_line_n,
                        reason: format!("{}", e),
                    };
                    error_writer.encode(err).expect("Unable to write to error file");
                    continue;
                },
                Ok(ref rec) => self.ruleset.apply_rules(&headers, rec, original_line_n)
            };
            let record_fields: Vec<Option<String>> = {
                let mut fs = vec![Some(original_line_n.to_string())];
                fs.extend(transformed_record.field_values);
                fs
            };
            output_writer.encode(record_fields).expect("Unable to write to output file");
            for error in transformed_record.errors {
                error_writer.encode(error).expect("Unable to write to error file");
            }
        }
    }

    fn reader_from_file<P: AsRef<Path>>(&self, path: P) -> (csv::Reader<File>, Vec<String>) {
        let mut reader = csv::Reader::from_file(path.as_ref().clone()).map(|r| {
            // Configure the reader according to the options passed to the Cli constructor.
            r.has_headers(true)
                .delimiter(self.options.csv_options.delimiter)
                .record_terminator(self.options.csv_options.record_terminator)
                .quote(self.options.csv_options.quote)
                .escape(self.options.csv_options.escape)
                .double_quote(self.options.csv_options.double_quote)
                .flexible(true)
        }).expect(&format!("Unable to read file {}", path.as_ref().display()));
        let headers = reader.headers()
            .expect(&format!("Unable to read headers from input file {}", path.as_ref().display()));
        (reader, headers)
    }
}
