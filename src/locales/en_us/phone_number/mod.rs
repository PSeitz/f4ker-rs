mod area_code;
mod exchange_code;

pub fn area_code() -> Option<&'static [&'static str]> {
    Some(self::area_code::AREA_CODE)
}

pub fn exchange_code() -> Option<&'static [&'static str]> {
    Some(self::exchange_code::EXCHANGE_CODE)
}

pub fn formats() -> Option<&'static [&'static str]> {
    None
}
