mod utils;
use std::collections::HashMap;
use utils::error::Error;
use std::io::{self, prelude::*, stdout, Write, Read, BufReader, BufWriter};
use std::fs::File;
use utils::io::{make_reader, make_writer};


#[derive(Debug)]
pub struct SeqLiteDb  {
    head:   Vec<String>,  // Strings, str
    id:     Vec<String>,    // Strings, str
    seq:    Vec<u8>,     // u8 bitvec
    qual:   Vec<u8>,    // u8 bitvec
    rindex: HashMap<String, Vec<usize>>, // associate seq with start pos
    mindex: Vec<usize>,   // where do seqs start
    findex: Vec<usize>,  // on which seq does the next file starts
    format: String,
    llen: usize,
    getter: String,
    qres: Vec<usize>
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
            head: Vec::new(),
            id: Vec::new(),
            seq: Vec::new(),
            qual: Vec::new(),
            rindex: HashMap::new(),  // replace with the faser one
            mindex: Vec::new(),
            findex: Vec::new(),
            format: typ,
            llen: 80,
            getter: "".to_string(),
            qres: Vec::new(),
        }
    }

    // getters

    pub fn get_fmt(&self)-> String{
        self.format.clone()
    }
//    pub fn get_seqset(&self)-> Vec<u8>{
//        self.seq.clone()
//    }

    // setters

    pub fn set_llen(&mut self, llen: usize)-> &mut Self{
        self.llen = llen;
        self
    }



    // internal
    fn upload_fasta<R: BufRead>(&mut self,  reader:  R ) -> Result<bool,Error> {

        // check if fasta
        let mut i= self.head.len();
        self.findex.push(i);  // not sure what this is about...

        for line in reader.lines() {
            let str = line.unwrap();
            if &str[..1] == ">" {
                self.head.push(str.clone());
                let id = (&str[1..str.find(" ").unwrap_or_else(|| str.len())]).to_string();
                self.id.push(id.clone());
                self.rindex.entry(id).or_insert(Vec::new()).push(i);
                self.mindex.push(self.seq.len());
                i=i+1;
                continue;
            }
            self.seq.extend(str.as_bytes());
        }
        Ok(true)
    }

    fn upload_fastq<R: BufRead>(&mut self,  reader:  R ) -> Result<bool,Error> {

        // check if fastq

        let mut i= self.head.len();
        let mut cnt=0;
        self.findex.push(i);  // not sure what this is about...

        for line in reader.lines() {
            let  str = line.unwrap();
            if &str[..1] == "@" && cnt == 0 {
                self.head.push(str.clone());
                let id = (&str[1..str.find(" ").unwrap()]).to_string();
                self.id.push(id.clone());
                self.rindex.entry(id).or_insert(Vec::new()).push(i);
                i=i+1;
            }else if cnt == 1 {
                self.mindex.push(self.seq.len());
                self.seq.extend(str.as_bytes());
            }else if cnt == 3 {
                self.qual.extend(str.as_bytes());
                cnt = 0;
                continue;
            }
            cnt = cnt+1;
        }
        Ok(true)
    }

    fn upload_txt<R: BufRead>(&mut self,  reader:  R ) -> Result<bool,Error> {

        // check if rawlist

        for line in reader.lines() {
            let str = line.unwrap();
            self.mindex.push(self.seq.len());
            self.seq.extend(str.as_bytes());
        }
        Ok(true)
    }

    fn upload_bin<R: BufRead>(&mut self,  reader:  R ) -> Result<bool,Error> {

        // check if bin

        for line in reader.lines() {
            let str = line.unwrap();
            self.seq.extend(str.as_bytes());
        }
        Ok(true)
    }

    fn download_fasta <W: Write>(&self, mut writer:  W, pos: Vec<usize> ) -> Result<bool,Error> {

        for i in 0..pos.len()-1 {
            writeln!(writer, "{}", self.head[i]).unwrap();
            let mut en = if self.mindex[i] + self.llen < self.mindex[i+1] {
                self.mindex[i] + self.llen
            }else{
                self.mindex[i+1]
            };
            let mut st = self.mindex[i];
            while st <  self.mindex[i+1] {
                writer.write_all(&self.seq[st..en]).unwrap();  // need to fx this
                writer.write(b"\n").unwrap();                  // need to fx this
                st = en;
                en = if st + self.llen < self.mindex[i+1] {
                    st + self.llen
                }else{
                    self.mindex[i+1]
                };
            }
        }

        writer.flush().unwrap();

        Ok(true)

    }

    fn download_fastq () {

    }

    fn download_txt () {

    }


}

/**

# Design :

let sdb =  SeqLite::new("fastq");

// constructor
sdb.read("file.fq|stdin").compress("lzt");

let table = sdb.select("rand|list").get("raw|table");

sdb.dump("file.dmp");

sdb.write("file.out|stdout");

**/



// Implement traits

pub trait IO{
    fn read  (mut self, file: &str)-> Self;
    fn write (mut self, file: &str)-> Self;
    fn dump  (self,    file: &str) -> Result<bool, Error>;
}



impl IO for SeqLiteDb{

    fn read(mut self, file: &str)->Self{

        let mut reader = make_reader(file);

        match &self.format[..] {
            "fasta" => {

                if let  Ok(true) = self.upload_fasta(reader) {
                    println!("File {} uploaded !", file);
                };

            },
            "fastq" => {

                if let Ok(true) = self.upload_fastq(reader) {
                    println!("File {} uploaded !", file);
                };

            },
            "raw"   => {

                if let Ok(true) = self.upload_txt(reader) {
                    println!("File {} uploaded !", file);
                };

            }
            _        => {panic!("Format {} not supported !", self.format)}
        }

        self

    }

    fn dump (self,    file: &str) -> Result<bool, Error> {
        let mut writer = make_writer(file);
        match &self.format[..] {
            "fasta" => {
                let mut all = vec![0; self.id.len()];
                for i in 0..self.id.len() {
                    all[i] = i;
                }

                if let  Ok(true) = self.download_fasta(writer, all) {
                    println!("Data downloaded into {}  !", file);
                };

            },
/*            "fastq" => {

                if let Ok(true) = self.upload_fastq(reader) {
                    println!("File {} uploaded !", file);
                };

            },
            "raw"   => {

                if let Ok(true) = self.upload_txt(reader) {
                    println!("File {} uploaded !", file);
                };

            } */
            _      => {panic!("Format {} not supported !", self.format)}
        }

        Ok(true)
    }

    fn write(mut self, file: &str) -> Self{

        let mut writer = make_writer(file);
        match &self.format[..] {
            "fasta" => {

                if let  Ok(true) = self.download_fasta(writer,self.qres) {
                    println!("Data downloaded into {}  !", file);
                };

            },
            "fastq" => {

                if let Ok(true) = self.download_fastq(writer) {
                    println!("File {} uploaded !", file);
                };

            },
            "raw"   => {

                if let Ok(true) = self.download_txt(writer) {
                    println!("File {} uploaded !", file);
                };

            }
            _        => {panic!("Format {} not supported !", self.format)}
        }
        self
    }


}


//Query trait

pub trait Query {
    fn qwhere <F: Fn> (mut self, condition: F) -> Self;
    fn qmatch (mut self, pattern: String) -> Self;
}

// Shrink trait


impl Query for SeqLiteDb {

    fn qwhere () -> Self {

    }
    fn qmatch {} -> Self {
        
    }


}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
