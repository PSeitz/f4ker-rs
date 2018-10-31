use std::io::{self, BufReader};
use std::io::Read;
use std::io::Write;
use std::fs::File;
use std::fs::OpenOptions;
// use std::fs;
use std::io::prelude::*;
use regex::Regex;

fn find_matching_braces(text: &str, opening_brace:char, closing_brace:char) -> Option<usize> {
    let mut num = 1;
    for (i, cha) in text.char_indices() {
        if cha == closing_brace {
            num -=1;
            if num == 0 {
                return Some(i + closing_brace.len_utf8());
            }
        }
        if cha == opening_brace {
            num +=1;
        }
    }
    None
}

#[test]
fn test_matching_braces() {
    // let test_text = "if let Some(pat) = expr {unimplemented!(); }"
    println!("{:?}", find_matching_braces("unimplemented!(); }", '{', '}'));
}


#[test]
fn testo() {
    let mut orig =  "    return faker.random.arrayElement(faker.definitions.name.first_name);".to_string();
    let find = "faker.random.arrayElement(";
    if let Some(pos) = orig.find_end(find) {
        orig.replace_range(pos-find.len()..pos, "thread_rng().choose(");
        assert_eq!(orig, "    return thread_rng().choose(faker.definitions.name.first_name);");

        // let offset = pos-find.len() + "thread_rng().choose(".len();
        // let _wa = std::str::from_utf8(&orig.as_bytes()[offset .. ]).unwrap();

        assert_eq!(orig, "    return thread_rng().choose(faker.definitions.name.first_name);");

        let pos = orig.find_end("thread_rng().choose(").unwrap();
        let end_braces = find_matching_braces(&orig[pos..], '(', ')').unwrap();
        orig.insert_str(pos+end_braces, ".unwrap()");
        assert_eq!(orig, "    return thread_rng().choose(faker.definitions.name.first_name).unwrap();");

        // assert_eq!(cooel, "    return thread_rng().choose(faker.definitions.name");
    }
}



use std::path::Path;

trait BetterAccess {
    fn gimme_that(&self) -> String;
}
impl BetterAccess for Path {
    fn gimme_that(&self) -> String {
        self.to_str().unwrap().to_string()
    }
}
impl BetterAccess for std::ffi::OsStr {
    fn gimme_that(&self) -> String {
        self.to_str().unwrap().to_string()
    }
}
impl BetterAccess for Option<&std::ffi::OsStr> {
    fn gimme_that(&self) -> String {
        self.as_ref().unwrap().to_str().unwrap().to_string()
    }
}

trait BetterFilename {
    fn gimme_filename(&self) -> String;
}
impl BetterFilename for Path {
    fn gimme_filename(&self) -> String {
        self.file_name().unwrap().to_str().unwrap().to_string()
    }
}
impl BetterFilename for Option<&Path> {
    fn gimme_filename(&self) -> String {
        self.unwrap().file_name().unwrap().to_str().unwrap().to_string()
    }
}
impl BetterFilename for walkdir::DirEntry {
    fn gimme_filename(&self) -> String {
        self.file_name().to_str().unwrap().to_string()
    }
}

trait BetterAccessDir {
    fn read_diro(&self) -> Vec<std::fs::DirEntry>;
}
impl BetterAccessDir for Path {
    fn read_diro(&self) -> Vec<std::fs::DirEntry> {
        self.read_dir().unwrap().map(|entry|entry.unwrap()).collect()
    }
}

impl BetterAccessDir for Option<&Path> {
    fn read_diro(&self) -> Vec<std::fs::DirEntry> {
        self.unwrap().read_dir().unwrap().map(|entry|entry.unwrap()).collect()
    }
}

trait FindEnd {
    fn find_end(&self, pat: &str) -> Option<usize>;
}

impl FindEnd for &str {
    fn find_end(&self, pat: &str) -> Option<usize>{
        self.find(pat).map(|pos|pos+pat.len())
    }
}

impl FindEnd for String {
    fn find_end(&self, pat: &str) -> Option<usize>{
        self.find(pat).map(|pos|pos+pat.len())
    }
}


