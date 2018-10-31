use rand::{thread_rng, Rng};
use std::char;

mod faker;
pub mod locales;
mod name;

pub use self::faker::Faker;

#[macro_export]
macro_rules! rand_cloned {
    ($arr:expr) => (thread_rng().choose(&$arr).cloned().unwrap())
}

trait RandArrayStatic {
    fn rand(&self) -> &'static str;
}
impl RandArrayStatic for &'static [&'static str] {
    fn rand(&self) -> &'static str {
        thread_rng().choose(&self).unwrap() //unwrap, because empty arrays are not allowed in locales
    }
}

impl RandArrayStatic for Option<&'static [&'static str]> {
    fn rand(&self) -> &'static str {
        if let Some(arr) = self {
            return thread_rng().choose(&arr).unwrap(); //unwrap, because empty arrays are not allowed in locales
        }
        ""
    }
}

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
