use std::fs;
use std::env;
use std::ffi::OsString;
use std::path::PathBuf;
use std::io::ErrorKind;

#[derive(PartialEq, Debug)]
pub enum DBError {
    InitialisationError(&'static str),
}

pub fn init(path: &str) -> Result<(), DBError> {
    /* Getting the path to the home directory */
    let home_path: OsString;

    match env::var_os("HOME") {
        Some(val) => {
            home_path = val;
        },
        _ => return Err(DBError::InitialisationError("Could not find home directory")),
    }

    /* Creating the data directory */
    let mut absolute_path = PathBuf::from(home_path);
    absolute_path.push(path);

    let creation_result = fs::create_dir(absolute_path);
    
    match creation_result {
        Ok(_) => {},
        Err(e) => {
            match e.kind() {
                ErrorKind::AlreadyExists => {}, // We don't mind if the directory already exists
                _ => return Err(DBError::InitialisationError("Error creating the data directory")),
            }
        },
    };

    Ok(())
}
