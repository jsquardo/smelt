pub fn resolve_path(current_file: &str, import_path: &str) -> String {
    let base = std::path::PathBuf::from(current_file);
    let dir = base.parent().unwrap();
    let clean = import_path.trim_start_matches("./");
    let resolved = dir.join(clean);
    resolved.to_string_lossy().to_string()
}

pub fn parse_imports(file_path: &str) -> Vec<String> {
    let contents = std::fs::read_to_string(file_path).unwrap();
    let mut paths: Vec<String> = Vec::new();

    for line in contents.lines() {
        if line.contains("from '") {
            let path = line.split('\'').nth(1).unwrap();
            paths.push(resolve_path(file_path, path));
        } else if line.contains("from \"") {
            let path = line.split('"').nth(1).unwrap();
            paths.push(resolve_path(file_path, path));
        }
    }
    paths
}
