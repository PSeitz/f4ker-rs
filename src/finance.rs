use rand::{thread_rng, Rng};
struct Finance {
{
}
/**
 * @namespace faker.finance
 */
impl Finance {
    fn new() -> Self {

    }
  var ibanLib = require("./iban");
  var Helpers = faker.helpers,
      self = this;

  /**
   * account
   *
   * @method faker.finance.account
   * @param {number} length
   */
fn account(&self, length: &str) -> String {

      length = length || 8;

      var template = '';

      for (var i = 0; i < length; i++) {
          template = template + '#';
      }
      length = null;
      return Helpers.replaceSymbolWithNumber(template);
  };

  /**
   * accountName
   *
   * @method faker.finance.accountName
   */
fn accountName(&self) -> String {

      return [Helpers.randomize(self.faker.finance_account_type()), 'Account'].join(' ');
  };

  /**
   * routingNumber
   *
   * @method faker.finance.routingNumber
   */
fn routingNumber(&self) -> String {

      var routingNumber = Helpers.replaceSymbolWithNumber('########');

      // Modules 10 straight summation.
      var sum = 0;

      for (var i = 0; i < routingNumber.length; i += 3) {
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
fn mask(&self, length: &str,  parens: &str,  ellipsis: &str) -> String {

      //set defaults
      length = (length == 0 || !length || typeof length == 'undefined') ? 4 : length;
      parens = (parens === null) ? true : parens;
      ellipsis = (ellipsis === null) ? true : ellipsis;

      //create a template for length
      var template = '';

      for (var i = 0; i < length; i++) {
          template = template + '#';
      }

      //prefix with ellipsis
      template = (ellipsis) ? ['...', template].join('') : template;

      template = (parens) ? ['(', template, ')'].join('') : template;

      //generate random numbers
      template = Helpers.replaceSymbolWithNumber(template);

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
fn amount(&self, min: &str,  max: &str,  dec: &str,  symbol: &str) -> String {

      min = min || 0;
      max = max || 1000;
      dec = dec === undefined ? 2 : dec;
      symbol = symbol || '';
      var randValue = faker.random.number({ max: max, min: min, precision: Math.pow(10, -dec) });

      return symbol + randValue.toFixed(dec);
  };

  /**
   * transactionType
   *
   * @method faker.finance.transactionType
   */
fn transactionType(&self) -> String {
      return Helpers.randomize(self.faker.finance_transaction_type());
  };

  /**
   * currencyCode
   *
   * @method faker.finance.currencyCode
   */
fn currencyCode(&self) -> String {
      return faker.random.objectElement(self.faker.finance_currency())['code'];
  };

  /**
   * currencyName
   *
   * @method faker.finance.currencyName
   */
fn currencyName(&self) -> String {
      return faker.random.objectElement(self.faker.finance_currency(), 'key');
  };

  /**
   * currencySymbol
   *
   * @method faker.finance.currencySymbol
   */
fn currencySymbol(&self) -> String {
      var symbol;

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
fn bitcoinAddress(&self) -> String {
    var addressLength = faker.random.number({ min: 25, max: 34 });

    var address = thread_rng().choose(['1', '3']);

    for (var i = 0; i < addressLength - 1; i++)
      address += thread_rng().choose('123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ'.split(''));

    return address;
  }

  /**
   * Credit card number
   * @method faker.finance.creditCardNumber
   * @param {string} provider | scheme
  */
fn creditCardNumber(&self, provider: &str) -> String {
    provider = provider || "";
    var format, formats;
    var localeFormat = self.faker.finance_credit_card();
    if (provider in localeFormat) {
      formats = localeFormat[provider]; // there chould be multiple formats
      if (typeof formats === "string") {
        format = formats;
      } else {
        format = thread_rng().choose(formats);
      }
    } else if (provider.match(/#/)) { // The user chose an optional scheme
      format = provider;
    } else { // Choose a random provider
      if (typeof localeFormat === 'string') {
        format = localeFormat;
      } else if( typeof localeFormat === "object") {
        // Credit cards are in a object structure
        formats = faker.random.objectElement(localeFormat, "value"); // There chould be multiple formats
        if (typeof formats === "string") {
          format = formats;
        } else {
          format = thread_rng().choose(formats);
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
fn creditCardCVV(&self) -> String {
    var cvv = "";
    for (var i = 0; i < 3; i++) {
      cvv += faker.random.number({max:9}).toString();
    }
    return cvv;
  };

  /**
   * ethereumAddress
   *
   * @method  faker.finance.ethereumAddress
   */
fn ethereumAddress(&self) -> String {
    var address = faker.random.hexaDecimal(40);

    return address;
  };

  /**
   * iban
   *
   * @method  faker.finance.iban
   */
fn iban(&self, formatted: &str) -> String {
      var ibanFormat = thread_rng().choose(ibanLib.formats);
      var s = "";
      var count = 0;
      for (var b = 0; b < ibanFormat.bban.length; b++) {
          var bban = ibanFormat.bban[b];
          var c = bban.count;
          count += bban.count;
          while (c > 0) {
              if (bban.type == "a") {
                  s += thread_rng().choose(ibanLib.alpha);
              } else if (bban.type == "c") {
                  if (faker.random.number(100) < 80) {
                      s += faker.random.number(9);
                  } else {
                      s += thread_rng().choose(ibanLib.alpha);
                  }
              } else {
                  if (c >= 3 && faker.random.number(100) < 30) {
                      if (faker.random.boolean()) {
                          s += thread_rng().choose(ibanLib.pattern100);
                          c -= 2;
                      } else {
                          s += thread_rng().choose(ibanLib.pattern10);
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
      var checksum = 98 - ibanLib.mod97(ibanLib.toDigitString(s + ibanFormat.country + "00"));
      if (checksum < 10) {
          checksum = "0" + checksum;
      }
      var iban = ibanFormat.country + checksum + s;
      return formatted ? iban.match(/.{1,4}/g).join(" ") : iban;
  };

  /**
   * bic
   *
   * @method  faker.finance.bic
   */
fn bic(&self) -> String {
      var vowels = ["A", "E", "I", "O", "U"];
      var prob = faker.random.number(100);
      return Helpers.replaceSymbols("???") +
          thread_rng().choose(vowels) +
          thread_rng().choose(ibanLib.iso3166) +
          Helpers.replaceSymbols("?") + "1" +
          (prob < 10 ?
              Helpers.replaceSymbols("?" + thread_rng().choose(vowels) + "?") :
          prob < 40 ?
              Helpers.replaceSymbols("###") : "");
  };
};

module['exports'] = Finance;
