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

    pub(crate)fn fasta_dw<W: Write> (&self, mut writer:  W)  -> Result<bool,Error>  {

        for pos in self.qres.clone().into_iter() {
            writeln!(writer, "{}", self.head[pos]).unwrap();
            let lindex = if pos < self.mindex.len()-1{
                self.mindex[pos+1]
            }else{
                self.seq.len()
            };
            let mut en = if self.mindex[pos] + self.llen < lindex  {
                self.mindex[pos] + self.llen
            }else{
                lindex
            };
            let mut st = self.mindex[pos];
            while st <  lindex {
                writer.write_all(&self.seq[st..en]).unwrap();  // need to fx this
                writer.write(b"\n").unwrap();                  // need to fx this
                st = en;
                en = if st + self.llen < lindex {
                    st + self.llen
                }else{
                    lindex
                };
            }
        }
        writer.flush().unwrap();
        Ok(true)
    }


}
