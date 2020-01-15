use std::fs;
use std::env;
use std::error::Error;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;

const MANIFEST_FILE_NAME: &str = "manifest";
const MANIFEST_EXT: &str = "txt";
const MANIFEST_DELIM: &str = "=";

#[derive(Debug)]
pub struct Manifest {
    pub owl_filename: String,
    pub orig_filename: String,
}

impl Manifest {
    pub fn rename_owl_to_orig(&self, wd: &PathBuf) {
        let mut owl = PathBuf::from(wd);
        owl.push(self.owl_filename.as_str());

        let mut orig = PathBuf::from(wd);
        orig.push(self.orig_filename.as_str());

        let rn = fs::rename(&owl, &orig);
        match rn {
            Ok(()) => println!("{} renamed to {}", self.owl_filename, self.orig_filename),
            Err(error) => println!("Error renaming {}: {}", self.owl_filename, error),
        };
    }

    pub fn rename_orig_to_owl(self, wd: &PathBuf) {
        let mut owl = PathBuf::from(wd);
        owl.push(self.owl_filename.as_str());

        let mut orig = PathBuf::from(wd);
        orig.push(self.orig_filename.as_str());

        let rn = fs::rename(&orig, &owl);
        match rn {
            Ok(()) => println!("{} renamed to {}", self.orig_filename, self.owl_filename),
            Err(error) => println!("Error renaming {}: {}", self.orig_filename, error),
        };
    }
}

#[derive(Debug)]
pub struct ProcessOptions {
    pub reverse: bool,
    pub delete_manifest: bool,
}

impl ProcessOptions {
    pub fn new() -> ProcessOptions {
        ProcessOptions {
            reverse: false,
            delete_manifest: false,
        }
    }
}

pub fn get_path(in_args: &[String]) -> Result<PathBuf, Box<dyn Error>> {
    let wd = if in_args.len() > 1 {
        match in_args[1].as_str() {
            "-r" => env::current_dir()?,
            "-m" => env::current_dir()?,
            _ => PathBuf::from(&in_args[1]),
        }
    } else {
        env::current_dir()?
    };
    Ok(wd)
}

pub fn path_contains_manifest(wd: &PathBuf) -> bool {
    let manifest_path = get_manifest_path(wd);

    manifest_path.exists()
}

pub fn get_manifest_path(wd: &PathBuf) -> PathBuf {
    let mut manifest_path = PathBuf::from(wd);
    manifest_path.push(MANIFEST_FILE_NAME);
    manifest_path.set_extension(MANIFEST_EXT);
    manifest_path
}

pub fn read_manifest_to_vector(wd: &PathBuf) -> io::Result<Vec<String>> {
    let manifest_path = get_manifest_path(wd);
    let file_in = fs::File::open(manifest_path)?;
    let file_reader = BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}

pub fn split_manifest_string(manifest_str: &str) -> Result<Manifest, Box<dyn Error>> {
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

pub fn delete_manifest_file(wd: &PathBuf) -> Result<(), Box<dyn Error>> {
    fs::remove_file(get_manifest_path(wd))?;
    Ok(())
}

pub fn parse_args(in_args: &[String]) -> ProcessOptions {
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
