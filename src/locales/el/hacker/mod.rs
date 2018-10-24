mod abbreviation;
mod adjective;
mod noun;
mod verb;

pub fn abbreviation() -> Option<&'static [&'static str]> {
    Some(self::abbreviation::ABBREVIATION)
}

pub fn adjective() -> Option<&'static [&'static str]> {
    Some(self::adjective::ADJECTIVE)
}

pub fn ingverb() -> Option<&'static [&'static str]> {
    None
}

pub fn noun() -> Option<&'static [&'static str]> {
    Some(self::noun::NOUN)
}

pub fn phrase() -> Option<&'static [&'static str]> {
    None
}

pub fn verb() -> Option<&'static [&'static str]> {
    Some(self::verb::VERB)
}
