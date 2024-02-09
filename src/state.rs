use crate::*;
use std::path::{Path, PathBuf};

/// Delete progress if exists
pub fn rm(path: &Path) -> Result<(), Box<dyn Error>> {
    if progress_exists(path) {
        eprintln!("Removing state file from: {:?}", get_progress_path(path)?);
        fs::remove_file(get_progress_path(path)?)?;
    }
    Ok(())
}

// #[cfg(windows)]
// fn get_sep() -> char {
//     '\\'
// }
// #[cfg(not(windows))]
// fn get_sep() -> char {
//     '/'
// }

fn data_dir() -> PathBuf {
    dirs::data_dir()
        .expect("couldn't find data dir")
        .join("crablit")
}

/// Returns the existence of path got in state dir
pub fn progress_exists(path: &Path) -> bool {
    let path = get_progress_path(path).unwrap();
    fs::read_to_string(path).is_ok()
}

/// Returns the progress path, if doesn't exist, creates it's path, but not the file itself
pub fn get_progress_path(path: &Path) -> Result<PathBuf, Box<std::io::Error>> {
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
    let current_file_path = &format!("{}/{}", pwd, path.display())
        .replace('/', "%")
        .replace('\\', "%");

    Ok(self::data_dir().join(current_file_path))
}
