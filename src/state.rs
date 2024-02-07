use crate::*;
use std::path::PathBuf;

// fn is_state(path: &str) -> bool {
//     let orig = path.split('%').last().unwrap_or(path);
//     progress_exists(path) && get_path(orig).unwrap_or((&orig).to_string()) == path
// }

/// Delete progress if exists
pub fn rm(path: &str) -> Result<(), Box<dyn Error>> {
    if progress_exists(path) {
        eprintln!("Removing state file from: {:?}", get_progress_path(path)?);
        fs::remove_file(get_progress_path(path)?)?;
    }
    Ok(())
}

fn data_dir() -> PathBuf {
    PathBuf::from(format!(
        "{}/crablit/",
        dirs::data_dir()
            .expect("Couldn't find data_dir")
            .to_str()
            .unwrap(),
    ))
}

/// Returns the existence of path got in state dir
pub fn progress_exists(path: &str) -> bool {
    let path = get_progress_path(path).unwrap();
    // let path = path.split('%').last().unwrap();
    fs::read_to_string(path).is_ok()
}

/// Returns the progress path, if doesn't exist, creates it's path, but not the file itself
pub fn get_progress_path(path: &str) -> Result<PathBuf, Box<std::io::Error>> {
    let pwd = std::env::current_dir()?;
    let pwd = pwd.to_str().expect("Couldn't get working dir.");

    // try to create data_dir, if exists, don't do anything
    if let Err(err) = std::fs::create_dir(self::data_dir()) {
        if err.kind() == std::io::ErrorKind::NotFound {
            std::fs::create_dir_all(self::data_dir())?;
        } else if err.kind() == std::io::ErrorKind::AlreadyExists {
        } else {
            return Err(Box::new(err));
        }
    }
    let current_file_path = &format!("{}/{}", pwd, path).replace('/', "%");

    Ok(PathBuf::from(format!(
        "{}{}",
        self::data_dir().to_str().unwrap(),
        current_file_path
    )))
}

// pub fn get_path(path: &str) -> Result<String, Box<dyn Error>> {
//     let pwd = std::env::current_dir()?;
//     let pwd = pwd.to_str().expect("Couldn't get working dir.");

//     // try to create data_dir, if exists, don't do anything
//     if let Err(err) = std::fs::create_dir(self::data_dir()) {
//         if err.kind() == std::io::ErrorKind::NotFound {
//             std::fs::create_dir_all(self::data_dir())?;
//         } else if err.kind() == std::io::ErrorKind::AlreadyExists {
//         } else {
//             return Err(Box::new(err));
//         }
//     }
//     if  {

//     }
//     if self::data_dir(){

//     }

//     let current_file_path = &format!("{}/{}", pwd, path).replace('/', "%");

//     Ok(format!("{}{}", self::data_dir(), current_file_path))
// }
