use std::io::Read;
use std::io::Write;
use std::fs::File;
use std::fs;

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

    for entry in WalkDir::new("../src/lib/locales") {
        let entry = entry.unwrap();
        println!("{}", entry.path().display());

        let file_name = entry.file_name().to_str().unwrap();

        if file_name == "mod.rs" {
            let mut src = File::open(&entry.path())?;
            let mut data = String::new();
            src.read_to_string(&mut data)?;
            // drop(src); 

            let result = data.lines().map(|line|{
                if let Some(module) = get_between(line, "require(\"./", "\");") {
                    "mod " .to_string() + &module + ";"
                }else{
                    line.to_string()
                }

            }).collect::<Vec<_>>().join("\n") + "\n";

            // address.default_country = require("./default_country");
            // address.postcode = require("./postcode");
            src.write_all(result.as_bytes())?;
            


        }


    }

    Ok(())
}


fn get_between(src:&str, start: &str, end: &str) -> Option<String> {
    if let Some(mut start_pos) = src.find(start) {
        start_pos+= start.len();
        println!("{:?}", start_pos);
        if let Some(end_pos) = src[start_pos..].find(end) {
            println!("{:?}", end_pos);
            return Some(src[start_pos..start_pos+end_pos].to_string());
        }
    }
    None
}

#[test]
fn test_between() {
    println!("{:?}", get_between(r#"address.default_country = require("./default_country");YOLOOO"#, "require(\"./", "\");"));
}