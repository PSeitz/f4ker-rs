mod avatar_uri;
mod domain_suffix;
mod example_email;
mod free_email;

pub fn avatar_uri() -> Option<&'static [&'static str]> {
    Some(self::avatar_uri::AVATAR_URI)
}

pub fn domain_suffix() -> Option<&'static [&'static str]> {
    Some(self::domain_suffix::DOMAIN_SUFFIX)
}

pub fn example_email() -> Option<&'static [&'static str]> {
    Some(self::example_email::EXAMPLE_EMAIL)
}

pub fn free_email() -> Option<&'static [&'static str]> {
    Some(self::free_email::FREE_EMAIL)
}
