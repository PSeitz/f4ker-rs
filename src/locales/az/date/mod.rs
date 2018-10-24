mod weekday;
mod month;

pub fn weekday_abbr_context() -> Option<&'static [&'static str]> {
    Some(self::weekday::ABBR_CONTEXT)
}

pub fn month_wide() -> Option<&'static [&'static str]> {
    Some(self::month::WIDE)
}

pub fn weekday_wide_context() -> Option<&'static [&'static str]> {
    Some(self::weekday::WIDE_CONTEXT)
}

pub fn weekday_abbr() -> Option<&'static [&'static str]> {
    Some(self::weekday::ABBR)
}

pub fn weekday_wide() -> Option<&'static [&'static str]> {
    Some(self::weekday::WIDE)
}

pub fn month_abbr_context() -> Option<&'static [&'static str]> {
    Some(self::month::ABBR_CONTEXT)
}

pub fn month_wide_context() -> Option<&'static [&'static str]> {
    Some(self::month::WIDE_CONTEXT)
}

pub fn month_abbr() -> Option<&'static [&'static str]> {
    Some(self::month::ABBR)
}
