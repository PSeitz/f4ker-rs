use rand::{thread_rng, Rng};
use crate::faker::Faker;
use crate::*;
use uuid;
#[derive(Debug, Clone)]
pub struct Random <'a> {
    faker: &'a Faker,
}
// let mersenne = require('../vendor/mersenne');

/**
 *
 * @namespace faker.random
 */
impl<'a> Random<'a> {
    pub fn new(faker: &'a Faker) -> Self {
        Random { faker }
    }
  // Use a user provided seed if it exists
  // if (seed) {
  //   if (Array.isArray(seed) && seed.length) {
  //     mersenne.seed_array(seed);
  //   }
  //   else {
  //     mersenne.seed(seed);
  //   }
  // }
  /**
   * returns a single random number based on a max number or range
   *
   * @method faker.random.number
   * @param {mixed} options {min, max, precision}
   */
    // pub fn number(&self, options: &str) -> String {

    //     if (typeof options == "number") {
    //       options = {
    //         max: options
    //       };
    //     }

    //     options = options || {};

    //     if (options.min.is_none()) {
    //       options.min = 0;
    //     }

    //     if (options.max.is_none()) {
    //       options.max = 99999;
    //     }
    //     if (options.precision.is_none()) {
    //       options.precision = 1;
    //     }

    //     // Make the range inclusive of the max value
    //     let max = options.max;
    //     if (max >= 0) {
    //       max += options.precision;
    //     }

    //     let randomNumber = Math.floor(
    //       mersenne.rand(max / options.precision, options.min / options.precision));
    //     // Workaround problem in Float point arithmetics for e.g. 6681493 / 0.01
    //     randomNumber = randomNumber / (1 / options.precision);

    //     return randomNumber;

    // }

  /**
   * returns a single random floating-point number based on a max number or range
   *
   * @method faker.random.float
   * @param {mixed} options
   */
  //   pub fn float(&self, options: &str) -> String {
  //     if (typeof options == "number") {
  //       options = {
  //         precision: options
  //       };
  //     }
  //     options = options || {};
  //     let opts = {};
  //     for (let p in options) {
  //       opts[p] = options[p];
  //     }
  //     if (opts.precision.is_none()) {
  //       opts.precision = 0.01;
  //     }
  //     return faker.random.number(opts);
  // }

  /**
   * takes an array and returns a random element of the array
   *
   * @method faker.random.arrayElement
   * @param {array} array
   */
  //   pub fn array_element(&self, array: &str) -> String {
  //     array = array || ["a", "b", "c"];
  //     let r = faker.random.number({ max: array.length - 1 });
  //     return array[r];
  // }

  /**
   * takes an array and returns a subset with random elements of the array
   *
   * @method faker.random.arrayElements
   * @param {array} array
   * @param {number} count number of elements to pick
   */
  //   pub fn array_elements(&self, array: &str,  count: &str) -> String {
  //     array = array || ["a", "b", "c"];

  //     if (typeof count !== 'number') {
  //       count = faker.random.number({ min: 1, max: array.length });
  //     } else if (count > array.length) {
  //       count = array.length;
  //     } else if (count < 0) {
  //       count = 0;
  //     }

  //     let arrayCopy = array.slice();
  //     let countToRemove = arrayCopy.length - count;
  //     for (let i = 0; i < countToRemove; i++) {
  //       let indexToRemove = faker.random.number({ max: arrayCopy.length - 1 });
  //       arrayCopy.splice(indexToRemove, 1);
  //     }

  //     return arrayCopy;
  // }

  /**
   * takes an object and returns the randomly key or value
   *
   * @method faker.random.objectElement
   * @param {object} object
   * @param {mixed} field
   */
  //   pub fn object_element(&self, object: &str,  field: &str) -> String {
  //     object = object || { "foo": "bar", "too": "car" };
  //     let array = Object.keys(object);
  //     let key = thread_rng().choose(array).unwrap();

  //     return field == "key" ? key : object[key];
  // }

