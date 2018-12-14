use structopt::StructOpt;

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
pub struct Opt {

  
   /// A flag, true if used in the command line.
    #[structopt(short = "d", long = "debug", help = "Activate debug mode")]
    debug: bool,

    /// An argument of type float, with a default value.
    #[structopt(short = "s", long = "speed", help = "Set speed", default_value = "42")]
    speed: f64,

    /// Needed parameter, the first on the command line.
    #[structopt(help = "Input file")]
    input: String,

    /// An optional parameter, will be `None` if not present on the
    /// command line.
    #[structopt(help = "Offbin config file")]
    config: Option<String>,

  /// An argument of type float, with a default value.
    #[structopt(short = "p", long = "path", help = "set folder", default_value = "/")]
    path: String,

}
