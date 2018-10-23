
pub mod locales;
mod faker;
// use a::series::of::nested_modules;


fn interpol(templ:&str) -> String {
    let mut templ = templ.to_string();
    while let Some(start_bytes) = templ.find("{{") {
        if let Some(end) = &templ[start_bytes..].find("}}") {

            match &templ[start_bytes + 2.. start_bytes+end] {
                "name.lastName" => templ.replace_range(start_bytes.. start_bytes+end+2, "Fred"),
                _ => templ.replace_range(start_bytes.. start_bytes+end+2, "TEST"),
            }

            ;
        }
    }

    templ
}


fn replace_symbol_with_number(templ:&str) -> String {

// *
//    * parses string for a symbol and replace it with a random number from 1-10
//    *
//    * @method faker.helpers.replaceSymbolWithNumber
//    * @param {string} string
//    * @param {string} symbol defaults to `"#"`

//   self.replaceSymbolWithNumber = function (string, symbol) {
      // string = string || "";
      // default symbol is '#'
      // if (symbol === undefined) {
      //     symbol = '#';
      // }

    let templ = templ.chars().map(|cha|{
        if cha == '#' {
            'a'
        }else if cha == '!' {
            'a'
        }else{
            cha
        }
    }).collect();


      // var str = '';
      // for (var i = 0; i < string.length; i++) {
      //     if (string.charAt(i) == symbol) {
      //         str += faker.random.number(9);
      //     } else if (string.charAt(i) == "!"){
      //         str += faker.random.number({min: 2, max: 9});
      //     } else {
      //         str += string.charAt(i);
      //     }
      // }
      // return str;
    templ
}


#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_interpol() {
        println!("{}", interpol("{{name.lastName}}, {{name.firstName}} - {{name.suffix}}"));
        println!("{}", replace_symbol_with_number("AB ## AB"));

    }
}
