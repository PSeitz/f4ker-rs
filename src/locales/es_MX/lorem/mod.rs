mod words;

pub fn supplemental() -> Option<&'static [&'static str]> {
    None
}

pub fn words() -> Option<&'static [&'static str]> {
    Some(self::words::WORDS)
}
