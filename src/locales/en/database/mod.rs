mod collation;
mod column;
mod engine;
// mod type;

pub fn collation() -> Option<&'static [&'static str]> {
    Some(self::collation::COLLATION)
}

pub fn column() -> Option<&'static [&'static str]> {
    Some(self::column::COLUMN)
}

pub fn engine() -> Option<&'static [&'static str]> {
    Some(self::engine::ENGINE)
}

// pub fn type() -> Option<&'static [&'static str]> {
//     Some(self::type::TYPE)
// }
