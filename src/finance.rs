use rand::{thread_rng, Rng};
use crate::faker::Faker;
use crate::*;
#[derive(Debug, Clone)]
pub struct Finance <'a> {
    faker: &'a Faker,
{
}
/**
 * @namespace faker.finance
 */
impl Finance {
    pub fn new(faker: &'a Faker) -> Self {
        Self { faker }

    }
  let ibanLib = require("./iban");
      self = this;

  /**
   * account
   *
   * @method faker.finance.account
   * @param {number} length
   */
    pub fn account(&self, length: &str) -> String {

      length = length || 8;

      let template = '';

      for (let i = 0; i < length; i++) {
          template = template + '#';
      }
      length = null;
      return Helpers.replace_symbol_with_number(template);
  };

  /**
   * accountName
   *
   * @method faker.finance.accountName
   */
    pub fn account_name(&self) -> String {

      return [Helpers.randomize(self.faker.finance_account_type()), 'Account'].join(' ');
  };

  /**
   * routingNumber
   *
   * @method faker.finance.routingNumber
   */
    pub fn routing_number(&self) -> String {

      let routingNumber = Helpers.replace_symbol_with_number('########');

      // Modules 10 straight summation.
      let sum = 0;

      for (let i = 0; i < routingNumber.length; i += 3) {
        sum += Number(routingNumber[i]) * 3;
        sum += Number(routingNumber[i + 1]) * 7;
        sum += Number(routingNumber[i + 2]) || 0;
      }

      return routingNumber + (Math.ceil(sum / 10) * 10 - sum);
  }

  /**
   * mask
   *
   * @method faker.finance.mask
   * @param {number} length
   * @param {boolean} parens
   * @param {boolean} ellipsis
   */
    pub fn mask(&self, length: &str,  parens: &str,  ellipsis: &str) -> String {

      //set defaults
      length = (length == 0 || !length || length.is_none()) ? 4 : length;
      parens = (parens == null) ? true : parens;
      ellipsis = (ellipsis == null) ? true : ellipsis;

      //create a template for length
      let template = '';

      for (let i = 0; i < length; i++) {
          template = template + '#';
      }

      //prefix with ellipsis
      template = (ellipsis) ? ['...', template].join('') : template;

      template = (parens) ? ['(', template, ')'].join('') : template;

      //generate random numbers
      template = Helpers.replace_symbol_with_number(template);

      return template;
  };

  //min and max take in minimum and maximum amounts, dec is the decimal place you want rounded to, symbol is $, €, £, etc
  //NOTE: this returns a string representation of the value, if you want a number use parseFloat and no symbol

  /**
   * amount
   *
   * @method faker.finance.amount
   * @param {number} min
   * @param {number} max
   * @param {number} dec
   * @param {string} symbol
   *
   * @return {string}
   */
    pub fn amount(&self, min: &str,  max: &str,  dec: &str,  symbol: &str) -> String {

      min = min || 0;
      max = max || 1000;
      dec = dec == undefined ? 2 : dec;
      symbol = symbol || '';
      let randValue = faker.random.number({ max: max, min: min, precision: Math.pow(10, -dec) });

      return symbol + randValue.toFixed(dec);
  };

  /**
   * transactionType
   *
   * @method faker.finance.transactionType
   */
    pub fn transaction_type(&self) -> String {
      return Helpers.randomize(self.faker.finance_transaction_type());
  };

  /**
   * currencyCode
   *
   * @method faker.finance.currencyCode
   */
    pub fn currency_code(&self) -> String {
      return faker.random.objectElement(self.faker.finance_currency())['code'];
  };

  /**
   * currencyName
   *
   * @method faker.finance.currencyName
   */
    pub fn currency_name(&self) -> String {
      return faker.random.objectElement(self.faker.finance_currency(), 'key');
  };

  /**
   * currencySymbol
   *
   * @method faker.finance.currencySymbol
   */
    pub fn currency_symbol(&self) -> String {
      let symbol;

      while (!symbol) {
          symbol = faker.random.objectElement(self.faker.finance_currency())['symbol'];
      }
      return symbol;
  };

