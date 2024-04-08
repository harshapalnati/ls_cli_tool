use std::fs;
use std::io::Error;
use std::path::PathBuf;
use ansi_term::Colour;


pub struct Ls {
    pub path :PathBuf, 
}

impl Ls{
    pub fn new(path:PathBuf) -> Self{
        Ls { path }
    }
    pub fn path_exists (&self) -> bool{
        self.path.exists()
    }
    
    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }
    
    pub fn read_and_print_directories (&self)-> Result<String,Error> {
    
            let entries = fs::read_dir(&self.path)?;
            let mut result = String::new();
           

    
            for entry in entries {
                let entry = entry?;
                let filename = entry.file_name().into_string().unwrap_or_else(|_| String::from(""));
                let icon = if entry.path().is_dir() {
                
                
                    Colour::Blue.paint("📁").to_string()
                } else {
                    
                    Colour::Green.paint("📄").to_string()
                   
                };
                let colored_filename = if entry.path().is_dir() {
                    Colour::Blue.paint(filename).to_string()
                } else {
                    Colour::Green.paint(filename).to_string()
                };
                result.push_str(&format!("{} {}\n", icon,colored_filename));
            }
    
            Ok(result)
    }
}

