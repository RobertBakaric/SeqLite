use SeqLite::*;
use clap::*;


fn parse_cli ()->  clap::ArgMatches<'static> {

    let head : &str =
"
-------------------------------------------------
Testing Upload/Download !!
-------------------------------------------------
 ";

    let  matches = App::new("upload/download")
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
          .arg(Arg::with_name("v")
               .short("v")
               .multiple(true)
               .help("Sets the level of verbosity"))
        .get_matches();

    matches
}



fn main(){

    let cli = parse_cli();
    println!("{:#?}",cli);

    let mut sdb = SeqLiteDb::new(cli.value_of("fformat").unwrap())
        .set_llen(60)
        .upload(cli.value_of("input").unwrap());

    //sdb.set_llen(60);
    // u will only download or get
    sdb.select("all".to_string()).download(cli.value_of("output").unwrap()).unwrap();


    //sdb.select("rand(2)".to_string()).download("stdout").unwrap();

/*
    if let Ok(true) = sdb.download(cli.value_of("output").unwrap()){
        println!("Done !");
    };
*/
    //println!("Format:  {}",sdb.get_fmt());

}
