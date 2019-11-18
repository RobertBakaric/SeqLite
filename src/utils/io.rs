use std::io::{self, prelude::*, stdout, Write, Read, BufReader, BufWriter};
use std::fs::File;

pub fn make_reader(file: &str)-> BufReader<Box< dyn Read >> {

    let tmp : Box<dyn Read> = match file {
        "stdin" => {
            Box::new(io::stdin())
        },
        _       => {
            Box::new(File::open(file)
                .expect(&(format!("Error opening {} file",file))))
        }
    };
    BufReader::new(tmp)
}


pub fn make_writer (file: &str)-> BufWriter<Box<dyn Write>> {

    let tmp : Box<dyn Write> = match file {
        "stdout" => {
            Box::new(io::stdout())
        },
        _       => {
            Box::new(File::create(file)
                .expect(&(format!("Error opening {} file",file))))
        }
    };
    BufWriter::new(tmp)
}
