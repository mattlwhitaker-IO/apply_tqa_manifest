use std::env;
use std::error::Error;
use std::fs;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;

use apply_tqa_manifest::Manifest;
use apply_tqa_manifest::ProcessOptions;

const MANIFEST_FILE_NAME: &str = "manifest";
const MANIFEST_EXT: &str = "txt";
const MANIFEST_DELIM: &str = "=";

fn main() {
    let args: Vec<String> = env::args().collect();

    let wd = apply_tqa_manifest::get_path(&args).expect("Problem determining working directory");

    let op = parse_args(&args);

    println!("{:?}", op);

    println!("The directory being modified is {}", wd.display());

    if !wd.exists() {
        println!("{} does not exist.", wd.display());
        return;
    }

    if !path_contains_manifest(&wd) {
        println!(
            "{} does not contain a file named {}.{}",
            wd.display(),
            MANIFEST_FILE_NAME,
            MANIFEST_EXT
        );
        return;
    }

    let manifest_text = read_manifest_to_vector(&wd).expect("Error reading file");
    for line in manifest_text {
        let m = split_manifest_string(&line).expect("Error splitting line");
        if op.reverse {
            m.rename_orig_to_owl(&wd);
        } else {
            m.rename_owl_to_orig(&wd);
        }
    }

    if op.delete_manifest {
        delete_manifest_file(&wd).expect("Error deleting manifest file");
    }
}

fn parse_args(in_args: &[String]) -> ProcessOptions {
    let mut op = ProcessOptions::new();

    for s in in_args {
        match s.as_str() {
            "-r" => {
                op.reverse = true;
            }
            "-m" => {
                op.delete_manifest = true;
            }
            _ => (),
        }
    }

    op
}

fn path_contains_manifest(wd: &PathBuf) -> bool {
    let manifest_path = get_manifest_path(wd);

    manifest_path.exists()
}

fn get_manifest_path(wd: &PathBuf) -> PathBuf {
    let mut manifest_path = PathBuf::from(wd);
    manifest_path.push(MANIFEST_FILE_NAME);
    manifest_path.set_extension(MANIFEST_EXT);
    manifest_path
}

fn read_manifest_to_vector(wd: &PathBuf) -> io::Result<Vec<String>> {
    let manifest_path = get_manifest_path(wd);
    let file_in = fs::File::open(manifest_path)?;
    let file_reader = BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}

fn split_manifest_string(manifest_str: &str) -> Result<Manifest, Box<dyn Error>> {
    let split = manifest_str.split(MANIFEST_DELIM);
    let split_vec: Vec<&str> = split.collect();

    let manifest = if split_vec.len() > 1 {
        Manifest {
            owl_filename: split_vec[0].to_string(),
            orig_filename: split_vec[1].to_string(),
        }
    } else {
        Manifest {
            owl_filename: manifest_str.to_string(),
            orig_filename: String::from(""),
        }
    };
    Ok(manifest)
}

fn delete_manifest_file(wd: &PathBuf) -> Result<(), Box<dyn Error>> {
    fs::remove_file(get_manifest_path(wd))?;
    Ok(())
}
