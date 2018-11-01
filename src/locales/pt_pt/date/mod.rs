mod month;
mod weekday;

pub fn month_abbr() -> Option<&'static [&'static str]> {
    Some(self::month::ABBR)
}

pub fn month_abbr_context() -> Option<&'static [&'static str]> {
    None
}

pub fn month_wide() -> Option<&'static [&'static str]> {
    Some(self::month::WIDE)
}

pub fn month_wide_context() -> Option<&'static [&'static str]> {
    None
}

pub fn weekday_abbr() -> Option<&'static [&'static str]> {
    Some(self::weekday::ABBR)
}

pub fn weekday_abbr_context() -> Option<&'static [&'static str]> {
    None
}

pub fn weekday_wide() -> Option<&'static [&'static str]> {
    Some(self::weekday::WIDE)
}

pub fn weekday_wide_context() -> Option<&'static [&'static str]> {
    None
}
