use crate::utils::error::{Error};
use crate::SeqLiteDb;
use core::str::from_utf8;


impl SeqLiteDb{

    pub(crate) fn export_seq (&self) -> Result<Vec<String>,Error>{
        let mut res = Vec::new();

        for pos in self.qres.clone().into_iter() {

            let st = self.mindex[pos];
            let en = if pos < self.mindex.len() -1 {
                self.mindex[pos+1]
            }else{
                self.seq.len()
            };
            res.push(from_utf8(&self.seq[st..en]).unwrap().to_string());
        }

        Ok(res)
    }

    pub(crate) fn export_head (&self) -> Result<Vec<String>,Error>{
        let mut res = Vec::new();

        for pos in self.qres.clone().into_iter() {
            res.push(self.head[pos].clone());
        }

        Ok(res)
    }

    pub(crate) fn export_qual (&self) -> Result<Vec<String>,Error>{
        let mut res = Vec::new();

        for pos in self.qres.clone().into_iter() {
            let en = if pos < self.mindex.len() -1 {
                self.mindex[pos+1]
            }else{
                self.seq.len()
            };
            let st = self.mindex[pos];
            res.push(from_utf8(&self.qual[st..en]).unwrap().to_string());
        }

        Ok(res)
    }

    pub(crate) fn export_rid (&self) -> Result<Vec<String>,Error>{
        let mut res = Vec::new();

        for pos in self.qres.clone().into_iter() {
            res.push(self.id[pos].clone());
        }

        Ok(res)
    }

    pub(crate) fn export_seq_vec (&self) -> Result<Vec<u8>,Error>{
        Ok(self.seq.clone())
    }

    pub(crate) fn export_qual_vec (&self) -> Result<Vec<u8>,Error>{
        Ok(self.qual.clone())
    }

}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
