mod credit_card_types;
mod credit_card_numbers;
mod credit_card_expiry_dates;

pub fn credit_card_expiry_dates() -> Option<&'static [&'static str]> {
    Some(self::credit_card_expiry_dates::CREDIT_CARD_EXPIRY_DATES)
}

pub fn credit_card_types() -> Option<&'static [&'static str]> {
    Some(self::credit_card_types::CREDIT_CARD_TYPES)
}

pub fn credit_card_numbers() -> Option<&'static [&'static str]> {
    Some(self::credit_card_numbers::CREDIT_CARD_NUMBERS)
}
