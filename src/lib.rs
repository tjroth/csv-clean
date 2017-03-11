extern crate csv;

use std::error::Error;
use csv::{Reader, Writer};

use std::path::Path;

pub struct Config {
    input_file: String,
    output_file: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let inp = args[1].clone();
        let outp = args[2].clone();
        Ok(Config { input_file: inp, output_file: outp })
    }
}

// Remove white space and uppercase string
fn clean_string(s: &String) -> String {
    s.trim().to_uppercase().to_string()
}

// Cleans all strings int the vector.  See clean_string
fn clean_record(record: Vec<String>) -> Vec<String> {
    record.iter().map(|x| clean_string(x)).collect::<Vec<_>>()
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut rdr = Reader::from_file(config.input_file)
        .expect("Unable to load file")
        .has_headers(true);

    let mut records = Vec::new();
    println!("Parsing csv...");
    for row in rdr.decode() {
        let rec: Vec<String> = row.unwrap();
        let cr = clean_record(rec);
        records.push(cr);
    }
    
    let out_file = Path::new(&config.output_file[..]);
    let mut wtr = Writer::from_file(out_file).expect("unable to create writer");
    for record in records.iter() {
        let result = wtr.encode(record);
        assert!(result.is_ok());
    }
    
    println!("{:?}", records.len());
    print!("{:?}", records[0]);
    Ok(())
}
