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
            drop(src);  // Close the file early

            let result = data.lines().skip(2).collect::<Vec<_>>().join("\n");

            // Run the replace operation in memory
            // let new_data = data.replace(&*word_from, &*word_to);

            // Recreate the file and dump the processed contents to it
            let mut dst = File::create(&entry.path())?;
            dst.write(result.as_bytes())?;
        }


    }

    Ok(())
}
