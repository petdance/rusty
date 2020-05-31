use walkdir::WalkDir;

fn main() {
    for entry in WalkDir::new("target/debug").into_iter().filter_map(|e| e.ok()) {
        println!("{}", entry.path().display());
    }
}
