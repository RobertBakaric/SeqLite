mod utils;
mod cmds;
mod uldl;


use std::collections::HashMap;
use utils::error::Error;
//use std::io::{self, prelude::*, stdout, Write, Read, BufReader, BufWriter};
//use std::fs::File;
use utils::io::{make_reader, make_writer};



/**

# Design :

let sdb =  SeqLite::new("fastq");

// constructor
sdb.upload("file.fq|stdin").compress("lzt");

let table = sdb.select("rand|list").get("raw|table");

sdb.dump("file.dmp");

sdb.write("file.out|stdout");

**/


#[derive(Debug, Clone)]
pub struct SeqLiteDb  {
    head:   Vec<String>,  // Strings, str
    id:     Vec<String>,    // Strings, str
    seq:    Vec<u8>,     // u8 bitvec
    qual:   Vec<u8>,    // u8 bitvec
    rindex: HashMap<String, Vec<usize>>, // associate seq with start pos
    mindex: Vec<usize>,   // where do seqs start
    findex: Vec<usize>,  // on which seq does the next file starts
    format: String,
    llen:   usize,
    getter: String,
    qres:   Vec<usize>
}


impl  SeqLiteDb
    //where T: Hash + Eq,
{
    pub fn new (rtype: &str)-> Self{

        let typ : String   = match rtype {
            "fasta" | "fastq" | "raw" => rtype.to_string(),
             _ => panic!("File format {} not supported !",rtype ),
        };

        SeqLiteDb{
            head:   Vec::new(),
            id:     Vec::new(),
            seq:    Vec::new(),
            qual:   Vec::new(),
            rindex: HashMap::new(),  // replace with the faser one
            mindex: Vec::new(),
            findex: Vec::new(),
            format: typ,
            llen:   80,
            getter: "".to_string(),
            qres:   Vec::new(),
        }
    }

    // test getters

    pub fn get_fmt(&self)-> String{
        self.format.clone()
    }

    // direct builder

    pub fn set_llen(mut self, llen: usize)-> Self{
        self.llen = llen;
        self
    }

}
/*
    fn bin_up<R: BufRead>(&mut self,  reader:  R ) -> Result<bool,Error> {

        // check if bin

        for line in reader.lines() {
            let str = line.unwrap();
            self.seq.extend(str.as_bytes());
        }
        Ok(true)
    }set
*/






// Traits

pub trait IO{
    fn upload  (self, file: &str)-> Self;
    fn download (&self, file: &str)-> Result<bool,Error>;
}

impl IO for SeqLiteDb{

    fn upload(mut self, file: &str)->Self{

        let reader = make_reader(file);

        match &self.format[..] {
            "fasta" => {

                if let  Ok(true) = self.fasta_up(reader) {
                    println!("File {} uploaded !", file);
                };

            },
            "fastq" => {

                if let Ok(true) = self.fastq_up(reader) {
                    println!("File {} uploaded !", file);
                };

            },
            "raw"   => {

                if let Ok(true) = self.txt_up(reader) {
                    println!("File {} uploaded !", file);
                };

            }
            _        => {
                panic!("Format {} not supported !", self.format)
            }
        }

        self

    }

    fn download(&self, file: &str) -> Result<bool,Error>{

        let writer = make_writer(file);

        match &self.format[..] {
            "fasta" => {
                //self.fasta_dw(writer);

                if let  Ok(true) = self.fasta_dw(writer) {
                    println!("Data downloaded [fa] into {}   !", file);
                };

            },
            "fastq" => {

                if let Ok(true) = self.fastq_dw(writer) {
                    println!("Data downloaded [fq] into {}  !", file);
                };

            },
            "raw"   => {

                if let Ok(true) = self.txt_dw(writer) {
                    println!("Data downloaded [txt] into {}  !", file);
                };

            }
            _        => {
                panic!("Format {} not supported !", self.format)
            }
        }
        Ok(true)
    }


}



