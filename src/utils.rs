use std::fs;

// todo: add md files discovery instead of reading a single file
pub fn read_md(path: &str) -> String {
     match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) =>{ 
            println!("Error reading md file: {}", e);
            String::from("Failed to read md file in the /content dir. Make sure there is content placed there.")},
     }
}