fn main() -> Result<(), std::io::Error> {
    use walkdir::WalkDir;

    for entry in WalkDir::new("../src/locales") {
        let entry = entry.unwrap();
        // println!("{}", entry.path().display());

        let file_name = entry.gimme_filename();

        if file_name == "index.js" {
            let result = into_lines(&entry.path()).into_iter().skip(2).collect::<Vec<_>>().join("\n") + "\n";

            let entree = entry.path().gimme_that();
            File::create(&entree.replace("index.js", "mod.rs"))?.write_all(result.as_bytes())?;

            std::fs::remove_file(entry.path())?;
            // std::fs::rename(entry.path(), entree.replace("index.js", "mod.rs"))?;
        }

    }

    //require to mod
    for entry in WalkDir::new("../src/locales") {
        let entry = entry.unwrap();
        let file_name = entry.gimme_filename();

        if file_name == "mod.rs" {
            let lines = into_lines(&entry.path());
            // let lines = into_lines(&entry.path()).into_iter().map(|line|{
            //     let mut line = if let Some(module) = get_between(&line, "require(\"./", "\");") {
            //         "mod " .to_string() + &module + ";"
            //     }else if line.trim().starts_with("pub mod"){
            //         (&line.trim()[3..]).to_string()
            //     }else{
            //         line
            //     };
            //     line
            // }).collect::<Vec<_>>();

            let mut sub_modules:Vec<_> = entry.path()
                .parent()
                .read_diro()
                .iter()
                .map(|entry|entry.path().file_stem().gimme_that())
                .filter(|file_name|file_name != "mod")
                .map(|mod_name|{
                    if entry.depth() <= 2{
                        format!("pub mod {};", mod_name)
                    }else{
                        format!("mod {};", mod_name)
                    }
                })
                .collect();
            sub_modules.sort();
            let lines = sub_modules.into_iter()
                .chain(lines.into_iter()
                    .filter(|el|el.trim() != "")
                    .filter(|el|!el.contains("require"))
                    .filter(|el|!el.contains("mod"))
                ).collect();

            write_lines(lines, &entry.path());

        }
    }

    //reexport
    for entry in WalkDir::new("../src/locales") {
        let entry = entry.unwrap();
        let file_name = entry.gimme_filename();

        if file_name == "mod.rs" {
            let mut liness:Vec<_> = into_lines(&entry.path()).into_iter().filter(|el|!el.contains("pub use")).collect();

            // let reexport = liness.iter()
            // .filter(|el|!el.contains("pub use"))
            // .filter(|line|!line.trim().starts_with("//")) // is commented
            // .flat_map(|line| get_between(&line, "mod", ";") )
            // .filter(|module|{
            //     let mod_path = entry.path().parent().unwrap().gimme_that()+"/" + module.trim()+".rs";
            //     let path = Path::new(&mod_path);
            //     if !path.exists(){
            //         return true;
            //     }
            //     let commented_file = lines(&path).iter().all(|line| line.trim().starts_with("//") || line.trim()=="");
            //     !commented_file
            // })
            // .map(|module|format!("pub use self::{}::*;", module.trim())).collect::<Vec<_>>();

            // liness.extend(reexport);

            write_lines(liness, &entry.path());

        }
    }

    let re_static_name = Regex::new(r#"(\s*pub static )([A-Za-z_]*)(: &'static \[&'static str\].*)"#).unwrap();
    //convert static arrays to upper case
    for entry in WalkDir::new("../src/locales") {
        let entry = entry.unwrap();
        if entry.path().is_dir(){
            continue;
        }
        let lines:Vec<_> = into_lines(&entry.path());

        let lines = lines.iter().map(|line|{
            re_static_name.replace(line, |caps: &regex::Captures| {
                caps[1].to_string()+ &caps[2].to_uppercase()+ &caps[3]
            }).to_string()
        }).collect();

        write_lines(lines, &entry.path());

    }

    use std::collections::HashMap;
    use std::collections::HashSet;
    let mut file_name_to_submodules:HashMap<String, HashSet<(String,String)>> = HashMap::default();
    // let mut file_name_to_submodules_always:HashMap<String, HashSet<String>> = HashMap::default();
    //collect a list of all available data
    for entry in WalkDir::new("../src/locales") {
        let entry = entry.unwrap();
        let file_name = entry.gimme_filename();

        if entry.path().is_dir() && entry.depth() == 2 {
            let set = file_name_to_submodules.entry(file_name.to_string()).or_insert(HashSet::new());
            let sub_modules: HashSet<(String, String)> = entry.path()
                .read_diro().iter()
                .filter(|entry|entry.file_name() != "mod.rs")
                .flat_map(|entry|{
                    if entry.path().is_dir() {
                        return vec![].into_iter();
                    }
                    let lines = into_lines(&entry.path());
                    let vecco = lines.iter().flat_map(|line|re_static_name.captures(line)).map(|cap|cap[2].to_string())
                        .map(|array_name|(entry.path().file_stem().gimme_that(),array_name)).collect::<Vec<_>>();
                    vecco.into_iter()
                })
                .collect();
            set.extend(sub_modules.clone());

        }
    }

    // name_first_name: en::name::first_name(),
    let vecco: Vec<String> = file_name_to_submodules.iter()
        .flat_map(|(file_name, sub_modules)|{
            let file_name = file_name.to_string();
            sub_modules.iter().map(move |(sub_mod, array_name)|{

                let fun_name = if sub_mod == &array_name.to_lowercase() {
                    array_name.to_lowercase()
                }else{
                    sub_mod.to_string()+"_"+&array_name.to_lowercase()
                };


                let name = if sub_mod == &array_name.to_lowercase() {
                    file_name.to_string()+"_"+&array_name.to_lowercase()
                }else{
                    file_name.to_string()+"_"+&sub_mod.to_string()+"_"+&array_name.to_lowercase()
                };
                name + ": "  + &format!("{}::{}()", file_name, fun_name)
            })
        }).collect();

    println!("{}", vecco.join(",\n"));
    //Add Accessor functions
    println!("{:?}", file_name_to_submodules);
    for entry in WalkDir::new("../src/locales") {
        let entry = entry.unwrap();
        let file_name = entry.gimme_filename();
        if file_name == "mod.rs" && entry.depth() == 3 {
            let file_name = entry.path().parent().gimme_filename();
            let all_submodules = file_name_to_submodules.entry(file_name.to_string()).or_insert(HashSet::new());
            // let inter_set = file_name_to_submodules_always.entry(file_name.to_string()).or_insert(HashSet::new());
            let sub_modules: HashSet<_> = entry.path().parent().unwrap()
                .read_diro().iter()
                .filter(|entry|entry.file_name() != "mod.rs")
                .flat_map(|entry|{
                    if entry.path().is_dir() {
                        return vec![].into_iter();
                    }
                    let lines = into_lines(&entry.path());
                    let vecco = lines.iter().flat_map(|line|re_static_name.captures(line)).map(|cap|cap[2].to_string())
                        .map(|array_name|(entry.path().file_stem().gimme_that(),array_name)).collect::<Vec<_>>();
                    vecco.into_iter()
                })
                // .map(|path|path.path().file_stem().unwrap().gimme_that())
                .collect();

            let mut much_fun: Vec<_> = all_submodules.iter().map(|(sub_mod, array_name)|{
                let fun_name = if sub_mod == &array_name.to_lowercase() {
                    array_name.to_lowercase()
                }else{
                    sub_mod.to_string()+"_"+&array_name.to_lowercase()
                };
                if sub_modules.contains(&(sub_mod.to_string(), array_name.to_string())) {
                    format!(r#"
pub fn {}() -> Option<&'static [&'static str]> {{
    Some(self::{}::{})
}}"#, fun_name, sub_mod, array_name)
                }else{
                    format!(r#"
pub fn {}() -> Option<&'static [&'static str]> {{
    None
}}"#, fun_name)
                }

            }).collect();
            much_fun.sort();

            let mut lines = into_lines(&entry.path());
            //remove old pub fn block
            if let Some(fn_start) = lines.iter().position(|line| line.contains("pub fn")) {
                lines = lines.split_at(fn_start).0.to_vec();
            }

            // if file_name != "date" {
                lines.extend(much_fun);
            // }
            write_lines(lines, &entry.path());

        }
    }
    // println!("{:?}", file_name_to_submodules_always);

    let re = Regex::new(r#"[A-Za-z]*\.([A-Za-z]*)\s*=\s*(".*").*"#).unwrap();

    // convert module["exports"] = [
    // az.title = "Azerbaijani"; to const title
    // az.separator = "Azerbaijani"; to const separator
    for entry in WalkDir::new("../src/locales") {
        let entry = entry.unwrap();
        if entry.path().is_dir() {
            continue;
        }
        let file_name = entry.gimme_filename();

        let result = into_lines(&entry.path()).into_iter().map(|line|{
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
        let file_name = entry.gimme_filename();
        if file_name.ends_with(".js") {
            let entree = entry.path().gimme_that();
            std::fs::rename(entry.path(), entree.replace(".js", ".rs"))?;
        }
    }

    // fix object to arrays -- commented only once
    for entry in WalkDir::new("../src/locales") {
        let entry = entry.unwrap();
        if entry.path().is_dir() {
            continue;
        }
        let file_name = entry.gimme_filename();
        if file_name == "product_name.rs" || file_name == "month.rs" || file_name == "weekday.rs"|| file_name == "title.rs" {

            let lines = into_lines(&entry.path());

            let re = Regex::new(r"\s*],\s*").unwrap();
            let re3 = Regex::new(r"\s*]\s*").unwrap();
            let re2 = Regex::new(r#"\s*"([a-z_]*)"\s*:\s*\[\s*"#).unwrap();
            let re4 = Regex::new(r#"\s*([a-z_]*)\s*:\s*\[\s*"#).unwrap();
            let lines = lines.iter().map(|line|{
                if line == "module[\"exports\"] = {" {
                    return "".to_string();
                }
                if line.contains("pub static") { // already converted
                    return line.to_string();
                }
                if line == "};" {
                    return "".to_string();
                }
                if re.is_match(line) {
                    return "];".to_string();
                }
                if re3.is_match(line) {
                    return "];".to_string();
                }
                if let Some(pat) = re2.captures(&line) {
                    return format!("pub static {}: &'static [&'static str] = &[ ", &pat[1]);
                }
                if let Some(pat) = re4.captures(&line) {
                    return format!("pub static {}: &'static [&'static str] = &[ ", &pat[1]);
                }
                line.to_string()
            }).collect();

            write_lines(lines, &entry.path());

        }
    }

    for entry in WalkDir::new("../src/locales") {
        let entry = entry.unwrap();
        if entry.path().is_dir() {
            continue;
        }
        let lines = into_lines(&entry.path());
        let lines:Vec<String> = lines.iter().map(|line|{
            line.replace("pub const title", "pub const TITLE")
        }).map(|line|{
            line.replace("pub const separator", "pub const SEPARATOR")
        }).collect();
        write_lines(lines, &entry.path());
    }


    // handle lib files
    for entry in WalkDir::new("../src/") {
        let entry = entry.unwrap();
        if entry.path().is_dir() || entry.depth() >= 2 {
            continue;
        }
        let file_name = entry.gimme_filename();
        println!("{:?}", file_name);
        let re = Regex::new(r"^function ([A-Z][A-Za-z]*).*").unwrap(); // function Address (faker) {
        let re2 = Regex::new(r"^var ([A-Z][A-Za-z]*).*").unwrap(); // var Phone = function (faker) {

        let lines = into_lines(&entry.path());

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


        //Convert to
        // #[derive(Debug, Clone)]
        // pub struct Name<'a> {
        //     faker: &'a Faker,
        // }
        // impl<'a> Name<'a> {
        //     pub fn new(faker: &'a Faker) -> Self {
        //         Name { faker }
        //     }
        // }

        let lines: Vec<String> = lines.iter().flat_map(|line|{
            if line.trim().starts_with("fn new() -> Self {"){
                return vec!["    pub fn new(faker: &'a Faker) -> Self {".to_string(), "        Name { faker }".to_string()];
            }
            if line.trim().starts_with("struct") {
                let mut line = line.to_string();
                line.insert_str(0, "pub ");
                line = line.replace("{", "<'a> {");
                return vec!["#[derive(Debug, Clone)]".to_string(), line, "    faker: &'a Faker,".to_string()];
            }
            vec![line.to_string()]
        }).collect();


        //this.zipCode = function(format) {  ->  fn zipCode(&self, ..param:String)
        let re = Regex::new(r"^\s*(this|self)\.([A-Za-z]*)\s*=\s*function\s*[A-Za-z]*\s*\(([A-Za-z,\s]*)\)*.").unwrap(); // var Phone = function (faker) {
        let lines: Vec<String> = lines.iter().flat_map(|line|{
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


        let re_def_name = Regex::new(r#"(.*?)typeof\s*faker\.definitions\.([A-Za-z\._]*)(\s*!==\s*"undefined")"#).unwrap();
        let lines:Vec<String> = lines.iter().map(|line|{
            re_def_name.replace_all(line, |caps: &regex::Captures| {
                caps[1].to_string() + "self.faker."+ &caps[2].replace(".", "_")+ ".is_some()"
            }).to_string()
        }).collect();


        //GENERIC
        //typeof useAbbr === 'undefined'
        let re = Regex::new(r#"(.*?)typeof\s*([0-9A-Za-z\._\(\)\[\]]*)\s*(==|===|!==)\s*["']undefined["'](.*)"#).unwrap();
        let lines:Vec<String> = lines.iter().map(|line|{
            re.replace_all(line, |caps: &regex::Captures| {
                let suffix = if  &caps[3] == "!=="{
                    ".is_some()"
                }else{
                    ".is_none()"
                };
                caps[1].to_string() + &caps[2] + suffix + &caps[4]
            }).to_string()
        }).collect();

        //var to let
        let lines:Vec<String> = lines.iter().map(|line|{
            line.replace("var", "let")
        }).map(|line|{
            line.replace("===", "==")
        }).collect();
        //GENERIC END


        //eg. convert faker.definitions.address.postcode to self.faker.address_postcode()
        let re_def_name = Regex::new(r#"(.*?)faker\.definitions\.([A-Za-z\._]*)(.*)"#).unwrap();
        let lines:Vec<String> = lines.iter().map(|line|{
            re_def_name.replace_all(line, |caps: &regex::Captures| {
                caps[1].to_string() + "self.faker."+ &caps[2].replace(".", "_")+ "()"+ &caps[3]
            }).to_string()
        }).collect();

        let lines:Vec<String> = lines.into_iter().map(|line|{
            line.replace("faker.random.arrayElement(", "thread_rng().choose(")
        }).collect();

        let lines:Vec<String> = lines.into_iter().map(|mut line|{
            if let Some(pos) = line.find_end("thread_rng().choose(") {
                if let Some(end_braces) = find_matching_braces(&line[pos..], '(', ')') {
                    if !line[pos+end_braces ..].starts_with(".unwrap()") && !line[pos+end_braces ..].starts_with(".cloned()") {
                        line.insert_str(pos+end_braces, ".unwrap()");
                    }
                }
            }
            line
        }).collect();


        let mut lines:Vec<String> = lines.into_iter().filter(|el|!el.contains("use rand")).collect();
        lines.insert(0, "use rand::{thread_rng, Rng};".to_string());

        // println!("result {:?}", result);

        write_lines(lines, &entry.path());

        if file_name.ends_with(".js") {
            let entree = entry.path().gimme_that();
            let entree = entree.replace("index.js", "mod.rs");
            let entree = entree.replace(".js", ".rs");
            std::fs::rename(entry.path(), entree)?;
        }
    }


    Ok(())
}

fn into_lines(path: &Path) -> Vec<String> {
    BufReader::new(File::open(path).expect(&format!("{:?}", path))).lines().map(|line|line.expect(&format!("{:?}", path))).collect()
}

fn write_lines(lines: Vec<String>, path: &Path) {
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