mod bs_noun;
mod descriptor;
mod adjective;
mod suffix;
mod name;
mod bs_verb;
mod noun;

pub fn prefix() -> Option<&'static [&'static str]> {
    None
}

pub fn name() -> Option<&'static [&'static str]> {
    Some(self::name::NAME)
}

pub fn bs_verb() -> Option<&'static [&'static str]> {
    Some(self::bs_verb::BS_VERB)
}

pub fn legal_form() -> Option<&'static [&'static str]> {
    None
}

pub fn bs_noun() -> Option<&'static [&'static str]> {
    Some(self::bs_noun::BS_NOUN)
}

pub fn descriptor() -> Option<&'static [&'static str]> {
    Some(self::descriptor::DESCRIPTOR)
}

pub fn suffix() -> Option<&'static [&'static str]> {
    Some(self::suffix::SUFFIX)
}

pub fn noun() -> Option<&'static [&'static str]> {
    Some(self::noun::NOUN)
}

pub fn adjective() -> Option<&'static [&'static str]> {
    Some(self::adjective::ADJECTIVE)
}

pub fn bs_adjective() -> Option<&'static [&'static str]> {
    None
}

pub fn adjetive() -> Option<&'static [&'static str]> {
    None
}
