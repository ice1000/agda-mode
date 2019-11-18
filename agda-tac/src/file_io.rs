use std::fs::{create_dir_all, remove_file, File};
use std::io::{self, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use agda_mode::agda::ReplState;

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
    file_buf: Vec<String>,
    last_line: usize,
    pub is_plain: bool,
}

impl Repl {
    pub fn new(agda: ReplState, file: File, path: PathBuf) -> Self {
        let file_buf = Vec::with_capacity(5);
        Self {
            agda,
            file,
            path,
            file_buf,
            last_line: 0,
            is_plain: false,
        }
    }

    pub fn any_goals_in_buffer(&self) -> bool {
        self.last_line == self.file_buf.len()
    }

    pub fn append_line_buffer(&mut self, line: String) {
        // TODO: support {!!}?
        if self.any_goals_in_buffer() && !line.contains("?") {
            self.last_line += 1;
        }
        self.file_buf.push(line)
    }

    pub fn pop_line_buffer(&mut self) -> Option<String> {
        if self.any_goals_in_buffer() && self.last_line > 0 {
            self.last_line -= 1;
        }
        self.file_buf.pop()
    }

    pub fn set_line_buffer(&mut self, line_num: usize, line: String) {
        if line.contains("?") {
            self.last_line = line_num.min(self.last_line);
        }
        self.file_buf[line_num] = line
    }

    pub fn insert_line_buffer(&mut self, line_num: usize, line: String) {
        if line.contains("?") {
            self.last_line = line_num.min(self.last_line);
        }
        self.file_buf.insert(line_num, line)
    }

    pub fn get_line_buffer(&mut self, line_num: usize) -> &String {
        &self.file_buf[line_num]
    }

    fn flush_file(&mut self) -> Monad {
        self.file.flush()
    }

    pub fn append_line(&mut self, line: String) -> Monad {
        Self::append_line_to_file(&mut self.file, &line)?;
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
        let mut recalculated_last_line = 0usize;
        let mut found_goal = false;
        for line in self.file_buf.iter() {
            Self::append_line_to_file(&mut self.file, &line)?;
            if !found_goal {
                if line.contains("?") {
                    found_goal = true;
                } else {
                    recalculated_last_line += 1;
                }
            }
        }
        self.flush_file()?;
        self.last_line = recalculated_last_line;
        Ok(())
    }
}
