
    use std::env;
    use std::fs::File;
    use std::io::prelude::*;
    use std::io;

    pub struct Config {
        query: String,
        filename: String,
        case_sensitive: bool,
    }

    impl Config {
        pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
            args.next();
            
            let query = match args.next() {
                Some(a) => a,
                None => return Err("Didn't get a query string"),
            };

            let filename = match args.next() {
                Some(a) => a,
                None => return Err("Didn't get a file name"),
            };

            let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

            Ok(Config { query, filename, case_sensitive})
        }
    }

    pub fn run(cfg:Config) -> Result<(), io::Error>{
        println!("Searching {} in file: {}", cfg.query, cfg.filename);

        let mut f = File::open(cfg.filename)?;

        let mut s = String::new();
        f.read_to_string(&mut s)?;

        for line in search(&cfg.query, &s){
            print!("{}\n",line);
        }
        Ok(())
    }

    pub fn search<'a>(quary:&str, content:&'a str) -> Vec<&'a str>{
        content.lines()
            .filter(|line| { line.contains(quary) as bool})
            .collect()
    }

//     pub fn grep() {
//     let config = Config::new(env::args()).unwrap_or_else(|err| {
//         println!("Problem parsing arguments: {}", err);
//         process::exit(1);
//     });

//     let (tx,rx) = mpsc::channel();

//     thread::spawn( move || { 
//         if let Err(e) = run(config){
//             println!("Application error: {}", e);
//             process::exit(1);
//         }
//         tx.send(String::from("yes!\n")).unwrap();
//     });

//         for recv in rx{
//         println!("revice: {}", recv);
//     }
// }
