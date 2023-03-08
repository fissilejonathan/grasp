use clap::{App, Arg};
use regex::Regex;
use std::io::{self, BufRead};

fn main() {
    println!(
        r#"
          __   _ __   _ _   ___  _ _   
        /'_ `\( '__)/'_` )/',__)( '_`\ 
       ( (_) || |  ( (_| |\__, \| (_) )
       `\__  |(_)  `\__,_)(____/| ,__/'
       ( )_) |                  | |    
        \___/'                  (_)  
        "#
    );

    let matches = App::new("grasp")
        .arg(
            Arg::with_name("input")
                .long("input")
                .takes_value(true)
                .help("Specifies input"),
        )
        .get_matches();

    if let Some(input) = matches.value_of("input") {
        println!("User specified input: {}", input);
    } else {
        println!("Please enter input:");

        let stdin = io::stdin();

        let mut input = stdin
            .lock()
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .replace(" ", "");

        while !Regex::new(r#"^[^\n,]+(?:,[^\n,]+)*$"#)
            .unwrap()
            .is_match(&input)
        {
            println!("Invalid input format. Please enter input in the format 'input1' or 'input1, input2, input3'.");
            input = stdin.lock().lines().next().unwrap().unwrap();
        }
    }
}
