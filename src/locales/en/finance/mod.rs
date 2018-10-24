mod account_type;
mod credit_card;
mod currency;
mod transaction_type;

pub fn account_type() -> Option<&'static [&'static str]> {
    Some(self::account_type::ACCOUNT_TYPE)
}

pub fn transaction_type() -> Option<&'static [&'static str]> {
    Some(self::transaction_type::TRANSACTION_TYPE)
}
