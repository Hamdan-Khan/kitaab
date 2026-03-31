use std::{ffi::OsStr, fs};

pub trait PageLoader {
    fn load_page(&self, index: u32) -> Option<String>;
    fn total_pages(&self) -> Option<u32>;
}

pub struct FsPageLoader {
    base_path: String,
}

impl PageLoader for FsPageLoader {
    fn load_page(&self, index: u32) -> Option<String> {
        let path = format!("{}/{}.md", self.base_path, index);
        match fs::read_to_string(&path) {
            Ok(content) => Some(content),
            Err(e) => {
                eprintln!("Error reading md file at {}. Error: {}", path, e);
                None
            }
        }
    }

    fn total_pages(&self) -> Option<u32> {
        match fs::read_dir(&self.base_path) {
            Ok(res) => {
                let mut count = 0;

                for entry in res {
                    let entry = match entry {
                        Ok(e) => e,
                        Err(_) => continue,
                    };

                    if entry.path().extension() == Some(OsStr::new("md")) {
                        count += 1;
                    }
                }

                Some(count)
            }
            Err(e) => {
                eprintln!(
                    "Error reading directory at {}. Error: {}",
                    self.base_path, e
                );
                None
            }
        }
    }
}
