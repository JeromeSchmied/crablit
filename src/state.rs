use crate::*;
use std::path::{Path, PathBuf};

/// Returns `data_dir` of current file using `dirs` crate
fn data_dir() -> PathBuf {
    dirs::data_dir()
        .expect("couldn't find data dir")
        .join("crablit")
}

/// Returns the progress path, if doesn't exist, creates it's path, but not the file itself
///
/// # Errors
///
/// - `pwd()` errors
/// - `create_dir(_all)` errors
///
/// # Panics
///
/// - `pwd()` doesn't contain valid utf8
pub fn prog_path(path: &Path) -> AnyErr<PathBuf> {
    let pwd = std::env::current_dir()?;
    info!("pwd: {pwd:?}");
    let pwd = pwd.to_str().expect("couldn't get working dir");

    // try to create data_dir, if exists, don't do anything
    if let Err(err) = std::fs::create_dir(self::data_dir()) {
        if err.kind() == std::io::ErrorKind::NotFound {
            info!("data_dir: {:?} doesn't exist, creating", self::data_dir());
            std::fs::create_dir_all(self::data_dir())?;
        } else if err.kind() == std::io::ErrorKind::AlreadyExists {
        } else {
            error!("error creating data_dir: {err:?}");
            return Err(Box::new(err));
        }
    }
    let current_file_path = &format!("{}/{}", pwd, path.display())
        .replace('/', "%")
        .replace('\\', "%");
    info!("current file path formatted for being state-file: {current_file_path:?}");

    Ok(self::data_dir().join(current_file_path))
}

/// Returns the existence of path got in state dir
///
/// # Panics
///
/// `get_prog_path()`
pub fn prog_exists(path: &Path) -> bool {
    let path = prog_path(path).unwrap();
    path.exists()
}

/// Get content of file specified in `conf.file_path`,
/// if a progress/state file is found, use that
pub fn get_content(conf: &config::Config) -> AnyErr<String> {
    if !conf.no_state && state::prog_exists(&conf.file_path_orig()) {
        let state_file_path = state::prog_path(&conf.file_path_orig())?;

        info!("Opening file from previously saved state.");

        let state_file = fs::read_to_string(state_file_path)?;
        // println!("state file content:\n{state_file:?}\n");
        Ok(state_file)
    } else {
        info!("Trying to open {}", &conf.file_path().display());

        Ok(fs::read_to_string(conf.file_path())?)
    }
}

/// Delete progress if exists
///
/// # Errors
///
/// - `get_prog_path()` errors
/// - `fs::remove_file()` errors
pub fn rm_prog(path: &Path) -> AnyErr<()> {
    if prog_exists(path) {
        info!("Removing state file from: {:?}", prog_path(path)?);
        fs::remove_file(prog_path(path)?)?;
    }
    Ok(())
}

/// Make item writable to file
///
/// # usage
/// ```
/// use crablit::Card;
/// use crablit::state::serialize;
///
/// let deck = vec![
///     Card::new("def1", "trm1", None),
///     Card::new("def2", "trm2", None),
///     Card::new("def3", "trm3", None),
///     Card::new("def4", "trm4", None),
/// ];
///
/// let r = "\
/// def1;trm1;Nothing
/// def2;trm2;Nothing
/// def3;trm3;Nothing
/// def4;trm4;Nothing\n";
///
/// assert_eq!(r, serialize(&deck, ';'));
/// ```
pub fn serialize(v: &[Card], delim: char) -> String {
    v.iter().fold(String::new(), |r, item| {
        r + &item.ser(&delim.to_string()) + "\n"
    })
}

/// Save progress to `data_dir`/crablit/`current_file`
///
/// # Errors
///
/// - `fs::create()` errors
/// - `get_prog_path()` errors
/// - `writeln!()` errors
pub fn save_prog(deck: &[Card], conf: &config::Config) -> AnyErr<()> {
    let ofile_path = prog_path(&conf.file_path_orig())?;
    let mut ofile = File::create(&ofile_path)?;

    writeln!(ofile, "# [crablit]")?;
    writeln!(ofile, "# delim = \'{}\'\n\n", conf.delim())?;

    trace!("r: {deck:?}");
    let content = serialize(deck, conf.delim());
    writeln!(ofile, "{content}")?;

    info!("Saved file to {}{:?}.\n\n", SPCR.repeat(2), ofile_path);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{state, Card};

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
    fn get_data_dir() {
        data_dir();
    }

    // get_prog_path
    #[test]
    fn get_progress_path() {
        assert!(prog_path(Path::new("test_prog_path.txt")).is_ok());
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
        let state_path = state::prog_path(orig_path).unwrap();

        let mut ofile = File::create(state_path).unwrap();
        writeln!(ofile, "is there anybody in there?").unwrap();

        assert!(prog_exists(orig_path));
        assert_eq!(Some(()), rm_prog(orig_path).ok());
    }

    // rm_prog
    #[test]
    fn remove_progress() {
        let orig_path = Path::new("test_rm_prog.txt");
        let state_path = state::prog_path(orig_path).unwrap();

        let mut ofile = File::create(state_path).unwrap();
        writeln!(ofile, "is there anybody in there?").unwrap();

        assert!(prog_exists(orig_path));
        assert_eq!(Some(()), rm_prog(orig_path).ok());
    }
}
