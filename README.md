# SeqLite
Ultra-fast, simple fast(q/a) key-value stores (with raw data storage support)

## Intro
SeqLite is yet another ultra-fast  fast(a/q)  key-value stores with a support for raw data storage. It  implements a simple model that pairs a unique key with an associated value. Because of its simplicity it is to be considered as extremely performant and highly scalable for management and caching of large datasets. Moreover, the database utilizes a novel data structure with a support for random access queries on both raw and compressed datasets. It is oriented to work with RAM and solid-state drives.

## Copyright

Robert Bakaric <robertbakaric@zoho.com>


## Project Status

- [x] Implement fasta read/write
- [x] Implement fastq read/write
- [ ] Implement raw read/write
- [ ] Implement select()  
- [ ] Implement get()
- [ ] Implement delete()
- [ ] Implement update()
- [x] Implement rand(X)
- [ ] Implement max(len)
- [ ] Implement min(len)
- [ ] Implement list(1,2,3)
- [ ] Implement regex((.*?))
- [ ] Implement tests
- [x] Implement io handlers
- [ ] Implement error handlers
