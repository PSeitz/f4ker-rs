mod common_cell_prefix;
mod formats;

pub fn formats() -> Option<&'static [&'static str]> {
    Some(self::formats::FORMATS)
}

pub fn common_cell_prefix() -> Option<&'static [&'static str]> {
    Some(self::common_cell_prefix::COMMON_CELL_PREFIX)
}
