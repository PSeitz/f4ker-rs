// #![feature(test)]
use rand::{thread_rng, Rng};
use std::char;

mod faker;
pub mod locales;
mod name;


fn replace_symbol_with_number(templ: &str) -> String {

    let templ = templ
        .chars()
        .map(|cha| {
            if cha == '#' {
                char::from_digit(thread_rng().gen_range::<u32>(0, 9), 10).unwrap()
            } else if cha == '!' {
                char::from_digit(thread_rng().gen_range::<u32>(2, 9), 10).unwrap()
            } else {
                cha
            }
        })
        .collect();
    templ
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_interpol() {
        // println!(
        //     "{}",
        //     interpol("{{name.lastName}}, {{name.firstName}} - {{name.suffix}}")
        // );
        println!("{}", replace_symbol_with_number("AB ## AB"));
    }
}
