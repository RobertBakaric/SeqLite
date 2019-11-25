use crate::utils::error::{Error};
use crate::SeqLiteDb;
use regex::Regex;
use rand::prelude::*;



// select conditions:
//  a) rand(10)
//  b) max(len|lcp)
//  c) min(len|lcp)
//  d) pos_list(1,2,52,5,33,67,322,4,56)
//  e) id_list(brca, M12, ssb, TP53)
//  f) regex(regEx(.*?)\t)


impl SeqLiteDb {

    pub(crate) fn parse_condition(&self,text: String) -> Result<(String,Vec<usize>),Error>{

        let re = Regex::new(r"(\w+)\((.*?)\)").unwrap();
        let cap = re.captures(&text).unwrap();
        let mut val = cap[2].to_string();
        val.retain(|c| !c.is_whitespace());

        let tokens: Vec<usize> = val
            .split(",")
            .map(|token| token.parse::<usize>().unwrap())
            .collect();
        Ok((cap[1].to_string(),tokens))

    }

    pub(crate) fn seq_select_all (&mut self) -> &mut Self {

        let mut all = vec![0; self.id.len()];
        for i in 0..self.id.len() {
            all[i] = i;
        }
        self.qres = all;
        self
    }

    pub(crate) fn seq_select_rand (&mut self, num: usize ) -> &mut Self {

        // generate num random positions

        let mut rng = thread_rng();

        let mut all = vec![0; self.id.len()];
        for i in 0..self.id.len() {
            all[i] = i;
        }
        all.shuffle(&mut rng);

        let vec: Vec<usize> = all[..num].to_vec();
        //println!("{:?}", vec);
        self.qres = vec;

        self
    }

    pub(crate) fn seq_select_min (&mut self, n: usize) -> &mut Self {

        let mut bottomx : Vec<usize> = Vec::with_capacity(n);

        for i in self.mindex.clone().into_iter(){
            bottomx.push(i);
            let mut j = bottomx.len() - 1;
            while j > 0 {
                if bottomx[j] > bottomx[j-1] {
                    break;
                }

                let t = bottomx[j];
                bottomx[j] = bottomx[j-1];
                bottomx[j-1] = t;

                j = j - 1;
            }
            if  bottomx.len() > n {
                bottomx.pop();
            }
        }
        self.qres = bottomx;

        self
    }

    pub(crate) fn seq_select_max (&mut self, n: usize ) -> &mut Self {

        let mut topx : Vec<usize> = Vec::with_capacity(n);

        for i in self.mindex.clone().into_iter(){
            topx.push(i);
            let mut j = topx.len() - 1;
            while j > 0 {
                if topx[j] < topx[j-1] {
                    break;
                }

                let t = topx[j];
                topx[j] = topx[j-1];
                topx[j-1] = t;

                j = j - 1;
            }
            if  topx.len() > n {
                topx.pop();
            }
        }
        self.qres = topx;
        self
    }

    pub(crate) fn seq_select_list<T> (&mut self, condition: Vec<T>) -> &mut Self {

        self
    }
}






#[cfg(test)]
mod tests {
    #[test]
    fn select_min() {
        assert!(2>3, "Test Missing");
    }
    #[test]
    fn select_max() {
        assert!(2>3, "Test Missing");
    }
    #[test]
    fn select_rand() {
        assert!(2>3, "Test Missing");
    }
    #[test]
    fn select_list() {
        assert!(2>3, "Test Missing");
    }
    #[test]
    fn select_all() {
        assert!(2>3, "Test Missing");
    }
    #[test]
    fn select_regex() {
        assert!(2>3, "Test Missing");
    }

}
