


    #[test]
    fn adding_fastq_records() {
        use crate::IO;
        use crate::Queries;
        use std::str::from_utf8;

        let record ="@SRR8374 vcndjvn/1\n\
                    ATGTCGTGCAGACGTGCCCCCCCCT\n\
                    +\n\
                    #$%#$%#$%%%$$&$$%&&/&/%%$\n\
                    @SRR8374654 vcn/2\n\
                    ATGTCGTGCAGACAAAGTGCCCCCCCCT\n\
                    +\n\
                    #$%#$%#$%%&/////&$$%&&/&/%%$\n\
                    @SRR8374654 vcn/2\n\
                    ATGTCGTGCAGACAAAGTGCCCCCCCCT\n\
                    +\n\
                    #$%#$%#$%%&/////&$$%&&/&/%%$\n".to_string();

        let mut sdb = crate::SeqLiteDb::new("fastq");
        sdb.import(&record).select("all".to_string());

        assert_eq!(sdb.export_head().unwrap()[0],sdb.head[0]);
        assert_eq!(sdb.export_seq().unwrap()[0],from_utf8(&sdb.seq[0..25]).unwrap());
        assert_eq!(sdb.export_qual().unwrap()[1],from_utf8(&sdb.qual[25..53]).unwrap());
        assert_eq!(sdb.export_rid().unwrap()[2], "SRR8374654".to_string());

    }

    #[test]
    fn dumping_raw_db_records(){
        use crate::IO;
        use crate::Queries;

        let record ="@SRR8374 vcndjvn/1\n\
                    ATGTCGTGCAGACGTGCCCCCCCCT\n\
                    +\n\
                    #$%#$%#$%%%$$&$$%&&/&/%%$\n\
                    @SRR8374654 vcn/2\n\
                    ATGTCGTGCAGACAAAGTGCCCCCCCCT\n\
                    +\n\
                    #$%#$%#$%%&/////&$$%&&/&/%%$\n".to_string();

        let mut sdb = crate::SeqLiteDb::new("fastq");
        sdb.import(&record).select("all".to_string());

        assert_eq!(sdb.dump_seq().unwrap(), b"ATGTCGTGCAGACGTGCCCCCCCCTATGTCGTGCAGACAAAGTGCCCCCCCCT".to_vec());
        assert_eq!(sdb.dump_qual().unwrap(), b"#$%#$%#$%%%$$&$$%&&/&/%%$#$%#$%#$%%&/////&$$%&&/&/%%$".to_vec());
    }

    #[test]
    fn uploading_fastq_records() {
        use crate::IO;

        let sdb = crate::SeqLiteDb::new("fastq");
        sdb.upload("./data/illumina.fq");

    }

    #[test]
    fn downloading_fastq_records() {
        use crate::IO;
        use crate::Queries;

        let mut sdb = crate::SeqLiteDb::new("fastq").upload("./data/illumina.fq");
        sdb.select("rand(2)".to_string());
        sdb.download("./data/fq.tmp").unwrap();

    }


    #[test]
    fn adding_fasta_records() {
        use crate::IO;
        use crate::Queries;
        use std::str::from_utf8;

        let record =">FA8374 vcndjvn\n\
                    ATGTCGTGCAGACGTGCCCCCCCCT\n\
                    ATGTCGTGCAGACAA\n\
                    >FA8374654 vcn get\n\
                    ATGTCGTGCAGACAAAGTGCCCCCCCCT\n".to_string();

        let mut sdb = crate::SeqLiteDb::new("fasta");
        sdb.import(&record).select("all".to_string());

        assert_eq!(sdb.export_head().unwrap()[0],sdb.head[0]);
        assert_eq!(sdb.export_seq().unwrap()[0],from_utf8(&sdb.seq[0..40]).unwrap());
        assert_eq!(sdb.export_rid().unwrap()[1], "FA8374654".to_string());

    }

    #[test]
    fn uploading_fasta_records() {
        use crate::IO;

        let sdb = crate::SeqLiteDb::new("fasta");
        sdb.upload("./data/rand.fa");

    }

    #[test]
    fn downloading_fasta_records() {
        use crate::IO;
        use crate::Queries;

        let mut sdb = crate::SeqLiteDb::new("fasta").upload("./data/rand.fa");
        sdb.select("rand(1)".to_string());
        sdb.download("./data/fa.tmp").unwrap();

    }
