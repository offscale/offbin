// use structopt::StructOpt;
// use clap::App;
// use crate::config::Config;

//// A basic example
//#[derive(StructOpt, Debug)]
//#[structopt(name = "basic")]
//pub struct Opt {
//    /// A flag, true if used in the command line.
//    #[structopt(short = "d", long = "debug", help = "Activate debug mode")]
//    debug: bool,
//
//    /// An argument of type float, with a default value.
//    #[structopt(short = "s", long = "speed", help = "Set speed", default_value = "42")]
//    speed: f64,
//
//    /// Needed parameter, the first on the command line.
//    #[structopt(help = "Input file")]
//    input: String,
//
//    /// An optional parameter, will be `None` if not present on the
//    /// command line.
//    #[structopt(help = "Offbin config file")]
//    config: Option<String>,
//
//    /// An argument of type float, with a default value.
//    #[structopt(short = "p", long = "path", help = "set folder", default_value = "/")]
//    path: String,
//}

//fn create_custom_cli_from_config<'a, 'b>(config: &Config) -> App {
//    let mut app = App::new(config.name)
//        .version("1.0")
//        .about(config.name.description);
//
//    for task in config.tasks.unwrap() {
//        app.arg(task.name)
//    }
//
//    app
//}

//fn create_offbin_cli() -> App {
//    let mut app = App::new("offbin")
//        .version("1.0")
//        .about("Binary packager");
//}



//
//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn test_execute_task() {
//
//        let config = Task {
//            name: "hello_from_rake".to_string(),
//            command: "rake".to_string(),
//            args: vec!["--rakefile".to_string(), "/Users/rishflab/offbin/assets/Rakefile".to_string()],
//        };
//
//        let output = task.execute();
//
//        println!("{:?}", output);
//
//        assert_eq!(2, 2);
//
//    }
//}