pub trait Getters {
    fn get_head   (&self) -> Result<Vec<String>,Error>;
    fn get_seq    (&self) -> Result<Vec<String>,Error>;
    fn get_qual   (&self) -> Result<Vec<String>,Error>;
    fn get_rid    (&self) -> Result<Vec<String>,Error>;
//    fn get_record (&self) -> Result<Vec<T>,Error>; //  set T to be a struct

}


impl Getters  for SeqLiteDb{

    fn get_head (&self) -> Result<Vec<String>,Error>{
        match &self.format[..] {
            "fasta" | "fastq" => {
                self.get_head()
            },
            _                  => {
                panic!("Header can only be obtained for : [fa,fq] file formats ")
            }
        }
    }
    fn get_seq (&self) -> Result<Vec<String>,Error>{
        match &self.format[..] {
            "fasta" | "fastq" | "raw" => {
                self.get_seq()
            },
            _                  => {
                panic!("Sequence can only be obtained for : [fa,fq,txt] file formats ")
            }
        }

    }
    fn get_qual (&self) -> Result<Vec<String>,Error>{
        match &self.format[..] {
            "fastq" => {
                self.get_qual()
            },
            _                  => {
                panic!("Quality can only be obtained for : [fq] file formats ")
            }
        }

    }
    fn get_rid (&self) -> Result<Vec<String>,Error>{
        match &self.format[..] {
            "fasta" | "fastq" => {
                self.get_rid()
            },
            _                  => {
                panic!("Record identifier can only be obtained for : [fa,fq] file formats ")
            }
        }
    }
}



// hm is this smart ?
pub trait Loaders {
    fn load_head   (&self) -> Result<Vec<String>,Error>;
    fn load_seq    (&self) -> Result<Vec<String>,Error>;
    fn load_qual   (&self) -> Result<Vec<String>,Error>;
    fn load_rid    (&self) -> Result<Vec<String>,Error>;
//    fn load_record (&self) -> Result<Vec<T>,Error>; //  set T to be a struct
}


impl Loaders  for SeqLiteDb{

    fn load_head (&self) -> Result<Vec<String>,Error>{
        match &self.format[..] {
            "fasta" | "fastq" => {
                self.get_head()
            },
            _                  => {
                panic!("Header can only be obtained for : [fa,fq] file formats ")
            }
        }
    }
    fn get_seq (&self) -> Result<Vec<String>,Error>{
        match &self.format[..] {
            "fasta" | "fastq" | "raw" => {
                self.get_seq()
            },
            _                  => {
                panic!("Sequence can only be obtained for : [fa,fq,txt] file formats ")
            }
        }

    }
    fn get_qual (&self) -> Result<Vec<String>,Error>{
        match &self.format[..] {
            "fastq" => {
                self.get_qual()
            },
            _                  => {
                panic!("Quality can only be obtained for : [fq] file formats ")
            }
        }

    }
    fn get_rid (&self) -> Result<Vec<String>,Error>{
        match &self.format[..] {
            "fasta" | "fastq" => {
                self.get_rid()
            },
            _                  => {
                panic!("Record identifier can only be obtained for : [fa,fq] file formats ")
            }
        }
    }
}




//Query trait

pub trait Queries {
    fn select (&mut self, condition: String)-> &mut Self ;

//    fn delete(&mut self)->&mut Self;
//    fn insert(&mut self)-> &mut Self;
//    fn update(&mut self)-> &mut Self;

}



impl Queries for SeqLiteDb {

    fn select (&mut self, condition: String) -> &mut Self {

        match &condition[..] {
            "all" => {
                self.seq_select_all();
            },
            _     => {
                let (func,val) :(String,Vec<usize>) = self.parse_condition(condition).unwrap();
                match &func[..] {
                    "rand" => {
                        self.seq_select_rand(val[0]);
                    },
                    "max"  => {
                        panic!("Condition not recognized!")
                    },
                    "min"  => {
                        panic!("Condition not recognized!")
                    },
                    "list" => {
                        panic!("Condition not recognized!")
                    },
                    "regex" => {
                        panic!("Condition not recognized!")
                    },
                    _      => {
                        panic!("Condition not recognized!")
                    }
                }
            }
        }
        self
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
