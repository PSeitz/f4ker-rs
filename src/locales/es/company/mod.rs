mod adjective;
mod descriptor;
mod name;
mod noun;
mod suffix;

pub fn adjective() -> Option<&'static [&'static str]> {
    Some(self::adjective::ADJECTIVE)
}

pub fn adjetive() -> Option<&'static [&'static str]> {
    None
}

pub fn bs_adjective() -> Option<&'static [&'static str]> {
    None
}

pub fn bs_noun() -> Option<&'static [&'static str]> {
    None
}

pub fn bs_verb() -> Option<&'static [&'static str]> {
    None
}

pub fn descriptor() -> Option<&'static [&'static str]> {
    Some(self::descriptor::DESCRIPTOR)
}

pub fn legal_form() -> Option<&'static [&'static str]> {
    None
}

pub fn name() -> Option<&'static [&'static str]> {
    Some(self::name::NAME)
}

pub fn noun() -> Option<&'static [&'static str]> {
    Some(self::noun::NOUN)
}

pub fn prefix() -> Option<&'static [&'static str]> {
    None
}

pub fn suffix() -> Option<&'static [&'static str]> {
    Some(self::suffix::SUFFIX)
}
