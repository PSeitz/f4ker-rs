mod discover;
mod maestro;
mod visa;
mod american_express;
mod mastercard;

pub fn maestro() -> Option<&'static [&'static str]> {
    Some(self::maestro::MAESTRO)
}

pub fn mastercard() -> Option<&'static [&'static str]> {
    Some(self::mastercard::MASTERCARD)
}

pub fn visa() -> Option<&'static [&'static str]> {
    Some(self::visa::VISA)
}

pub fn discover() -> Option<&'static [&'static str]> {
    Some(self::discover::DISCOVER)
}

pub fn american_express() -> Option<&'static [&'static str]> {
    Some(self::american_express::AMERICAN_EXPRESS)
}