  /**
   * uuid
   *
   * @method faker.random.uuid
   */
    pub fn uuid_plain(&self) -> String {
      uuid::Uuid::new_v4().to_simple().to_string()
    }
    pub fn uuid(&self) -> String {
      uuid::Uuid::new_v4().to_string()
      // println!("{}", uuid::Uuid::new_v4().to_simple().to_string())

      // let RFC4122_TEMPLATE = 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx';
      // let replacePlaceholders = function (placeholder) {
      //     let random = self.number({ min: 0, max: 15 });
      //     let value = placeholder == 'x' ? random : (random &0x3 | 0x8);
      //     return value.toString(16);
      // };
      // return RFC4122_TEMPLATE.replace(/[xy]/g, replacePlaceholders);
  }

  // /**
  //  * boolean
  //  *
  //  * @method faker.random.boolean
  //  */
  //   pub fn boolean(&self) -> String {
  //     return !!faker.random.number(1)
  // }

  // // TODO: have ability to return specific type of word? As in: noun, adjective, verb, etc
  // /**
  //  * word
  //  *
  //  * @method faker.random.word
  //  * @param {string} type
  //  */
  //   pub fn word(&self, type: &str) -> String {

  //   let wordMethods = [
  //   'commerce.department',
  //   'commerce.productName',
  //   'commerce.productAdjective',
  //   'commerce.productMaterial',
  //   'commerce.product',
  //   'commerce.color',

  //   'company.catchPhraseAdjective',
  //   'company.catchPhraseDescriptor',
  //   'company.catchPhraseNoun',
  //   'company.bsAdjective',
  //   'company.bsBuzz',
  //   'company.bsNoun',
  //   'address.streetSuffix',
  //   'address.county',
  //   'address.country',
  //   'address.state',

  //   'finance.accountName',
  //   'finance.transactionType',
  //   'finance.currencyName',

  //   'hacker.noun',
  //   'hacker.verb',
  //   'hacker.adjective',
  //   'hacker.ingverb',
  //   'hacker.abbreviation',

  //   'name.jobDescriptor',
  //   'name.jobArea',
  //   'name.jobType'];

  //   // randomly pick from the many faker methods that can generate words
  //   let randomWordMethod = thread_rng().choose(wordMethods).unwrap();
  //   return faker.fake('{{' + randomWordMethod + '}}');

  // }

  // /**
  //  * randomWords
  //  *
  //  * @method faker.random.words
  //  * @param {number} count defaults to a random value between 1 and 3
  //  */
  //   pub fn words(&self, count: &str) -> String {
  //   let words = [];
  //   if (count.is_none()) {
  //     count = faker.random.number({min:1, max: 3});
  //   }
  //   for (let i = 0; i<count; i++) {
  //     words.push(faker.random.word());
  //   }
  //   return words.join(' ');
  // }

  // /**
  //  * locale
  //  *
  //  * @method faker.random.image
  //  */
  //   pub fn image(&self) -> String {
  //   return faker.image.image();
  // }

  // /**
  //  * locale
  //  *
  //  * @method faker.random.locale
  //  */
  //   pub fn locale(&self) -> String {
  //   return thread_rng().choose(Object.keys(faker.locales)).unwrap();
  // };

  // /**
  //  * alphaNumeric
  //  *
  //  * @method faker.random.alphaNumeric
  //  * @param {number} count defaults to 1
  //  */
  //   pub fn alpha_numeric(&self, count: &str) -> String {
  //   if (count.is_none()) {
  //     count = 1;
  //   }

  //   let wholeString = "";
  //   for(let i = 0; i < count; i++) {
  //     wholeString += thread_rng().choose(["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"]).unwrap();
  //   }

  //   return wholeString;
  // };

  // /**
  //  * hexaDecimal
  //  *
  //  * @method faker.random.hexaDecimal
  //  * @param {number} count defaults to 1
  //  */
  //   pub fn hexa_decimal(&self, count: &str) -> String {
  //   if (count.is_none()) {
  //     count = 1;
  //   }

  //   let wholeString = "";
  //   for(let i = 0; i < count; i++) {
  //     wholeString += thread_rng().choose(["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f", "A", "B", "C", "D", "E", "F"]).unwrap();
  //   }

  //   return "0x"+wholeString;
  // };

  // return this;

}

#[test]
fn name() {
    let facker = Faker::new();
    let rand = Random::new(&facker);
    assert_eq!(rand.uuid_plain().contains("-"), false);
    assert_eq!(rand.uuid().contains("-"), true);
}
