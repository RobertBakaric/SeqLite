use crate::SeqLiteDb;
use crate::utils::error::Error;
use std::io::{self, prelude::*, stdout, Write, Read, BufReader, BufWriter};

impl SeqLiteDb{
    pub(crate) fn fastq_up<R: BufRead>(&mut self,  reader:  R ) -> Result<bool,Error> {

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

    pub(crate) fn fastq_dw<W: Write> (&self, mut writer:  W)   -> Result<bool,Error>  {

        for pos in self.qres.clone().into_iter() {
            writeln!(writer, "{}", self.head[pos]).unwrap();
            let en = if pos < self.seq.len() {
                self.mindex[pos+1]
            }else{
                self.seq.len()
            };
            let st = self.mindex[pos];
            writer.write_all(&self.seq[st..en]).unwrap();
            writer.write(b"\n+\n").unwrap();
            writer.write_all(&self.qual[st..en]).unwrap();
            writer.write(b"\n").unwrap();
        }
        Ok(true)

    }

}
