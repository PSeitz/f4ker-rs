/*!
Crate `f4ker` is a library for generating realistic fake data
[`f4ker::Faker`](f4ker/struct.Faker.html) type.
# Installation
Simply add a corresponding entry to your `Cargo.toml` dependency list:
```ignore
[dependencies]
f4ker = "0.2"
```
And add this to your crate root:
```ignore
extern crate faker;
```

```rust
extern crate f4ker;
use f4ker::Faker;
fn main() {
    println!("{:?}", Faker::new().fake("{{name.last_name}}, {{name.first_name}} and {{name.last_name}}"));
}
```
*/

use rand::{thread_rng, Rng};
use std::char;

mod address;
mod company;
mod faker;
pub mod locales;
mod name;
mod random;

pub use self::address::Address;
pub use self::company::Company;
pub use self::faker::Faker;
pub use self::name::Name;
pub use self::random::Random;

#[macro_export]
macro_rules! rand_cloned {
    ($arr:expr) => {
        thread_rng().choose(&$arr).cloned().unwrap()
    };
}

pub fn sample<T>(arr: &[T]) -> &T {
    thread_rng().choose(&arr).unwrap()
}

#[macro_export]
macro_rules! rand {
    ($arr:expr) => {
        thread_rng().choose(&$arr).unwrap()
    };
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

fn random_digit() -> char {
    char::from_digit(thread_rng().gen_range::<u32>(0, 9), 10).unwrap()
}
pub fn replace_symbol_with_number(templ: &str) -> String {
    let templ = templ
        .chars()
        .map(|cha| {
            if cha == '#' {
                random_digit()
            } else if cha == '!' {
                char::from_digit(thread_rng().gen_range::<u32>(2, 9), 10).unwrap()
            } else {
                cha
            }
        })
        .collect();
    templ
}

pub fn replace_symbols(string: &str) -> String {
    let alpha = &[
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V',
        'W', 'X', 'Y', 'Z',
    ];
    string
        .chars()
        .map(|cha| {
            if cha == '#' {
                random_digit()
            } else if cha == '?' {
                *thread_rng().choose(alpha).unwrap()
            } else if cha == '*' {
                if rand::random() {
                    *thread_rng().choose(alpha).unwrap()
                } else {
                    random_digit()
                }
            } else {
                cha
            }
        })
        .collect()
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
