extern crate csv;
extern crate rustc_serialize;

use csv::{Reader};

#[derive(RustcDecodable, Debug)]
struct Record {
    date: String,
    accession: String,
    exam: String,
    modality: String,
    site_code: String,
    reader: String
}
/*
fn cleanRecord(r : Record) -> Record {
let s = r.modality[..].trim();
r = Record {modality: String::from(s), ..r};
r
}*/
impl Record {
    fn clean(&mut self) {
        //modality
        let m = self.modality.clone();
        let trimmed = &m[..].trim();
        self.modality = trimmed.to_string();

        //site_code
        let sc = self.site_code.clone();
        let trimmed = &sc[..].trim();
        self.site_code = trimmed.to_string();

        //exam to_upper
        let ex = self.exam.clone();
        let trimmed = &ex[..].to_uppercase();
        self.exam = trimmed.to_string();
    }
}

fn store_record(record: Vec<String>){
    println!("storing {:#?}", record);
}

fn clean_record(record : Vec<String>) -> Vec<String>{
    fn cleaner (s: &String) -> String {
        let cleaned = s.trim().to_uppercase();
        cleaned.to_string()
    }
    record.iter().map(|x| cleaner(x)).collect::<Vec<_>>()
}

fn main() {
    println!("Parsing csv data...");
    let file = "/Users/toddroth/Desktop/productivity.csv";
    let mut rdr = Reader::from_file(file).unwrap().has_headers(true);   
    for row in rdr.decode() {
        let mut rec :Vec<String> = row.unwrap(); 
        let cr = clean_record(rec);
        store_record(cr);
    } 
}


