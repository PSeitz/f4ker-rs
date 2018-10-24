mod collation;
// mod type;
mod column;
mod engine;

pub fn collation() -> Option<&'static [&'static str]> {
    Some(self::collation::COLLATION)
}

pub fn column() -> Option<&'static [&'static str]> {
    Some(self::column::COLUMN)
}

// pub fn type() -> Option<&'static [&'static str]> {
//     Some(self::type::TYPE)
// }

pub fn engine() -> Option<&'static [&'static str]> {
    Some(self::engine::ENGINE)
}
