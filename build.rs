use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Get the output directory where the executable will be placed
    let out_dir = env::var("OUT_DIR").unwrap();
    // Navigate from OUT_DIR (target/debug/build/<pkg>/out) to target/debug or target/release
    let target_dir = Path::new(&out_dir)
        .ancestors()
        .nth(3)
        .expect("Could not find target directory");

    let source_dir = Path::new("quizz");
    let dest_dir = target_dir.join("quizz");

    // Copy the quizz directory to the target directory
    if source_dir.exists() {
        copy_dir_recursive(source_dir, &dest_dir).expect("Failed to copy quizz directory");
    }

    // Tell Cargo to rerun this script if the quizz directory changes
    println!("cargo:rerun-if-changed=quizz");
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = dst.join(entry.file_name());

        if path.is_dir() {
            copy_dir_recursive(&path, &dest_path)?;
        } else {
            fs::copy(&path, &dest_path)?;
        }
    }

    Ok(())
}
