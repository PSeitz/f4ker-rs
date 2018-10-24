mod street_address;
mod secondary_address;
mod city_prefix;
mod postcode;
mod direction;
mod time_zone;
mod city;
mod default_country;
mod street_name;
mod building_number;
mod state_abbr;
mod state;
mod postcode_by_state;
mod direction_abbr;
mod country_code;
mod county;
mod country;
mod city_suffix;
mod street_suffix;

pub fn street_name() -> Option<&'static [&'static str]> {
    Some(self::street_name::STREET_NAME)
}

pub fn street_root() -> Option<&'static [&'static str]> {
    None
}

pub fn city_suffix() -> Option<&'static [&'static str]> {
    Some(self::city_suffix::CITY_SUFFIX)
}

pub fn city_root() -> Option<&'static [&'static str]> {
    None
}

pub fn country_code() -> Option<&'static [&'static str]> {
    Some(self::country_code::COUNTRY_CODE)
}

pub fn street_title() -> Option<&'static [&'static str]> {
    None
}

pub fn street() -> Option<&'static [&'static str]> {
    None
}

pub fn city() -> Option<&'static [&'static str]> {
    Some(self::city::CITY)
}

pub fn postcode() -> Option<&'static [&'static str]> {
    Some(self::postcode::POSTCODE)
}

pub fn street_address() -> Option<&'static [&'static str]> {
    Some(self::street_address::STREET_ADDRESS)
}

pub fn uk_country() -> Option<&'static [&'static str]> {
    None
}

pub fn common_street_suffix() -> Option<&'static [&'static str]> {
    None
}

pub fn time_zone() -> Option<&'static [&'static str]> {
    Some(self::time_zone::TIME_ZONE)
}

pub fn secondary_address() -> Option<&'static [&'static str]> {
    Some(self::secondary_address::SECONDARY_ADDRESS)
}

pub fn streets() -> Option<&'static [&'static str]> {
    None
}

pub fn province() -> Option<&'static [&'static str]> {
    None
}

pub fn city_name() -> Option<&'static [&'static str]> {
    None
}

pub fn direction_abbr() -> Option<&'static [&'static str]> {
    Some(self::direction_abbr::DIRECTION_ABBR)
}

pub fn street_prefix() -> Option<&'static [&'static str]> {
    None
}

pub fn direction() -> Option<&'static [&'static str]> {
    Some(self::direction::DIRECTION)
}

pub fn postcode_by_state() -> Option<&'static [&'static str]> {
    Some(self::postcode_by_state::POSTCODE_BY_STATE)
}

pub fn building_number() -> Option<&'static [&'static str]> {
    Some(self::building_number::BUILDING_NUMBER)
}

pub fn street_suffix() -> Option<&'static [&'static str]> {
    Some(self::street_suffix::STREET_SUFFIX)
}

pub fn county() -> Option<&'static [&'static str]> {
    Some(self::county::COUNTY)
}

pub fn state_abbr() -> Option<&'static [&'static str]> {
    Some(self::state_abbr::STATE_ABBR)
}

pub fn city_prefix() -> Option<&'static [&'static str]> {
    Some(self::city_prefix::CITY_PREFIX)
}

pub fn region() -> Option<&'static [&'static str]> {
    None
}

pub fn country() -> Option<&'static [&'static str]> {
    Some(self::country::COUNTRY)
}

pub fn state() -> Option<&'static [&'static str]> {
    Some(self::state::STATE)
}

pub fn default_country() -> Option<&'static [&'static str]> {
    Some(self::default_country::DEFAULT_COUNTRY)
}
