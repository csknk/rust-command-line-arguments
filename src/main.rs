use std::env;
use std::fs;
use std::process;

/// Unwrap the return value from `Config::new()` using `unwrap_or_else()`.
/// If the incorrect number of arguments are provided, the programme returns an exit value of 1,
/// achieved using `process::exit(1)`.
fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args)
        .unwrap_or_else(|err| {
            println!("Problem parsing arguments: {}", err);
            process::exit(1);
        });
    run(config);
    Ok(())
}

struct Config {
    query: String,
    filename: String,
}


/// Parse config values into a Config object.
///
/// It's more idiomatic to place the parse config functionality
/// into a member function of Config rather than as a standalone
/// function that returns a Config object.
impl Config {
    /// Note that each command line argument received in `main` have the `String`
    /// data type. These are grouped (in main) into a `Vec<String>` collection.
    /// This function must receive a slice of that Vec, `&[String]`.
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Please supply 2 arguments.")
        }
        // The `args` variable in `main()` is the owner of the argument variables,
        // and only lets this function borrow them. Rust won't let ownership of these
        // variables be transferred to the Config object.
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

fn run(config: Config) {
    println!("The query is {}", config.query);
    let contents = fs::read_to_string(config.filename)
        .expect("Didn't open file properly...");
    println!("Text:\n{}", contents);
}

//fn read_file() {}
