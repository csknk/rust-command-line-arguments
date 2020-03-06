use std::env;
use std::fs;

// https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html
fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();
    let config = match Config::new(&args) {
        Ok(config) => config,
        Err(e) => return Err(e),
    };

    println!("Query: {}", config.query);
    println!("Opening {}", config.filename);
    let contents = fs::read_to_string(config.filename)
        .expect("File didn't open properly");
    println!("Text:\n{}", contents);
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

//fn read_file() {}
