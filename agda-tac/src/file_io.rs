use std::fs::{create_dir_all, remove_file, File};
use std::io::{self, BufWriter, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use agda_mode::agda::ReplState;
use ropey::{Rope, RopeSlice};

const FAIL_CREATE_DEFAULT: &str = "Failed to create default working file";

pub type Monad<T = ()> = io::Result<T>;

pub fn init_module(mut file: String) -> Monad<(File, PathBuf, String)> {
    // Extracted as variable to make the borrow checker happy
    if !file.ends_with(".agda") {
        file.push_str(".agda")
    }
    let path = Path::new(&file);
    if path.exists() {
        eprintln!("I don't want to work with existing files, sorry.");
        std::process::exit(1);
    }
    let mut f = File::create(path)?;
    let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("");
    let mod_name = path
        .file_name()
        .and_then(|s| s.to_str())
        .map(|s| s.trim_end_matches(extension))
        .map(|s| s.trim_end_matches("."))
        .map(|s| s.trim())
        .expect("File does not have a name");
    // TODO: check if it's a valid module name
    let first_line = format!("module {} where", mod_name);
    f.write(first_line.as_bytes())?;
    f.write("\n".as_bytes())?;
    f.flush()?;
    Ok((f, path.to_path_buf().canonicalize()?, first_line))
}

pub fn find_default_unwrap() -> String {
    find_default().expect(FAIL_CREATE_DEFAULT)
}

pub fn config_dir() -> Monad<PathBuf> {
    let agda_tac_dir = dirs::home_dir()
        .expect(FAIL_CREATE_DEFAULT)
        .join(".agda-tac");
    create_dir_all(&agda_tac_dir)?;
    Ok(agda_tac_dir)
}

pub fn history_file() -> Monad<PathBuf> {
    Ok(config_dir()?.join(".repl_history"))
}

pub fn find_default() -> Monad<String> {
    println!("No input file specified, using default.");
    let file_path = config_dir()?
        .join("Nameless.agda")
        .into_os_string()
        .into_string()
        .expect(FAIL_CREATE_DEFAULT);
    println!("Default to {}", file_path);
    if Path::new(&file_path).exists() {
        remove_file(&file_path)?;
    }
    Ok(file_path)
}

pub struct Repl {
    pub agda: ReplState,
    pub file: File,
    pub path: PathBuf,
    file_buf: Rope,
    pub is_plain: bool,
}

impl Repl {
    pub fn new(agda: ReplState, file: File, path: PathBuf) -> Self {
        Self {
            agda,
            file,
            path,
            file_buf: Rope::new(),
            is_plain: false,
        }
    }

    pub fn append_line_buffer(&mut self, line: &str) {
        let index = self.file_buf.len_chars();
        self.file_buf.insert(index, line)
    }

    pub fn insert_line_buffer(&mut self, line_num: usize, line: &str) {
        let index = self.file_buf.line_to_char(line_num);
        self.file_buf.insert(index, line)
    }

    pub fn line_in_buffer(&mut self, line_num: usize) -> RopeSlice {
        self.file_buf.line(line_num)
    }

    fn flush_file(&mut self) -> Monad {
        self.file.flush()
    }

    pub fn append_line(&mut self, line: &str) -> Monad {
        Self::append_line_to_file(&mut self.file, line)?;
        self.flush_file()?;
        self.append_line_buffer(line);
        Ok(())
    }

    fn append_line_to_file(file: &mut File, line: &str) -> Monad {
        file.write(line.as_bytes())?;
        file.write("\n".as_bytes())?;
        Ok(())
    }

    fn clear_file(&mut self) -> Monad {
        let file = &mut self.file;
        file.sync_all()?;
        file.set_len(0)?;
        file.seek(SeekFrom::Start(0))?;
        Ok(())
    }

    pub fn sync_buffer(&mut self) -> Monad {
        self.clear_file()?;
        self.file_buf.write_to(BufWriter::new(&self.file))?;
        self.flush_file()?;
        Ok(())
    }
}
