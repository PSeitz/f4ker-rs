mod phrase;
mod verb;
mod abbreviation;
mod adjective;
mod ingverb;
mod noun;

pub fn abbreviation() -> Option<&'static [&'static str]> {
    Some(self::abbreviation::ABBREVIATION)
}

pub fn ingverb() -> Option<&'static [&'static str]> {
    Some(self::ingverb::INGVERB)
}

pub fn verb() -> Option<&'static [&'static str]> {
    Some(self::verb::VERB)
}

pub fn noun() -> Option<&'static [&'static str]> {
    Some(self::noun::NOUN)
}

pub fn phrase() -> Option<&'static [&'static str]> {
    Some(self::phrase::PHRASE)
}

pub fn adjective() -> Option<&'static [&'static str]> {
    Some(self::adjective::ADJECTIVE)
}
