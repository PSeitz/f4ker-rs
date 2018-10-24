mod author;
mod version;
mod name;

pub fn author() -> Option<&'static [&'static str]> {
    Some(self::author::AUTHOR)
}

pub fn name() -> Option<&'static [&'static str]> {
    Some(self::name::NAME)
}

pub fn version() -> Option<&'static [&'static str]> {
    Some(self::version::VERSION)
}
