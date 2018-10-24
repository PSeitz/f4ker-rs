mod suffix;
mod name;

pub fn name() -> Option<&'static [&'static str]> {
    Some(self::name::NAME)
}

pub fn suffix() -> Option<&'static [&'static str]> {
    Some(self::suffix::SUFFIX)
}

pub fn creature() -> Option<&'static [&'static str]> {
    None
}
