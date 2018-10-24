mod free_email;
mod domain_suffix;

pub fn example_email() -> Option<&'static [&'static str]> {
    None
}

pub fn avatar_uri() -> Option<&'static [&'static str]> {
    None
}

pub fn domain_suffix() -> Option<&'static [&'static str]> {
    Some(self::domain_suffix::DOMAIN_SUFFIX)
}

pub fn free_email() -> Option<&'static [&'static str]> {
    Some(self::free_email::FREE_EMAIL)
}
