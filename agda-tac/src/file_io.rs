use agda_mode::agda::ReplState;
use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

pub fn init_module(file: &str) -> io::Result<(File, PathBuf, String)> {
    // Extracted as variable to make the borrow checker happy
    let file_dot_agda = format!("{}.agda", file);
    let path = Path::new(if file.ends_with(".agda") {
        file
    } else {
        &file_dot_agda
    });
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
    Ok((f, path.to_path_buf(), first_line))
}

pub struct Repl {
    pub agda: ReplState,
    pub file: File,
    pub path: PathBuf,
    file_buf: Vec<String>,
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
            is_plain: false,
        }
    }

    pub fn append_line_buffer(&mut self, line: String) {
        self.file_buf.push(line)
    }

    pub fn pop_line_buffer(&mut self) -> Option<String> {
        self.file_buf.pop()
    }

    pub fn set_line_buffer(&mut self, line_num: usize, line: String) {
        self.file_buf[line_num] = line
    }

    pub fn get_line_buffer(&mut self, line_num: usize) -> &String {
        &self.file_buf[line_num]
    }

    pub fn append_line(&mut self, line: String) -> io::Result<()> {
        self.file.write(line.as_bytes())?;
        self.file.write("\n".as_bytes())?;
        self.file.flush()?;
        self.append_line_buffer(line);
        Ok(())
    }
}
