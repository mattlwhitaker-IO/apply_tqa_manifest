use std::env;
use std::error::Error;
use std::fs;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;

pub const MANIFEST_FILE_NAME: &str = "manifest";
pub const MANIFEST_EXT: &str = "txt";
pub const MANIFEST_DELIM: &str = "=";

#[derive(Debug)]
struct Manifest {
    owl_filename: String,
    orig_filename: String,
}

impl Manifest {
    fn rename_owl_to_orig(&self, wd: &PathBuf) {
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

    pub fn rename_orig_to_owl(&self, wd: &PathBuf) {
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
pub struct ProcessConfig {
    pub process_path: PathBuf,
    pub reverse: bool,
    pub delete_manifest: bool,
}

impl ProcessConfig {
    pub fn new() -> ProcessConfig {
        ProcessConfig {
            process_path: PathBuf::new(),
            reverse: false,
            delete_manifest: false,
        }
    }

    pub fn parse_args(in_args: &[String]) -> Result<ProcessConfig, Box<dyn Error>> {
        let mut op = ProcessConfig::new();

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

        op.process_path = get_path(in_args)?;
        Ok(op)
    }

    pub fn run(&self) -> Result<(), String> {
        //check that the path exists
        if !self.process_path.exists() {
            let no_path: String =
                format!("The path does not exist: {}", self.process_path.display());
            return Err(no_path);
        }

        //check that the manifest file exists in the path
        if !path_contains_manifest(&self.process_path) {
            let no_manifest = format!(
                "{} does not contain a file named {}.{}",
                self.process_path.display(),
                MANIFEST_FILE_NAME,
                MANIFEST_EXT
            );
            return Err(no_manifest);
        }

        //read in the manifest file
        let manifest_text = read_manifest_to_vector(&self.process_path);
        let manifest_text = match manifest_text {
            Ok(m_text) => m_text,
            Err(_error) => {
                return Err(String::from("Error reading file"));
            }
        };

        //process each line of the manifest file
        for line in manifest_text {
            let m = split_manifest_string(&line);
            let m = match m {
                Ok(manifest) => manifest,
                Err(_error) => {
                    return Err(String::from("Error Splitting Line"));
                }
            };

            if self.reverse {
                m.rename_orig_to_owl(&self.process_path);
            } else {
                m.rename_owl_to_orig(&self.process_path);
            }
        }

        // delete the manifest if required
        if self.delete_manifest {
            if let Err(_e) = delete_manifest_file(&self.process_path) {
                return Err(String::from("Error deleting manifest file."));
            }
        }

        Ok(())
    }
}

fn get_path(in_args: &[String]) -> Result<PathBuf, Box<dyn Error>> {
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
