use crate::SeqLiteDb;
use crate::utils::error::Error;
use std::io::{self, prelude::*, stdout, Write, Read, BufReader, BufWriter};


impl SeqLiteDb {
    pub(crate) fn txt_up<R: BufRead>(&mut self,  reader:  R ) -> Result<bool,Error> {

        // check if rawlist

        for line in reader.lines() {
            let str = line.unwrap();
            self.mindex.push(self.seq.len());
            self.seq.extend(str.as_bytes());
        }
        Ok(true)
    }

    pub(crate) fn txt_dw<W: Write>  (&self, mut writer:  W)  -> Result<bool,Error> {

        for pos in self.qres.clone().into_iter(){
            let en = if pos < self.seq.len() - 1 {
                self.mindex[pos+1]
            }else{
                self.seq.len()
            };
            let st = self.mindex[pos];
            writer.write_all(&self.seq[st..en]).unwrap();
        }

        Ok(true)
    }

}
