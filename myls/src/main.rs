use std::env;
use std::path::PathBuf;
use myls::Ls;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        PathBuf::from(".")
    };

    let ls = Ls::new(path);

    if ls.path_exists() {
        match ls.read_and_print_directories() {
            Ok(entries) => println!("{}", entries),
            Err(e) => {
                println!("Error occurred: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        println!("Error: Path '{}' not found", ls.get_path().display());
        std::process::exit(1);
    }
}