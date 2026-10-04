fn main() {
    println!("Execute examples with: cargo run --example example_name");
    println!("Available examples:");
    let dir_entries = match std::fs::read_dir("examples") {
        Ok(dir_entries) => dir_entries,
        Err(error) => {
            eprintln!("can't read examples directory: {error}");
            return;
        }
    };
    let example_names_to_exclude = ["common"]; 
    let mut example_names = Vec::<String>::new();
    for dir_entry in dir_entries.flatten() {
        if !dir_entry.file_type().is_ok_and(|file_type| file_type.is_file()) { continue; }
        let path = dir_entry.path();
        if path.extension().and_then(|ext| ext.to_str()) == Some("rs") {
            let example_name = path.file_stem().and_then(|stem| stem.to_str()).unwrap_or_default().to_string();
            if !example_names_to_exclude.contains(&example_name.as_str()) {
                example_names.push(example_name.clone());
            }
        }
    }
    example_names.sort();
    for example_name in example_names {
        println!("  {}", example_name);
    }
}
