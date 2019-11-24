use crate::utils::error::{Error};
use crate::SeqLiteDb;
use core::str::from_utf8;


impl SeqLiteDb{

    pub(crate) fn delete_data (&mut self)-> &mut Self{

        assert!(self.qres.len() > 0, "There is nothing to delete! ");

        let mut j = 0;
        let mut vec = self.qres.clone();

        vec.sort_unstable();
        let mut all : Vec<usize> = Vec::with_capacity(self.id.len());


        for i in 0..self.id.len() {
            if vec[j] == i {
                while vec[j] == i && j < vec.len()-1 {
                    j=j+1;
                }
                continue;
            }
            all.push(i);
        }

        self.qres = all;
        self
    }

}





#[cfg(test)]
mod tests {
    #[test]
    fn delete() {

        use crate::SeqLiteDb;
        use crate::IO;
        use crate::Queries;


        let _sdb = SeqLiteDb::new("fasta")
            .upload("./data/rand_short.fa")
            .select("rand(3)".to_string())
            .delete()
            .download("./data/del.tmp").unwrap();

    }
}
