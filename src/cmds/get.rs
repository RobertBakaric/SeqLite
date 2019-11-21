use crate::utils::error::{Error};
use crate::SeqLiteDb;
use core::str::from_utf8;


impl SeqLiteDb{

    pub(crate) fn get_seq (&self) -> Result<Vec<String>,Error>{
        let mut res = Vec::new();
        println!("{:?}", self.qres);

        for pos in self.qres.clone().into_iter() {
            let en = if pos < self.seq.len() {
                self.mindex[pos+1]
            }else{
                self.seq.len()
            };
            let st = self.mindex[pos];
            res.push(from_utf8(&self.seq[st..en]).unwrap().to_string());
        }
        println!("{:?}", self.qres);

        Ok(res)
    }

    pub(crate) fn get_head (&self) -> Result<Vec<String>,Error>{
        let mut res = Vec::new();
        println!("{:?}", self.qres);

        for pos in self.qres.clone().into_iter() {
            res.push(self.head[pos].clone());
        }

        println!("{:?}", self.qres);

        Ok(res)
    }

    pub(crate) fn get_qual (&self) -> Result<Vec<String>,Error>{
        let mut res = Vec::new();

        for pos in self.qres.clone().into_iter() {
            let en = if pos < self.qual.len() {
                self.mindex[pos+1]
            }else{
                self.seq.len()
            };
            let st = self.mindex[pos];
            res.push(from_utf8(&self.qual[st..en]).unwrap().to_string());
        }

        Ok(res)
    }

    pub(crate) fn get_rid (&self) -> Result<Vec<String>,Error>{
        let mut res = Vec::new();

        for pos in self.qres.clone().into_iter() {
            res.push(self.id[pos].clone());
        }

        Ok(res)
    }

}
