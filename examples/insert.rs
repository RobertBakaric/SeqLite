use SeqLite::*;
use clap::*;
use std::str;

fn parse_cli ()->  clap::ArgMatches<'static> {

    let head : &str =
"
-------------------------------------------------
Testing Queries !!
-------------------------------------------------
 ";

    let  matches = App::new("query")
          .version("0.01")
          .author("Robert Bakaric <rbakaric@irb.hr>")
          .about(head)

          .arg(Arg::with_name("input")
               .short("i")
               .long("input")
               .required(false)
               .default_value("stdin")
               .value_name("FILE")
               .help("Input file [txt,fasta,fastq]")
               .takes_value(true))

          .arg(Arg::with_name("output")
               .short("o")
               .long("output")
               .required(false)
               .value_name("FILE")
               .default_value("stdout")
               .help("Output file")
               .takes_value(true))

          .arg(Arg::with_name("fformat")
               .short("f")
               .long("file-format")
               .required(true)
               .value_name("raw|fastq|fasta")
               .help("File format")
               .takes_value(true))

          .arg(Arg::with_name("cmd")
               .short("c")
               .long("cmd")
               .required(true)
               .default_value("select")
               .value_name("select|set|get|update|delete")
               .help("query comand to be utilized!")
               .takes_value(true))

          .arg(Arg::with_name("query")
               .short("q")
               .long("query")
               .required(true)
               .default_value("all")
               .value_name("all|rand(2)|max/min(len/lcp)|list(1,4,3)|where(cond)")
               .help("Condition to be met when executing cmd")
               .takes_value(true))

          .arg(Arg::with_name("llen")
               .short("l")
               .long("line-length")
               .required(true)
               .value_name("int > 0")
               .default_value("60")
               .help("Line length used when writing data")
               .takes_value(true))

          .arg(Arg::with_name("v")
               .short("v")
               .multiple(true)
               .help("Sets the level of verbosity"))

        .get_matches();

    matches
}



fn main(){

    let cli = parse_cli();
    //println!("{:#?}",cli);

    let mut sdb = SeqLiteDb::new(cli.value_of("fformat").unwrap())
        .set_llen(cli.value_of("llen").unwrap().parse::<usize>().unwrap());

    let record =
"@SRR8374 vcndjvn/1\n\
ATGTCGTGCAGACGTGCCCCCCCCC\n\
+\n\
#$%#$%#$%%%$$&$$%&&/&/%%$\n\
";

    let res_h = sdb.select(cli.value_of("query")
                  .unwrap()
                  .to_string()).get_head();

    println!("{:?}", res_h);

    let res_s = sdb.get_seq();

    println!("{:?}", res_s);

    let res_q = sdb.get_qual();

    println!("{:?}", res_q);

    let res_i = sdb.get_rid();

    println!("{:?}", res_i);

}
