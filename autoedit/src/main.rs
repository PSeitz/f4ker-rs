use std::io::{self, BufReader};
use std::io::Read;
use std::io::Write;
use std::fs::File;
use std::fs::OpenOptions;
// use std::fs;
use std::io::prelude::*;
fn main() -> Result<(), std::io::Error> {
    use walkdir::WalkDir;

    for entry in WalkDir::new("../src/lib/locales") {
        let entry = entry.unwrap();
        // println!("{}", entry.path().display());

        let file_name = entry.file_name().to_str().unwrap();

        if file_name == "index.js" {
            let mut src = File::open(&entry.path())?;
            let mut data = String::new();
            src.read_to_string(&mut data)?;
            drop(src); 

            let result = data.lines().skip(2).collect::<Vec<_>>().join("\n") + "\n";
            
            let entree = entry.path().to_str().unwrap().to_string();
            File::create(&entree.replace("index.js", "mod.rs"))?.write_all(result.as_bytes())?;

            std::fs::remove_file(entry.path())?;
            // std::fs::rename(entry.path(), entree.replace("index.js", "mod.rs"))?;
        }

    }

    //require to mod
    for entry in WalkDir::new("../src/lib/locales") {
        let entry = entry.unwrap();
        let file_name = entry.file_name().to_str().unwrap();

        if file_name == "mod.rs" {

            let result = lines(&entry.path()).into_iter().map(|line|{
                if let Some(module) = get_between(&line, "require(\"./", "\");") {
                    "mod " .to_string() + &module + ";"
                }else{
                    line
                }

            }).collect::<Vec<_>>();

            write_lines(result, &entry.path());

        }
    }

    //reexport
    for entry in WalkDir::new("../src/lib/locales") {
        let entry = entry.unwrap();
        let file_name = entry.file_name().to_str().unwrap();

        if file_name == "mod.rs" {
            let mut lines = lines(&entry.path());

            let reexport = lines.iter()
            .flat_map(|line| get_between(&line, "mod", ";") )
            .map(|module|format!("pub use {}::*;", module)).collect::<Vec<_>>();

            lines.extend(reexport);

            write_lines(lines, &entry.path());

        }
    }

    // convert module["exports"] = [
    for entry in WalkDir::new("../src/lib/locales") {
        let entry = entry.unwrap();
        if entry.path().is_dir() {
            continue;
        }
        let file_name = entry.file_name().to_str().unwrap();

        let result = lines(&entry.path()).into_iter().map(|line|{
            if line.contains(r#"module["exports"] = ["#) {
                format!("pub static {}: &'static [&'static str] = &[ ", &file_name[..file_name.len()-3])
            }else{
                line
            }

        }).collect::<Vec<_>>();

        write_lines(result, &entry.path());



    }

    Ok(())
}


fn lines(path: &std::path::Path) -> Vec<String> {
    BufReader::new(File::open(path).expect(&format!("{:?}", path))).lines().map(|line|line.expect(&format!("{:?}", path))).collect()
}

fn write_lines(lines: Vec<String>, path: &std::path::Path) {
    let text = lines.join("\n") + "\n";
    let mut src = OpenOptions::new().create(true).truncate(true).write(true).open(path).expect(&format!("{:?}", path));
    src.write_all(text.as_bytes()).expect(&format!("{:?}", path));
}

fn get_between(src:&str, start: &str, end: &str) -> Option<String> {
    if let Some(mut start_pos) = src.find(start) {
        start_pos+= start.len();
        // println!("{:?}", start_pos);
        if let Some(end_pos) = src[start_pos..].find(end) {
            // println!("{:?}", end_pos);
            return Some(src[start_pos..start_pos+end_pos].to_string());
        }
    }
    None
}

#[test]
fn test_between() {
    println!("{:?}", get_between(r#"address.default_country = require("./default_country");YOLOOO"#, "require(\"./", "\");"));
}