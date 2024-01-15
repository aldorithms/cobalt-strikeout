use std::fs::File;
use std::io::{BufReader, BufRead, ErrorKind};
use std::str::from_utf8_unchecked;
use regex::Regex;

use std::path::Path;
use std::fs::{self, DirEntry};
use std::io::Write;
use memmap::Mmap;
use std::io::Read;

const MAX_LINE_LENGTH: usize = 1024;

fn file_contains_pii(file_path: &str) -> bool {
    // Ignore binary files by checking for a null byte
    if let Ok(mut file) = File::open(file_path) {
        let mut buf = [0u8; 1];
        if file.read(&mut buf).unwrap_or(0) > 0 && buf[0] == 0 {
            return false;
        }
    }

    // Compile regular expressions
    let ssn_regex = Regex::new(r"[0-9]{3}-[0-9]{2}-[0-9]{4}").unwrap();
    let credit_card_regex = Regex::new(r"[0-9]{4}[- ]?[0-9]{4}[- ]?[0-9]{4}[- ]?[0-9]{4}").unwrap();
    let email_regex = Regex::new(r"[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,4}").unwrap();

    // Check each line of the file for matches to PII regex patterns
    if let Ok(file) = File::open(file_path) {
        let reader = BufReader::new(file);
        let mut found_pii = false;

        for line in reader.lines() {
            if let Ok(line) = line {
                if ssn_regex.is_match(&line) || credit_card_regex.is_match(&line) || email_regex.is_match(&line) {
                    found_pii = true;
                    break;
                }
            } else if let Err(err) = line {
                if err.kind() != ErrorKind::InvalidData {
                    eprintln!("Error reading line: {}", err);
                }
            }
        }
        return found_pii;
    } else {
        eprintln!("Error opening file: {}", file_path);
        return false;
    }
}



const MAX_PATH_LENGTH: usize = 1024;

// Function to search a directory and its subdirectories for files containing PII
pub fn search_directory(directory_path: &str, output_file: &mut std::fs::File) {
    for entry in fs::read_dir(directory_path).unwrap() {
        let entry = entry.unwrap();
        let file_path = entry.path();

        if entry.file_type().unwrap().is_dir() {
            // Recursively search subdirectories
            search_directory(file_path.to_str().unwrap(), output_file);
        } else if entry.file_type().unwrap().is_file() {
            // Check if file contains PII and write path to output file if it does
            if file_contains_pii(file_path.to_str().unwrap()) {
                if let Err(err) = writeln!(output_file, "{}", file_path.display()) {
                    if err.kind() != ErrorKind::BrokenPipe {
                        eprintln!("Error writing to output file: {}", err);
                    }
                }
            }
        }
    }
}

fn file_contains_pattern(file_path: &Path, regex: &Regex) -> bool {
    // Open the file in read-only mode
    let file = match File::open(&file_path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening file: {}", err);
            return false;
        }
    };

    // Memory-map the file
    let mmap = match unsafe { Mmap::map(&file) } {
        Ok(mmap) => mmap,
        Err(err) => {
            eprintln!("Error memory-mapping file: {}", err);
            return false;
        }
    };

    // Check for pattern match in the memory-mapped data
    let found_pattern 
        = regex
            .is_match( 
                unsafe { 
                    from_utf8_unchecked(&mmap) 
                }
            );

    // The memmap object is dropped here, automatically unmapping the file

    found_pattern
}


fn pii_discovery() {
    // Specify directory path and output file path
    let directory_path = "/home/";
    let output_file_path = "output";

    // Create the output file
    let mut output_file = match File::create(output_file_path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error creating output file: {}", err);
            return;
        }
    };

    // Search the directory for files containing PII
    search_directory(directory_path, &mut output_file);

    // The output file is automatically closed here when it goes out of scope
}

use std::collections::HashSet;

fn detect_databases() -> HashSet<String> {
    let database_names = ["mysql", "psql", "redis-server", "mongod", "sqlite3"]; // Add other database names as necessary
    let bin_dir = Path::new("/usr/bin");

    let mut installed_databases = HashSet::new();

    if let Ok(entries) = fs::read_dir(bin_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_name 
                    = entry.file_name();
                for database_name in &database_names {
                    if let Some(file_name_str) = file_name.to_str() {
                        if file_name_str == *database_name {
                            installed_databases
                                .insert(database_name.to_string());
                        }
                    }
                }
            }
        }
    }

    installed_databases
}
