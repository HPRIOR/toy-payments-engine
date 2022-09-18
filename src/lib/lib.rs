use std::{error::Error, ffi::OsString};

use csv_processor_mod::csv_processor::CsvProcessor;
use io_mod::csv_io::{output_csv, process_csv};

mod client_mod;
mod csv_processor_mod;
mod io_mod;

pub fn process_payments(csv_path: &OsString) -> Result<String, Box<dyn Error>> {
    let rows = process_csv(csv_path)?;

    let mut row_processor = CsvProcessor::new();
    row_processor.process_rows(&rows);
    let clients = row_processor.client_results();

    let result = output_csv(&clients)?;
    Ok(result)
}
