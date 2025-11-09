use std::fs::{create_dir_all, remove_file, File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Seek, SeekFrom, Write};
use std::ops::Range;
use std::path::PathBuf;

use ropey::{Rope, RopeSlice};

use agda_mode::agda::ReplState;
use agda_mode::pos::{InteractionPoint, Interval};

const FAIL_CREATE_DEFAULT: &str = "Failed to create default working file";

pub type Monad<T = ()> = io::Result<T>;

#[derive(Debug)]
pub struct InitModule(pub File, pub PathBuf, pub Rope);

pub fn init_module(mut file: PathBuf, allow_ex: bool) -> Monad<InitModule> {
    file.set_extension("agda");
    let path = &file;
    if path.exists() {
        if !allow_ex {
            eprintln!("I don't want to work with existing files, sorry.");
            std::process::exit(1);
        } else {
            let file = OpenOptions::new().read(true).write(true).open(path)?;
            let mut perms = file.metadata()?.permissions();
            perms.set_readonly(false);
            file.set_permissions(perms)?;
            let rope = Rope::from_reader(BufReader::new(&file))?;
            return Ok(InitModule(file, path.to_path_buf().canonicalize()?, rope));
        }
    }
    let mod_name = path
        .file_stem()
        .and_then(|s| s.to_str())
        .map(str::trim)
        .expect("File does not have a name");
    // TODO: check if it's a valid module name
    let first_line = format!("module {} where\n", mod_name);
    let mut f = File::create(path)?;
    f.write(first_line.as_bytes())?;
    f.flush()?;
    Ok(InitModule(
        f,
        path.to_path_buf().canonicalize()?,
        Rope::from(first_line),
    ))
}

pub fn find_default_unwrap() -> PathBuf {
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

pub fn find_default() -> Monad<PathBuf> {
    println!("No input file specified, using default.");
    let file_path = config_dir()?.join("Nameless.agda");
    println!("Default to {}", file_path.display());
    if file_path.exists() {
        remove_file(&file_path)?;
    }
    Ok(file_path)
}

pub fn agda_to_rope_range(i: &Interval) -> Range<usize> {
    i.range_shift_left(1)
}

#[derive(Debug)]
pub struct Repl {
    pub agda: ReplState,
    pub file: File,
    pub path: PathBuf,
    file_buf: Rope,
    pub is_plain: bool,
}

impl Repl {
    pub fn new(agda: ReplState, file: File, path: PathBuf, file_buf: Rope) -> Self {
        Self {
            agda,
            file,
            path,
            file_buf,
            is_plain: false,
        }
    }

    pub fn append_buffer(&mut self, text: &str) {
        let index = self.file_buf.len_chars();
        self.file_buf.insert(index, text)
    }

    pub fn remove_last_line_buffer(&mut self) {
        let line_last = self.file_buf.len_lines() - 2;
        let line_start = self.file_buf.line_to_char(line_last);
        let doc_end = self.file_buf.len_chars();
        self.file_buf.remove(line_start..doc_end)
    }

    pub fn remove_line_buffer(&mut self, line_num: usize) {
        // Previous line
        let line_start = self.file_buf.line_to_char(line_num - 1);
        let line_end = self.file_buf.line_to_char(line_num);
        self.file_buf.remove(line_start..line_end)
    }

    pub fn line_of_offset(&mut self, offset: usize) -> usize {
        self.file_buf.char_to_line(offset)
    }

    pub fn fill_goal_buffer(&mut self, i: InteractionPoint, text: &str) {
        let interval = i.the_interval();
        self.file_buf.remove(agda_to_rope_range(interval));
        self.file_buf.insert(interval.start.pos - 1, text);
    }

    pub fn intros_in_goal_buffer(&mut self, i: InteractionPoint, text: &str) -> Option<()> {
        let interval = i.the_interval();
        let line_num = interval.start.line - 1;
        let line_start = self.file_buf.line_to_char(line_num);
        let line = self.file_buf.line(line_num);
        let (idx, _) = (line.chars().into_iter().enumerate()).find(|(_, c)| c == &'=')?;
        self.file_buf.insert_char(line_start + idx, ' ');
        self.file_buf.insert(line_start + idx, text);
        Some(())
    }

    pub fn insert_line_buffer(&mut self, line_num: usize, line: &str) {
        // Previous line
        let index = self.file_buf.line_to_char(line_num - 1) - 1;
        self.file_buf.insert(index, line);
        self.file_buf.insert(index, "\n")
    }

    pub fn dump_proof(&mut self) -> Monad {
        self.file_buf.write_to(BufWriter::new(io::stdout()))
    }

    pub fn line_in_buffer(&mut self, line_num: usize) -> RopeSlice {
        self.file_buf.line(line_num)
    }

    fn flush_file(&mut self) -> Monad {
        self.file.flush()
    }

    pub fn line_count(&self) -> usize {
        self.file_buf.len_lines()
    }

    pub fn append(&mut self, text: &str) -> Monad {
        self.append_buffer(text);
        Self::append_to_file(&mut self.file, text.as_bytes())?;
        self.flush_file()
    }

    pub fn remove_last_line(&mut self) -> Monad {
        self.remove_last_line_buffer();
        self.sync_buffer()
    }

    fn append_to_file(file: &mut File, text: &[u8]) -> Monad<usize> {
        file.write(text)
    }

    fn clear_file(&mut self) -> Monad<u64> {
        let file = &mut self.file;
        file.sync_all()?;
        file.set_len(0)?;
        file.seek(SeekFrom::Start(0))
    }

    pub fn sync_buffer(&mut self) -> Monad {
        self.clear_file()?;
        self.file_buf.write_to(BufWriter::new(&self.file))?;
        self.flush_file()
    }
}
