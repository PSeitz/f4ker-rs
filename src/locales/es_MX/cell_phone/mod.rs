mod formats;

pub fn common_cell_prefix() -> Option<&'static [&'static str]> {
    None
}

pub fn formats() -> Option<&'static [&'static str]> {
    Some(self::formats::FORMATS)
}
