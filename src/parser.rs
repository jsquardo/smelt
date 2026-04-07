use std::collections::HashMap;

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

pub fn build_graph(entry: &str) -> HashMap<String, Vec<String>> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    let mut to_visit: Vec<String> = vec![entry.to_string()];

    while let Some(file) = to_visit.pop() {
        if graph.contains_key(&file) {
            continue;
        }
        let imports = parse_imports(&file);
        for import in &imports {
            to_visit.push(import.clone());
        }
        graph.insert(file, imports);
    }
    graph
}
