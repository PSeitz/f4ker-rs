mod product_name;
mod color;
mod department;

pub fn product_name_material() -> Option<&'static [&'static str]> {
    Some(self::product_name::MATERIAL)
}

pub fn department() -> Option<&'static [&'static str]> {
    Some(self::department::DEPARTMENT)
}

pub fn product_name_adjective() -> Option<&'static [&'static str]> {
    Some(self::product_name::ADJECTIVE)
}

pub fn product_name_product() -> Option<&'static [&'static str]> {
    Some(self::product_name::PRODUCT)
}

pub fn color() -> Option<&'static [&'static str]> {
    Some(self::color::COLOR)
}
