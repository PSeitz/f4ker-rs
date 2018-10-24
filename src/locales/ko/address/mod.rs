mod postcode;
mod street_root;
mod city;
mod street_name;
mod state_abbr;
mod state;
mod city_name;
mod city_suffix;
mod street_suffix;

pub fn street_name() -> Option<&'static [&'static str]> {
    Some(self::street_name::STREET_NAME)
}

pub fn street_root() -> Option<&'static [&'static str]> {
    Some(self::street_root::STREET_ROOT)
}

pub fn city_suffix() -> Option<&'static [&'static str]> {
    Some(self::city_suffix::CITY_SUFFIX)
}

pub fn city_root() -> Option<&'static [&'static str]> {
    None
}

pub fn country_code() -> Option<&'static [&'static str]> {
    None
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
    None
}

pub fn uk_country() -> Option<&'static [&'static str]> {
    None
}

pub fn common_street_suffix() -> Option<&'static [&'static str]> {
    None
}

pub fn time_zone() -> Option<&'static [&'static str]> {
    None
}

pub fn secondary_address() -> Option<&'static [&'static str]> {
    None
}

pub fn streets() -> Option<&'static [&'static str]> {
    None
}

pub fn province() -> Option<&'static [&'static str]> {
    None
}

pub fn city_name() -> Option<&'static [&'static str]> {
    Some(self::city_name::CITY_NAME)
}

pub fn direction_abbr() -> Option<&'static [&'static str]> {
    None
}

pub fn street_prefix() -> Option<&'static [&'static str]> {
    None
}

pub fn direction() -> Option<&'static [&'static str]> {
    None
}

pub fn postcode_by_state() -> Option<&'static [&'static str]> {
    None
}

pub fn building_number() -> Option<&'static [&'static str]> {
    None
}

pub fn street_suffix() -> Option<&'static [&'static str]> {
    Some(self::street_suffix::STREET_SUFFIX)
}

pub fn county() -> Option<&'static [&'static str]> {
    None
}

pub fn state_abbr() -> Option<&'static [&'static str]> {
    Some(self::state_abbr::STATE_ABBR)
}

pub fn city_prefix() -> Option<&'static [&'static str]> {
    None
}

pub fn region() -> Option<&'static [&'static str]> {
    None
}

pub fn country() -> Option<&'static [&'static str]> {
    None
}

pub fn state() -> Option<&'static [&'static str]> {
    Some(self::state::STATE)
}

pub fn default_country() -> Option<&'static [&'static str]> {
    None
}
