use crate::*;

fn data_dir() -> String {
    format!(
        "{}/crablit/",
        dirs::data_dir()
            .expect("Couldn't find data_dir")
            .to_str()
            .unwrap(),
    )
}

/// Returns the existence of path got in state dir
pub fn progress_exists(path: &str) -> bool {
    //     match std::fs::create_dir(self::data_dir() + path) {
    //         Err(err) => {
    //             if err.kind() == std::io::ErrorKind::NotFound {
    //                 false
    //             } else if err.kind() == std::io::ErrorKind::AlreadyExists {
    //                 true
    //             } else {
    //                 todo!("Couldn't determine progress state")
    //             }
    //         }
    //         _ => todo!("Couldn't determine progress state"),
    //     }
    fs::read_to_string(self::data_dir() + path).is_ok()
}

/// Path for statefile of filepath got
pub fn get_state_path(path: &str) -> Result<String, Box<dyn Error>> {
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

    Ok(format!("{}{}", self::data_dir(), current_file_path))
}
