mod transaction_type;
mod account_type;
mod currency;

pub fn transaction_type() -> Option<&'static [&'static str]> {
    Some(self::transaction_type::TRANSACTION_TYPE)
}

pub fn account_type() -> Option<&'static [&'static str]> {
    Some(self::account_type::ACCOUNT_TYPE)
}
