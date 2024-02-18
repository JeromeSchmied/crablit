use crate::*;
use std::path::{Path, PathBuf};

// #[cfg(windows)]
// fn get_sep() -> char {
//     '\\'
// }
// #[cfg(not(windows))]
// fn get_sep() -> char {
//     '/'
// }

/// Returns data_dir of current file using `dirs` crate
fn data_dir() -> PathBuf {
    dirs::data_dir()
        .expect("couldn't find data dir")
        .join("crablit")
}

/// Returns the progress path, if doesn't exist, creates it's path, but not the file itself
pub fn get_prog_path(path: &Path) -> Result<PathBuf, Box<std::io::Error>> {
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

/// Returns the existence of path got in state dir
pub fn prog_exists(path: &Path) -> bool {
    let path = get_prog_path(path).unwrap();
    fs::read_to_string(path).is_ok()
}

/// Delete progress if exists
pub fn rm_prog(path: &Path) -> Result<(), Box<dyn Error>> {
    if prog_exists(path) {
        eprintln!("Removing state file from: {:?}", get_prog_path(path)?);
        fs::remove_file(get_prog_path(path)?)?;
    }
    Ok(())
}

/// Make item writable to file
///
/// # usage
/// ```
/// use crablit::Verb;
/// use crablit::state::serialize;
///
/// let deck = vec![
///     Verb::new("inf1", "dri1", "pra1", "per1", "trm1", None),
///     Verb::new("inf2", "dri2", "pra2", "per2", "trm2", None),
///     Verb::new("inf3", "dri3", "pra3", "per3", "trm3", None),
///     Verb::new("inf4", "dri4", "pra4", "per4", "trm4", None),
/// ];
///
/// let r = "\
/// inf1;dri1;pra1;per1;trm1;Nothing
/// inf2;dri2;pra2;per2;trm2;Nothing
/// inf3;dri3;pra3;per3;trm3;Nothing
/// inf4;dri4;pra4;per4;trm4;Nothing\n";
///
/// assert_eq!(r, serialize(&deck, ';'));
/// ```
pub fn serialize<T: Learn>(v: &[T], delim: char) -> String {
    v.iter().fold(String::new(), |r, item| {
        r + &item.ser(&delim.to_string()) + "\n"
    })
}

/// Save progress to `data_dir`/crablit/`current_file`
pub fn save_prog<T>(deck: &[T], conf: &config::Config) -> Result<(), Box<dyn Error>>
where
    T: Learn + std::fmt::Debug,
{
    let ofile_path = state::get_prog_path(&conf.file_path_orig())?;
    let mut ofile = File::create(&ofile_path)?;

    writeln!(ofile, "# [crablit]")?;
    writeln!(ofile, "# mode = \"{}\"", conf.mode().disp())?;
    writeln!(ofile, "# delim = \'{}\'\n\n", conf.delim())?;

    println!("r: {:?}", deck);
    let content = serialize(deck, conf.delim());
    writeln!(ofile, "{}", content)?;

    eprintln!("Saved file to {}{:?}.\n\n", SPACER.repeat(2), ofile_path);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn serialize_cards() {
        let deck = vec![
            Card::new("term1", "def1", None),
            Card::new("term2", "def2", None),
            Card::new("term3", "def3", None),
            Card::new("term4", "def4", None),
            Card::new("term5", "def5", None),
            Card::new("term6", "def6", None),
            Card::new("term7", "def7", None),
        ];
        let r = "\
term1;def1;Nothing
term2;def2;Nothing
term3;def3;Nothing
term4;def4;Nothing
term5;def5;Nothing
term6;def6;Nothing
term7;def7;Nothing\n";
        assert_eq!(r, serialize(&deck, ';'));
    }
    #[test]
    fn serialize_verbs() {
        let deck = vec![
            Verb::new("inf1", "dri1", "pra1", "per1", "trm1", None),
            Verb::new("inf2", "dri2", "pra2", "per2", "trm2", None),
            Verb::new("inf3", "dri3", "pra3", "per3", "trm3", None),
            Verb::new("inf4", "dri4", "pra4", "per4", "trm4", None),
            Verb::new("inf5", "dri5", "pra5", "per5", "trm5", None),
            Verb::new("inf6", "dri6", "pra6", "per6", "trm6", None),
            Verb::new("inf7", "dri7", "pra7", "per7", "trm7", None),
        ];
        let r = "\
inf1;dri1;pra1;per1;trm1;Nothing
inf2;dri2;pra2;per2;trm2;Nothing
inf3;dri3;pra3;per3;trm3;Nothing
inf4;dri4;pra4;per4;trm4;Nothing
inf5;dri5;pra5;per5;trm5;Nothing
inf6;dri6;pra6;per6;trm6;Nothing
inf7;dri7;pra7;per7;trm7;Nothing\n";
        assert_eq!(r, serialize(&deck, ';'));
    }

    #[test]
    fn get_data_dir() {
        data_dir();
    }

    // get_prog_path
    #[test]
    fn get_progress_path() {
        assert!(get_prog_path(Path::new("test_prog_path.txt")).is_ok());
    }

    // // save_prog
    // #[test]
    // fn save_progress_cards() {
    //     let deck = vec![
    //         Card::new("term1", "def1"),
    //         Card::new("term2", "def2"),
    //         Card::new("term3", "def3"),
    //         Card::new("term4", "def4"),
    //         Card::new("term5", "def5"),
    //         Card::new("term6", "def6"),
    //         Card::new("term7", "def7"),
    //     ];
    //     // save_prog(&deck, ';');
    // }

    // prog_exists
    #[test]
    fn progress_exists() {
        let orig_path = Path::new("test_prog_exists.txt");
        let state_path = state::get_prog_path(orig_path).unwrap();

        let mut ofile = File::create(state_path).unwrap();
        writeln!(ofile, "is there anybody in there?").unwrap();

        assert!(prog_exists(orig_path));
        assert_eq!(Some(()), rm_prog(orig_path).ok());
    }

    // rm_prog
    #[test]
    fn remove_progress() {
        let orig_path = Path::new("test_rm_prog.txt");
        let state_path = state::get_prog_path(orig_path).unwrap();

        let mut ofile = File::create(state_path).unwrap();
        writeln!(ofile, "is there anybody in there?").unwrap();

        assert!(prog_exists(orig_path));
        assert_eq!(Some(()), rm_prog(orig_path).ok());
    }
}
