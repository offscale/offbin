extern crate clap;
use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("offbin")
        .version("1.0")
        .author("Rishab <rishflab@hotmail.com>")
        .about("Packages tasks into a standalone binary")
        .arg(Arg::with_name("package")
            .short("p")
            .long("package")
            .value_name("FILE")
            .help("Sets a custom config file")
            .takes_value(true))
        .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let config = matches.value_of("config").unwrap_or("assets/offbin.toml");
    println!("Value for config: {}", config);

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    match matches.occurrences_of("v") {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("Don't be crazy"),
    }
}