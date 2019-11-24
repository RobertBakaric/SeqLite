# SeqLite
Ultra-fast, simple fast(q/a) key-value stores (with raw data storage support)

## Intro
SeqLite is yet another ultra-fast  fast(a/q)  key-value stores with a support for raw data storage. It  implements a simple model that pairs a unique key with an associated value. Because of its simplicity it is to be considered as extremely performant and highly scalable for management and caching of large datasets. Moreover, the database utilizes a novel data structure with a support for random access queries on both raw and compressed datasets. It is oriented to work with RAM and solid-state drives.

## Copyright

Robert Bakaric <robertbakaric@zoho.com>


## Project Status

- [x] Implement fasta read/write
- [x] Implement fastq read/write
- [x] Implement raw read/write
- [x] Implement select()
- [x] Implement db dumpers for "seq", "qual" and "all"
- [x] Implement export()                       <- 24.11
- [x] Implement import()
- [ ] Implement delete()                       <- 24.11
- [ ] Implement update()
- [x] Implement rand(X)
- [ ] Implement max/min(len|lcp)   
- [ ] Implement list(1,2,3)                    <- 24.11
- [ ] Implement regex((.*?))                   <- continuous
- [x] Implement tests 6/72
- [x] Implement io handlers
- [ ] Implement error handlers
- [x] Implement (un)pack for dna
- [ ] Implement (un)pack for quality scores
- [ ] Implement (un)pack for fa
- [ ] Implement (un)pack for txt
- [x] Write CLI app for SeqLite (SeqApp)       <- 24.11
- [ ] Write Server for SeqLite (SeqServer)
- [ ] Write Client for SeqLite (SeqClient)

to do for tommorow:

- implement get for full records
- implement delete
- implement update
