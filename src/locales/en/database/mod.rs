mod collation;
mod column;
mod data_type;
mod engine;

pub fn collation() -> Option<&'static [&'static str]> {
    Some(self::collation::COLLATION)
}

pub fn column() -> Option<&'static [&'static str]> {
    Some(self::column::COLUMN)
}

pub fn data_type_type() -> Option<&'static [&'static str]> {
    Some(self::data_type::TYPE)
}

pub fn engine() -> Option<&'static [&'static str]> {
    Some(self::engine::ENGINE)
}
