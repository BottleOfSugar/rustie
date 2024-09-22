use std::process::Command;
use std::fs;
use std::io::{self, Write};

pub struct LinuxFramework;

impl LinuxFramework {
    // Funkcja do wykonywania komend systemowych
    pub fn run_command(command: &str) -> io::Result<String> {
        let output = Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    // Funkcja do tworzenia pliku
    pub fn create_file(path: &str, content: &str) -> io::Result<()> {
        let mut file = fs::File::create(path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    // Funkcja do usuwania pliku
    pub fn delete_file(path: &str) -> io::Result<()> {
        fs::remove_file(path)?;
        Ok(())
    }

    // Funkcja do przeglądania zawartości katalogu
    pub fn list_files(dir: &str) -> io::Result<Vec<String>> {
        let paths = fs::read_dir(dir)?
            .filter_map(Result::ok)
            .map(|entry| entry.file_name().into_string().unwrap())
            .collect();
        Ok(paths)
    }
}
