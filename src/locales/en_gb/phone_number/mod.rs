mod formats;

pub fn area_code() -> Option<&'static [&'static str]> {
    None
}

pub fn exchange_code() -> Option<&'static [&'static str]> {
    None
}

pub fn formats() -> Option<&'static [&'static str]> {
    Some(self::formats::FORMATS)
}
