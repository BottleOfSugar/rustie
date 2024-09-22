fn main() {
    let framework = my_linux_framework::LinuxFramework;

    // Przykład użycia funkcji do wykonywania komend
    match framework.run_command("ls") {
        Ok(output) => println!("Zawartość katalogu:\n{}", output),
        Err(e) => eprintln!("Błąd: {}", e),
    }

    // Przykład tworzenia pliku
    if let Err(e) = framework.create_file("test.txt", "To jest test.") {
        eprintln!("Błąd przy tworzeniu pliku: {}", e);
    }

    // Przykład usuwania pliku
    if let Err(e) = framework.delete_file("test.txt") {
        eprintln!("Błąd przy usuwaniu pliku: {}", e);
    }

    // Przykład przeglądania plików
    match framework.list_files(".") {
        Ok(files) => {
            println!("Pliki w bieżącym katalogu:");
            for file in files {
                println!("{}", file);
            }
        },
        Err(e) => eprintln!("Błąd przy przeglądaniu plików: {}", e),
    }
}
