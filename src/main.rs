extern crate rustc_serialize;
extern crate csvclean;

use std::env;
use std::process;
// use std::error::Error;
use csvclean::Config;


#[derive(RustcDecodable, Debug)]
struct Record {
    date: String,
    accession: String,
    exam: String,
    modality: String,
    site_code: String,
    reader: String,
}


// store the record in a database
// fn store_record(record: Vec<String>) {
// println!("storing {:#?}", record);
// }
//


fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = csvclean::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
