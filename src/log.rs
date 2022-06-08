use chrono::prelude::*;
use dirs;
use std::{
    cell::RefCell,
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

pub struct Log {
    should_print: bool,
    buffer: RefCell<String>,
    path: PathBuf,
}

impl Log {
    pub fn new() -> Self {
        Log {
            should_print: true,
            buffer: RefCell::new(String::new()),
            path: dirs::home_dir()
                .unwrap()
                .join(".tmgw-wifi-autholizer")
                .join("log.txt"),
        }
    }

    pub fn save(&self) {
        fs::create_dir_all(&self.path.parent().unwrap()).unwrap();
        let mut f = match File::create(&self.path) {
            Ok(f) => f,
            Err(e) => {
                self.record_error(&format!("ログファイルの生成に失敗しました。詳細: {}", e));
                std::process::exit(1);
            }
        };
        match f.write_all(self.buffer.borrow().as_bytes()) {
            Ok(_) => (),
            Err(e) => {
                self.record_error(&format!(
                    "ログファイルへの書き込みに失敗しました。詳細: {}",
                    e
                ));
                std::process::exit(1);
            }
        };
    }

    pub fn record_info(&self, text: &str) {
        let date = Local::now();
        let line = format!("[{}] [INFO] {}\n", date.to_owned(), text);
        self.buffer.replace(self.buffer.take() + &line);
        if self.should_print {
            print!("{}", line);
        }
    }

    pub fn record_error(&self, text: &str) {
        let date = Local::now();
        let line = format!("[{}] [ERROR] {}\n", date.to_owned(), text);
        self.buffer.replace(self.buffer.take() + &line);
        if self.should_print {
            eprint!("{}", line);
        }
    }
}
