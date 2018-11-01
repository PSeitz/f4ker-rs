mod adjective;
mod bs_adjective;
mod bs_noun;
mod bs_verb;
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
    Some(self::bs_adjective::BS_ADJECTIVE)
}

pub fn bs_noun() -> Option<&'static [&'static str]> {
    Some(self::bs_noun::BS_NOUN)
}

pub fn bs_verb() -> Option<&'static [&'static str]> {
    Some(self::bs_verb::BS_VERB)
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
