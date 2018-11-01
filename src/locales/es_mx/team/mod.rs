mod creature;
mod name;

pub fn creature() -> Option<&'static [&'static str]> {
    Some(self::creature::CREATURE)
}

pub fn name() -> Option<&'static [&'static str]> {
    Some(self::name::NAME)
}

pub fn suffix() -> Option<&'static [&'static str]> {
    None
}
