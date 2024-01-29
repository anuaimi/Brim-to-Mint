use std::error::Error;
use std::fs::File;
// use chrono::NaiveDate;


fn transactions( input_file: &str, output_file: &str) -> Result<(), Box<dyn Error>> {

  let mut rdr = csv::ReaderBuilder::new()
    .has_headers(true) 
    .from_path(input_file)?;

  let mut wtr = csv::WriterBuilder::new()
    .from_writer(File::create(output_file)?);

  let output_headers = vec!["Date","Description","Original Description","Amount","Transaction Type","Category","Account Name","Labels","Notes"];
  wtr.write_record(output_headers)?; // Write the reversed fields to the output file


  for result in rdr.records() {
    let record = result?;
    let fields = record.iter().collect::<Vec<&str>>();

    let transaction_date = fields[1].to_string();
    let date = chrono::NaiveDate::parse_from_str(transaction_date.as_str(), "%Y-%m-%d")?;
    let transaction_payee = fields[3].to_string();
    let transaction_amount = fields[5].to_string();

    let mut output_fields = Vec::new();

    output_fields.push(date.format("%m/%d/%Y").to_string());
    output_fields.push(transaction_payee.clone());
    output_fields.push(transaction_payee);
    output_fields.push(transaction_amount);
    output_fields.push("".to_string());
    output_fields.push("".to_string());
    output_fields.push("".to_string());
    output_fields.push("".to_string());
    output_fields.push("".to_string());

    wtr.write_record(output_fields)?; 
  }
  
  wtr.flush()?;

  Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {

  // input file
  let input_file = "data/2024-0117-brim.csv";
  let output_file =  "data/2024-0117-mint-new.csv";

  if let Err(err) = transactions(input_file, output_file) {
    println!("error converting transactions: {}", err);
    // process::exit(1);
  }

  Ok(())
}