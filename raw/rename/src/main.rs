use std::fs;
use std::error::Error;
use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn Error>> {
    let root = "../JM/";
    for entry in WalkDir::new(root)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok()) {

        let f_name = entry.file_name().to_string_lossy();

        if f_name.ends_with(')') {
            let split_1: Vec<&str> = f_name.rsplit('(').collect();
            let split_2: Vec<&str> = split_1[0].rsplit(')').collect();
            let num = split_2[1];

            let dir = String::from(root);
            let sub = f_name.to_string();

            let path = [dir, sub].join("");
            for dir in fs::read_dir(path).unwrap() {
                let old_file = dir.unwrap().path().display().to_string();
                let v: Vec<&str> = old_file.as_str().rsplit('.').collect();

                let file = v[1];
                let file = [file, "#"].join("");
                let file = [String::from(".."), file, num.to_string(), String::from(".PNG")].join("");
                println!("{} ----> {}", old_file, file);

                fs::rename(old_file, file)?;
            }
        }
    }

    Ok(())
}
