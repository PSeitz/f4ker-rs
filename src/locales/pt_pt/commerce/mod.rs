mod color;
mod department;
mod product_name;

pub fn color() -> Option<&'static [&'static str]> {
    Some(self::color::COLOR)
}

pub fn department() -> Option<&'static [&'static str]> {
    Some(self::department::DEPARTMENT)
}

pub fn product_name_adjective() -> Option<&'static [&'static str]> {
    Some(self::product_name::ADJECTIVE)
}

pub fn product_name_material() -> Option<&'static [&'static str]> {
    Some(self::product_name::MATERIAL)
}

pub fn product_name_product() -> Option<&'static [&'static str]> {
    Some(self::product_name::PRODUCT)
}
