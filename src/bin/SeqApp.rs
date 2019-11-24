mod cli;

use cli::appcli::parse_cli;
use seqlite::{SeqLiteDb, Queries, IO};


fn main(){

    let cli = parse_cli();

    let mut sdb = SeqLiteDb::new(cli.value_of("fformat").unwrap())
        .set_llen(cli.value_of("llen").unwrap().parse::<usize>().unwrap())
        .upload(cli.value_of("input").unwrap());

        match cli.value_of("cmd") {
            Some(e) => {
                match e {
                    "select" => {
                        sdb.select(cli.value_of("query")
                                      .unwrap()
                                      .to_string()
                                  )
                            .download(cli.value_of("output")
                                         .unwrap()
                                      )
                            .unwrap();
                    },
                    "delete" => {
                        /*
                        sdb.delete(cli.value_of("query")
                                      .unwrap()
                                      .to_string()
                                  )
                            .download(cli.value_of("output")
                                         .unwrap()
                                      )
                            .unwrap(); */
                        panic!("Not implemented yet !");
                    },
                    _        => {
                        panic!("Hey, set --query first !");
                    }
                }
            },
            None   => {
                panic!("Hey set --cmd first !");
            }
        }




}
