
use std::io::{self, prelude::*, stdout, Write, Read, BufReader, BufWriter};
use std::fs::File;

pub fn make_reader(file: &str)-> BufReader<Box< dyn Read >> {

    let reader = match file {
        "stdin" => {
            let tmp : Box<dyn Read> = Box::new(io::stdin());
            BufReader::new(tmp)
        },
        _       => {
            let tmp : Box<dyn Read> = Box::new(File::open(file)
                .expect(&(format!("Error opening {} file",file))));
            BufReader::new(tmp)
        }
    };

    reader
}



pub fn make_writer (file: &str)-> BufWriter<Box<dyn Write>> {

    let writer = match file {
        "stdin" => {
            let tmp : Box<dyn Write> = Box::new(io::stdout());
            BufWriter::new(tmp)
        },
        _       => {
            let tmp : Box<dyn Write> = Box::new(File::open(file)
                .expect(&(format!("Error opening {} file",file))));
            BufWriter::new(tmp)
        }
    };

    writer
/*
    let mut stdout;
    let mut fout;
    let write: &mut dyn Write = match file {
        "stdout" => {
            stdout = io::stdout();
            &mut stdout
        },
        _       => {
            fout = File::open(file).expect(&(format!("Error writting to {} file",file)));
            &mut fout
        }
    };

    let mut writer = BufWriter::new(write);

    writer

    */
}
