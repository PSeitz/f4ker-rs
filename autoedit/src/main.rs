use std::io::{self, BufReader};
use std::io::Read;
use std::io::Write;
use std::fs::File;
use std::fs::OpenOptions;
// use std::fs;
use std::io::prelude::*;
use regex::Regex;

fn main() -> Result<(), std::io::Error> {
    use walkdir::WalkDir;

    for entry in WalkDir::new("../src/locales") {
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
    for entry in WalkDir::new("../src/locales") {
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
    // for entry in WalkDir::new("../src/locales") {
    //     let entry = entry.unwrap();
    //     let file_name = entry.file_name().to_str().unwrap();

    //     if file_name == "mod.rs" {
    //         let mut lines = lines(&entry.path());

    //         let reexport = lines.iter()
    //         .flat_map(|line| get_between(&line, "mod", ";") )
    //         .map(|module|format!("pub use {}::*;", module)).collect::<Vec<_>>();

    //         lines.extend(reexport);

    //         write_lines(lines, &entry.path());

    //     }
    // }


    let re = Regex::new(r#"[A-Za-z]*\.([A-Za-z]*)\s*=\s*(".*").*"#).unwrap();

    // convert module["exports"] = [
    // az.title = "Azerbaijani"; to const title
    // az.separator = "Azerbaijani"; to const title
    for entry in WalkDir::new("../src/locales") {
        let entry = entry.unwrap();
        if entry.path().is_dir() {
            continue;
        }
        let file_name = entry.file_name().to_str().unwrap();

        let result = lines(&entry.path()).into_iter().map(|line|{
            if line.contains(r#"module["exports"] = ["#) {
                format!("pub static {}: &'static [&'static str] = &[ ", &file_name[..file_name.len()-3])
            }else if let Some(pat) = re.captures(&line) {
                format!("pub const {}: &str = {};", &pat[1], &pat[2])
            }else{
                line
            }

        }).collect::<Vec<_>>();

        write_lines(result, &entry.path());
    }

    // rename .js to .rs
    for entry in WalkDir::new("../src/locales") {
        let entry = entry.unwrap();
        if entry.path().is_dir() {
            continue;
        }
        let file_name = entry.file_name().to_str().unwrap();
        if file_name.ends_with(".js") {
            let entree = entry.path().to_str().unwrap().to_string();
            std::fs::rename(entry.path(), entree.replace(".js", ".rs"))?;
        }
    }

    // handle lib files
    for entry in WalkDir::new("../src/lib/") {
        let entry = entry.unwrap();
        if entry.path().is_dir() || entry.depth() >= 2 {
            continue;
        }
        let file_name = entry.file_name().to_str().unwrap();
        println!("{:?}", file_name);
        let re = Regex::new(r"^function ([A-Z][A-Za-z]*).*").unwrap(); // function Address (faker) {
        let re2 = Regex::new(r"^var ([A-Z][A-Za-z]*).*").unwrap(); // var Phone = function (faker) {

        let mut lines = lines(&entry.path());

        let mut structs = vec![];

        let mut lines: Vec<String> = lines.iter().flat_map(|line|{
            if let Some(pat) = re.captures(&line) {
                structs.push(pat[1].to_string());
                return vec![format!("impl {} {{",  &pat[1]),
                    "    fn new() -> Self {".to_string(),
                    "".to_string(),
                    "    }".to_string()
                ];

            }
            if let Some(pat) = re2.captures(&line) {
                println!("{:?}", &pat[1]);
                structs.push(pat[1].to_string());
                return vec![format!("impl {} {{",  &pat[1]),
                    "    fn new() -> Self {".to_string(),
                    "".to_string(),
                    "    }".to_string()
                ];
            }
            vec![line.to_string()]
        }).collect();



        for structo in structs {
            lines.insert(0, format!("struct {} {{", structo));
            lines.insert(1, "{".to_string());
            lines.insert(2, "}".to_string());
        }

        //this.zipCode = function(format) {

        let re = Regex::new(r"^\s*(this|self)\.([A-Za-z]*)\s*=\s*function\s*[A-Za-z]*\s*\(([A-Za-z,\s]*)\)*.").unwrap(); // var Phone = function (faker) {
        // MOETHOS
        let mut lines: Vec<String> = lines.iter().flat_map(|line|{
            if let Some(pat) = re.captures(&line) {
                // println!("{:?} {:?}", pat[2].to_string(), pat[3].to_string());
                if !pat[3].to_string().is_empty(){

                    let params = pat[3].to_string().split(",").map(|el| el.to_string() + ": &str").collect::<Vec<_>>().join(", ");

                    return vec![format!("fn {}(&self, {}) -> String {{", pat[2].to_string(), params)]
                }
                return vec![format!("fn {}(&self) -> String {{", pat[2].to_string())]

            }
            return vec![line.to_string()]
        }).collect();

        write_lines(lines, &entry.path());

        if file_name.ends_with(".js") {
            let entree = entry.path().to_str().unwrap().to_string();
            let entree = entree.replace("index.js", "mod.rs");
            let entree = entree.replace(".js", ".rs");
            std::fs::rename(entry.path(), entree)?;
        }
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