extern crate csv;
extern crate rustc_serialize;
extern crate rusqlite;

use csv::{Reader};
use rusqlite::Connection;

#[derive(RustcDecodable, Debug)]
struct Record {
    date: String,
    accession: String,
    exam: String,
    modality: String,
    site_code: String,
    reader: String
}

//store the record in a database
fn store_record(record: Vec<String>){
    println!("storing {:#?}", record);
}

//Remove white space and uppercase string
fn clean_string (s: &String) -> String {
        s.trim().to_uppercase().to_string()
    }

//Cleans all strings int the vector.  See clean_string
fn clean_record(record : Vec<String>) -> Vec<String>{
    record.iter().map(|x| clean_string(x)).collect::<Vec<_>>()
}

fn main() {
    let conn = Connection::open_in_memory().unwrap();
    println!("Parsing csv...");
    let file = "/Users/toddroth/Desktop/productivity.csv";
    let mut rdr = Reader::from_file(file).unwrap().has_headers(true);
    let mut records = Vec::new();
    for row in rdr.decode() {
        let rec :Vec<String> = row.unwrap();
        let cr = clean_record(rec);
        records.push(cr);

    }
    println!("{:?}", records.len());
    print!("{:?}", records[0]);
}
