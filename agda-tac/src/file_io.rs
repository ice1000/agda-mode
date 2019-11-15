use agda_mode::agda::ReplState;
use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

pub fn init_module(file: &str) -> io::Result<(File, PathBuf)> {
    let path = Path::new(&file);
    if path.exists() {
        eprintln!("I don't want to work with existing files, sorry.");
        std::process::exit(1);
    }
    let mut f = File::create(path)?;
    let file_name = path
        .file_name()
        .and_then(|s| s.to_str())
        .expect("File does not have a name");
    f.write("module ".as_bytes())?;
    f.write(file_name.as_bytes())?;
    f.write(" where\n".as_bytes())?;
    f.flush()?;
    Ok((f, path.to_path_buf()))
}

pub struct Repl {
    pub agda: ReplState,
    pub file: File,
    pub path: PathBuf,
    pub is_plain: bool,
}

impl Repl {
    pub fn new(agda: ReplState, file: File, path: PathBuf) -> Self {
        Self {
            agda,
            file,
            path,
            is_plain: false,
        }
    }
}
