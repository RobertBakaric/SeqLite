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

    pub(crate) fn seq_select_min (&mut self, len: usize) -> &mut Self {
        self
    }

    pub(crate) fn seq_select_max (&mut self, len: usize ) -> &mut Self {
        self
    }

    pub(crate) fn seq_select_list<T> (&mut self, condition: Vec<T>) -> &mut Self {
        self
    }


}
