use crate::utils::error::{Error};

use crate::SeqLiteDb;

impl SeqLiteDb{
    // To be implemented:
    //     - Packing of characters outside {A,T,C,G} should be done by combining RLE and DeltaCoding
    pub(crate) fn bit_pack_dna(&mut self, mut test: String) -> Result<Vec<u8>,Error> {


        while (test.len() % 4) != 0 {                  // Pad to multiple of four chars.
            test.push(' ');
        }

        let mut code_vec = Vec::new();
        let mut out_byte: u8 = 0;
        let mut shift = 0;

        for c in test.chars() {                        // Break it into characters.
            let code = match c {                        // Encode characters (00, 01, 11)
                'a' | 'A' => 0b00000000,
                't' | 'T' => 0b00000001,
                'g' | 'G' => 0b00000010,
                'c' | 'C' => 0b00000011,
                    _     => panic!("{} -> not a standard dna alphabet",c)
            };
            out_byte = out_byte | (code << shift);      // Magic happens !
            shift = shift + 2;
            if shift == 8 {
                code_vec.push(out_byte);
                shift = 0;
                out_byte = 0;
            }
        }

        Ok(code_vec)
    }

    // To be implemented:
    //     - Uacking of characters outside {A,T,C,G} should be done by combining RLE and DeltaCoding

    pub(crate) fn bit_unpack_dna(&mut self, bit_vec: Vec<u8>) -> Result<String,Error> {

        let mut uncompressed_string = "".to_string();   // Do it in reverse to get back to 1.
        for byte in bit_vec {
            for shift in 0..=3 {
                let code: u8 = byte >> (shift * 2) & 0b11;
                let char = match code {
                    0b00 => 'A',
                    0b01 => 'T',
                    0b10 => 'G',
                     _   => 'C',
                };
                uncompressed_string.push(char);
            }
        }

        Ok(uncompressed_string)

    }

}




#[cfg(test)]
mod tests {
    #[test]
    fn dna_bit_pack_unpack() {

        use crate::{SeqLiteDb};

        let my_dna = "TGACTGGGGCCATTCC".to_string();    // Take a string.
        let bitvec = SeqLiteDb::new("fasta").bit_pack_dna(my_dna.clone()).unwrap().clone();

        assert_eq!(bitvec.clone(), vec![201u8, 169u8, 62u8, 245u8]);

        let strng = SeqLiteDb::new("fasta").bit_unpack_dna(bitvec).unwrap().clone();

        assert_eq!(strng, my_dna);
    }
}
