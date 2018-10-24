mod color;
mod department;

pub fn color() -> Option<&'static [&'static str]> {
    Some(self::color::COLOR)
}

pub fn department() -> Option<&'static [&'static str]> {
    Some(self::department::DEPARTMENT)
}

pub fn product_name_adjective() -> Option<&'static [&'static str]> {
    None
}

pub fn product_name_material() -> Option<&'static [&'static str]> {
    None
}

pub fn product_name_product() -> Option<&'static [&'static str]> {
    None
}
