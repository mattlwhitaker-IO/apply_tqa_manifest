use apply_tqa_manifest::ProcessConfig;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = ProcessConfig::parse_args(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("{:?}", config);

    if let Err(e) = config.run() {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
