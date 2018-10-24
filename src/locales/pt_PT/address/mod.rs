mod building_number;
mod city;
mod city_name;
mod city_prefix;
mod city_suffix;
mod country;
mod default_country;
mod direction;
mod postcode;
mod street_address;
mod street_name;
mod street_prefix;

pub fn building_number() -> Option<&'static [&'static str]> {
    Some(self::building_number::BUILDING_NUMBER)
}

pub fn city() -> Option<&'static [&'static str]> {
    Some(self::city::CITY)
}

pub fn city_name() -> Option<&'static [&'static str]> {
    Some(self::city_name::CITY_NAME)
}

pub fn city_prefix() -> Option<&'static [&'static str]> {
    Some(self::city_prefix::CITY_PREFIX)
}

pub fn city_root() -> Option<&'static [&'static str]> {
    None
}

pub fn city_suffix() -> Option<&'static [&'static str]> {
    Some(self::city_suffix::CITY_SUFFIX)
}

pub fn common_street_suffix() -> Option<&'static [&'static str]> {
    None
}

pub fn country() -> Option<&'static [&'static str]> {
    Some(self::country::COUNTRY)
}

pub fn country_code() -> Option<&'static [&'static str]> {
    None
}

pub fn county() -> Option<&'static [&'static str]> {
    None
}

pub fn default_country() -> Option<&'static [&'static str]> {
    Some(self::default_country::DEFAULT_COUNTRY)
}

pub fn direction() -> Option<&'static [&'static str]> {
    Some(self::direction::DIRECTION)
}

pub fn direction_abbr() -> Option<&'static [&'static str]> {
    None
}

pub fn postcode() -> Option<&'static [&'static str]> {
    Some(self::postcode::POSTCODE)
}

pub fn postcode_by_state() -> Option<&'static [&'static str]> {
    None
}

pub fn province() -> Option<&'static [&'static str]> {
    None
}

pub fn region() -> Option<&'static [&'static str]> {
    None
}

pub fn secondary_address() -> Option<&'static [&'static str]> {
    None
}

pub fn state() -> Option<&'static [&'static str]> {
    None
}

pub fn state_abbr() -> Option<&'static [&'static str]> {
    None
}

pub fn street() -> Option<&'static [&'static str]> {
    None
}

pub fn street_address() -> Option<&'static [&'static str]> {
    Some(self::street_address::STREET_ADDRESS)
}

pub fn street_name() -> Option<&'static [&'static str]> {
    Some(self::street_name::STREET_NAME)
}

pub fn street_prefix() -> Option<&'static [&'static str]> {
    Some(self::street_prefix::STREET_PREFIX)
}

pub fn street_root() -> Option<&'static [&'static str]> {
    None
}

pub fn street_suffix() -> Option<&'static [&'static str]> {
    None
}

pub fn street_title() -> Option<&'static [&'static str]> {
    None
}

pub fn streets() -> Option<&'static [&'static str]> {
    None
}

pub fn time_zone() -> Option<&'static [&'static str]> {
    None
}

pub fn uk_country() -> Option<&'static [&'static str]> {
    None
}
