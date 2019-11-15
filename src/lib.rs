
use std::collections::HashMap;
use std::io::{self, prelude::*, stdout, Write,  BufReader, BufWriter};



#[derive(Debug)]
struct SeqLite  {
    head:   Vec<String>,  // Strings, str
    id:     Vec<String>,    // Strings, str
    seq:    Vec<u8>,     // u8 bitvec
    qual:   Vec<u8>,    // u8 bitvec
    rindex: HashMap<String, Vec<usize>>, // associate seq with start pos
    mindex: Vec<usize>,   // where do seqs start
    findex: Vec<usize>,  // on which seq does the next file starts
    format: String,
}


impl  SeqLite
    //where T: Hash + Eq,
{
    pub fn new (rtype: &str)-> Self{
        SeqLite{
            head: Vec::new(),
            id: Vec::new(),
            seq: Vec::new(),
            qual: Vec::new(),
            rindex: HashMap::new(),
            mindex: Vec::new(),
            findex: Vec::new(),
            format: rtype.to_string(),
        }
    }

    fn upload_fasta<R: BufRead>(&mut self,  reader:  R ) -> Result<bool,Error> {

        // check if fasta
        let mut i= self.head.len();
        self.findex.push(i);  // not sure what this is about...

        for line in reader.lines() {
            let str = line.unwrap();
            if &str[..1] == ">" {
                self.head.push(str.clone());
                let id = (&str[1..str.find(" ").unwrap()]).to_string();
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

        // check if raw

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


}





// Implement traits

pub trait IO{
    fn read (mut self, file: &str)-> Self;
    fn write (mut self, file: &str)-> Self;
}



impl IO for SeqLite{
    fn read(mut self, file: &str)->Self{

        let mut stdio;
        let mut file;
        let read: &mut dyn Read = match file {
            "stdin" => {
                stdio = io::stdin();
                &mut stdio
            },
            _       => {
                file = File::open(file).expect("Error opening input file");
                &mut file
            }
        };

        let reader = BufReader::new(read);

        match self.rtype {
            "fasta" => {
                self.upload_fasta(reader)?;

            },
            "fastq" => {

            },
            "raw"   => {

            }
        }

    }
    fn write(mut self,file: &str) -> Self{
        match file {
            "stdout" => {},
            _   => {}
        }
    }
}



// make an the object



// implement the objects


// IO trait

//Query trait

// Shrink trait





#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
