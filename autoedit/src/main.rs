use std::io::Read;
use std::io::Write;
use std::fs::File;

fn main() -> Result<(), std::io::Error> {
    use walkdir::WalkDir;

    for entry in WalkDir::new("../src/lib/locales") {
        let entry = entry.unwrap();
        println!("{}", entry.path().display());

        let file_name = entry.file_name().to_str().unwrap();

        if file_name == "index.js" {
            let mut src = File::open(&entry.path())?;
            let mut data = String::new();
            src.read_to_string(&mut data)?;
            
            let result = data.lines().skip(2).collect::<Vec<_>>().join("\n") + "\n";

            let entree = entry.path().to_str().unwrap().to_string();

            src.write(result.as_bytes())?;
            drop(src); 

            std::fs::rename(entry.path(), entree.replace("index.js", "mod.rs"))?;
        }


    }

    Ok(())
}
