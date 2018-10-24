mod supplemental;
mod words;

pub fn supplemental() -> Option<&'static [&'static str]> {
    Some(self::supplemental::SUPPLEMENTAL)
}

pub fn words() -> Option<&'static [&'static str]> {
    Some(self::words::WORDS)
}
