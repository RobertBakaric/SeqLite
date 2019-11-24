use clap::*;
use std::str;

pub(crate)fn parse_cli ()->  clap::ArgMatches<'static> {

    let head : &str ="                               \t
    _____                                            \t
   / ____|              /\\                          \t
  | (___   ___  __ _   /  \\   _ __  _ __            \t
   \\___ \\ / _ \\/ _` | / /\\ \\ | '_ \\| '_ \\     \t
   ____) |  __/ (_| |/ ____ \\| |_) | |_) |          \t
  |_____/ \\___|\\__, /_/    \\_\\ .__/| .__/        \t
                  | |        | |   | |               \t
                  |_|        |_|   |_|               \t
                             Auth: Robert Bakaric    ";

    let  matches = App::new("SeqApp")
          .version("0.01")
          .author("Robert Bakaric <robertbakaric@zoho.com>")
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
               .value_name("select|import|export|update|delete")
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
