mod first_name;
mod last_name;
mod name;
mod prefix;

pub fn female_first_name() -> Option<&'static [&'static str]> {
    None
}

pub fn female_last_name() -> Option<&'static [&'static str]> {
    None
}

pub fn female_middle_name() -> Option<&'static [&'static str]> {
    None
}

pub fn female_prefix() -> Option<&'static [&'static str]> {
    None
}

pub fn female_title() -> Option<&'static [&'static str]> {
    None
}

pub fn feminine_name() -> Option<&'static [&'static str]> {
    None
}

pub fn first_name() -> Option<&'static [&'static str]> {
    Some(self::first_name::FIRST_NAME)
}

pub fn first_name_men() -> Option<&'static [&'static str]> {
    None
}

pub fn first_name_women() -> Option<&'static [&'static str]> {
    None
}

pub fn gender() -> Option<&'static [&'static str]> {
    None
}

pub fn last_name() -> Option<&'static [&'static str]> {
    Some(self::last_name::LAST_NAME)
}

pub fn male_first_name() -> Option<&'static [&'static str]> {
    None
}

pub fn male_last_name() -> Option<&'static [&'static str]> {
    None
}

pub fn male_middle_name() -> Option<&'static [&'static str]> {
    None
}

pub fn male_prefix() -> Option<&'static [&'static str]> {
    None
}

pub fn male_title() -> Option<&'static [&'static str]> {
    None
}

pub fn masculine_name() -> Option<&'static [&'static str]> {
    None
}

pub fn name() -> Option<&'static [&'static str]> {
    Some(self::name::NAME)
}

pub fn nobility_title_prefix() -> Option<&'static [&'static str]> {
    None
}

pub fn ocker_first_name() -> Option<&'static [&'static str]> {
    None
}

pub fn prefix() -> Option<&'static [&'static str]> {
    Some(self::prefix::PREFIX)
}

pub fn suffix() -> Option<&'static [&'static str]> {
    None
}

pub fn title_descriptor() -> Option<&'static [&'static str]> {
    None
}

pub fn title_job() -> Option<&'static [&'static str]> {
    None
}

pub fn title_level() -> Option<&'static [&'static str]> {
    None
}

pub fn tussenvoegsel() -> Option<&'static [&'static str]> {
    None
}
