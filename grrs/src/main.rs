use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]

struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    // println!("Hello, world!");
    
    // let progfile = std::env::args().nth(0).expect("no progfile");

    // let pattern = std::env::args().nth(1).expect("no pattern given");
    // let path = std::env::args().nth(2).expect("no path given");


    // let args = Cli {
    //     pattern,
    //     path: std::path::PathBuf::from(path),
    // };
    //println!("progfile: {:?}, pattern: {:?}, path: {:?}", progfile, args.pattern, args.path);


    let args = Cli::parse();
    //println!("pattern: {:?}, path: {:?}", args.pattern, args.path)
    let content = std::fs::read_to_string(&args.path).expect("could not read file");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }


    
}
