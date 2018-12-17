extern crate offbin;

use structopt::StructOpt;

use offbin::cli::Opt;

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