  /**
   * bitcoinAddress
   *
   * @method  faker.finance.bitcoinAddress
   */
    pub fn bitcoin_address(&self) -> String {
    let addressLength = faker.random.number({ min: 25, max: 34 });

    let address = thread_rng().choose(['1', '3']).unwrap();

    for (let i = 0; i < addressLength - 1; i++)
      address += thread_rng().choose('123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ'.split('')).unwrap();

    return address;
  }

  /**
   * Credit card number
   * @method faker.finance.creditCardNumber
   * @param {string} provider | scheme
  */
    pub fn credit_card_number(&self, provider: &str) -> String {
    provider = provider || "";
    let format, formats;
    let localeFormat = self.faker.finance_credit_card();
    if (provider in localeFormat) {
      formats = localeFormat[provider]; // there chould be multiple formats
      if (typeof formats == "string") {
        format = formats;
      } else {
        format = thread_rng().choose(formats).unwrap();
      }
    } else if (provider.match(/#/)) { // The user chose an optional scheme
      format = provider;
    } else { // Choose a random provider
      if (typeof localeFormat == 'string') {
        format = localeFormat;
      } else if( typeof localeFormat == "object") {
        // Credit cards are in a object structure
        formats = faker.random.objectElement(localeFormat, "value"); // There chould be multiple formats
        if (typeof formats == "string") {
          format = formats;
        } else {
          format = thread_rng().choose(formats).unwrap();
        }
      }
    }
    format = format.replace(/\//g,"")
    return Helpers.replaceCreditCardSymbols(format);
  };
  /**
   * Credit card CVV
   * @method faker.finance.creditCardNumber
  */
    pub fn credit_card_cvv(&self) -> String {
    let cvv = "";
    for (let i = 0; i < 3; i++) {
      cvv += faker.random.number({max:9}).toString();
    }
    return cvv;
  };

  /**
   * ethereumAddress
   *
   * @method  faker.finance.ethereumAddress
   */
    pub fn ethereum_address(&self) -> String {
    let address = faker.random.hexaDecimal(40);

    return address;
  };

  /**
   * iban
   *
   * @method  faker.finance.iban
   */
    pub fn iban(&self, formatted: &str) -> String {
      let ibanFormat = thread_rng().choose(ibanLib.formats).unwrap();
      let s = "";
      let count = 0;
      for (let b = 0; b < ibanFormat.bban.length; b++) {
          let bban = ibanFormat.bban[b];
          let c = bban.count;
          count += bban.count;
          while (c > 0) {
              if (bban.type == "a") {
                  s += thread_rng().choose(ibanLib.alpha).unwrap();
              } else if (bban.type == "c") {
                  if (faker.random.number(100) < 80) {
                      s += faker.random.number(9);
                  } else {
                      s += thread_rng().choose(ibanLib.alpha).unwrap();
                  }
              } else {
                  if (c >= 3 && faker.random.number(100) < 30) {
                      if (faker.random.boolean()) {
                          s += thread_rng().choose(ibanLib.pattern100).unwrap();
                          c -= 2;
                      } else {
                          s += thread_rng().choose(ibanLib.pattern10).unwrap();
                          c--;
                      }
                  } else {
                      s += faker.random.number(9);
                  }
              }
              c--;
          }
          s = s.substring(0, count);
      }
      let checksum = 98 - ibanLib.mod97(ibanLib.toDigitString(s + ibanFormat.country + "00"));
      if (checksum < 10) {
          checksum = "0" + checksum;
      }
      let iban = ibanFormat.country + checksum + s;
      return formatted ? iban.match(/.{1,4}/g).join(" ") : iban;
  };

  /**
   * bic
   *
   * @method  faker.finance.bic
   */
    pub fn bic(&self) -> String {
      let vowels = ["A", "E", "I", "O", "U"];
      let prob = faker.random.number(100);
      return Helpers.replace_symbols("???") +
          thread_rng().choose(vowels).unwrap() +
          thread_rng().choose(ibanLib.iso3166).unwrap() +
          Helpers.replace_symbols("?") + "1" +
          (prob < 10 ?
              Helpers.replace_symbols("?" + thread_rng().choose(vowels).unwrap() + "?") :
          prob < 40 ?
              Helpers.replace_symbols("###") : "");
  };
};

module['exports'] = Finance;
