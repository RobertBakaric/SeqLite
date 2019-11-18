mod utils;
use std::collections::HashMap;
use utils::error::Error;
use std::io::{self, prelude::*, stdout, Write, Read, BufReader, BufWriter};
use std::fs::File;
use utils::io::{make_reader, make_writer};


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

    pub fn set_llen(mut self, llen: usize)-> Self{
        self.llen = llen;
        self
    }



    // internal
    fn fasta_up<R: BufRead>(&mut self,  reader:  R ) -> Result<bool,Error> {

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

    fn fastq_up<R: BufRead>(&mut self,  reader:  R ) -> Result<bool,Error> {

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

    fn txt_up<R: BufRead>(&mut self,  reader:  R ) -> Result<bool,Error> {

        // check if rawlist

        for line in reader.lines() {
            let str = line.unwrap();
            self.mindex.push(self.seq.len());
            self.seq.extend(str.as_bytes());
        }
        Ok(true)
    }

    fn bin_up<R: BufRead>(&mut self,  reader:  R ) -> Result<bool,Error> {

        // check if bin

        for line in reader.lines() {
            let str = line.unwrap();
            self.seq.extend(str.as_bytes());
        }
        Ok(true)
    }

    fn fasta_dw<W: Write> (&self, mut writer:  W)  -> Result<bool,Error>  {

        for i in 0..self.qres.len()-1 {
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

    fn fastq_dw<W: Write> (&self, mut writer:  W)   -> Result<bool,Error>  {

        for i in 0..self.qres.len()-1 {
            writeln!(writer, "{}", self.head[i]).unwrap();
            let mut en = self.mindex[i+1];
            let mut st = self.mindex[i];
            writer.write_all(&self.seq[st..en]).unwrap();
            writer.write(b"\n+\n").unwrap();
            writer.write_all(&self.qual[st..en]).unwrap();
            writer.write(b"\n").unwrap();
        }
        Ok(true)

    }

    fn txt_dw<W: Write>  (&self, mut writer:  W)  -> Result<bool,Error> {

        Ok(true)
    }


}

/**

# Design :

let sdb =  SeqLite::new("fastq");

// constructor
sdb.upload("file.fq|stdin").compress("lzt");

let table = sdb.select("rand|list").get("raw|table");

sdb.dump("file.dmp");

sdb.write("file.out|stdout");

**/



// Implement traits

pub trait IO{
    fn upload  (mut self, file: &str)-> Self;
    fn download (&self, file: &str)-> Result<bool,Error>;
}



impl IO for SeqLiteDb{

    fn upload(mut self, file: &str)->Self{

        let mut reader = make_reader(file);

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
            _        => {panic!("Format {} not supported !", self.format)}
        }

        self

    }
/*
    fn dump (mut self,    file: &str) -> Self {

        match &self.format[..] {
            "fasta" => {
                let mut all = vec![0; self.id.len()];
                for i in 0..self.id.len() {
                    all[i] = i;
                }

                self.write(file, all);

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
            _      => {panic!("Format {} not supported !", self.format)}
        }

        self
    }
*/
    fn download(&self, file: &str) -> Result<bool,Error>{

        let mut writer = make_writer(file);

        match &self.format[..] {
            "fasta" => {
                //self.fasta_dw(writer);

                if let  Ok(true) = self.fasta_dw(writer) {
                    println!("Data downloaded into {} [fa]  !", file);
                };

            },
            "fastq" => {
/*
                if let Ok(true) = self.fastq_dw(writer) {
                    println!("Data downloaded into {} [fq] !", file);
                };
*/
            },
            "raw"   => {

                if let Ok(true) = self.txt_dw(writer) {
                    println!("Data downloaded into {} [txt] !", file);
                };

            }
            _        => {panic!("Format {} not supported !", self.format)}
        }
        Ok(true)
    }


}


//Query trait

pub trait Query {
    // select conditions:
    //  a) rand(10)
    //  b) max(len|lcp)
    //  c) min(len|lcp)
    //  d) pos_list(1,2,52,5,33,67,322,4,56)
    //  e) id_list(brca, M12, ssb, TP53)
    //  f) regex(regEx(.*?)\t)
    fn select (&mut self, condition: String)-> &mut Self ;
    fn get (&mut self) -> &mut Self;
//    fn parse(exp: String)-> Result<(String,Vec<usize>),Error>;
}

// Shrink trait


impl Query for SeqLiteDb {

    fn select (&mut self, condition: String) -> &mut Self {

        match &condition[..] {
            "all" => {
                let mut all = vec![0; self.id.len()];
                for i in 0..self.id.len() {
                    all[i] = i;
                }
                self.qres = all;
            },
            _     => {
                //let (f,val): (String,Vec<usize>) = self.parse("all(xx)".to_string()).unwrap();
            }


        }


        self

    }
    fn get (&mut self) ->  &mut Self {

        self

    }
    /*
    fn parse(exp: String)-> Result<(String,Vec<usize>),Error>{

        Ok(("all".to_string(), Vec::new()))

    }

*/
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
