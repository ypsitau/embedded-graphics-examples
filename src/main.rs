fn main() {
    println!("Execute examples with: cargo run --example example_name");
    println!("Available examples:");
    let dir_entries = match std::fs::read_dir("examples") {
        Ok(dir_entries) => dir_entries,
        Err(error) => {
            eprintln!("examplesディレクトリを読み込めません: {error}");
            return;
        }
    };
    for dir_entry in dir_entries.flatten() {
        if !dir_entry.file_type().is_ok_and(|file_type| file_type.is_file()) { continue; }
        let path = dir_entry.path();
        if path.extension().and_then(|ext| ext.to_str()) == Some("rs") {
            println!("  {}", path.file_stem().and_then(|stem| stem.to_str()).unwrap_or_default());
        }
    }
}
