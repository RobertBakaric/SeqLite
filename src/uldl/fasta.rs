use crate::SeqLiteDb;
use crate::utils::error::Error;
use std::io::{self, prelude::*, stdout, Write, Read, BufReader, BufWriter};

impl SeqLiteDb{
    pub(crate) fn fasta_up<R: BufRead>(&mut self,  reader:  R ) -> Result<bool,Error> {

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

}
