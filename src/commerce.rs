use rand::{thread_rng, Rng};
use crate::faker::Faker;
use crate::*;
#[derive(Debug, Clone)]
pub struct Commerce <'a> {
    faker: &'a Faker,
{
}
/**
 *
 * @namespace faker.commerce
 */
impl Commerce {
    pub fn new(faker: &'a Faker) -> Self {
        Self { faker }

    }

  /**
   * color
   *
   * @method faker.commerce.color
   */
fn color(&self) -> String {
      return thread_rng().choose(self.faker.commerce_color()).unwrap();
  };

  /**
   * department
   *
   * @method faker.commerce.department
   */
fn department(&self) -> String {
      return thread_rng().choose(self.faker.commerce_department()).unwrap();
  };

  /**
   * productName
   *
   * @method faker.commerce.productName
   */
fn product_name(&self) -> String {
      return faker.commerce.productAdjective() + " " +
              faker.commerce.productMaterial() + " " +
              faker.commerce.product();
  };

  /**
   * price
   *
   * @method faker.commerce.price
   * @param {number} min
   * @param {number} max
   * @param {number} dec
   * @param {string} symbol
   *
   * @return {string}
   */
fn price(&self, min: &str,  max: &str,  dec: &str,  symbol: &str) -> String {
      min = min || 1;
      max = max || 1000;
      dec = dec == undefined ? 2 : dec;
      symbol = symbol || '';

      if (min < 0 || max < 0) {
          return symbol + 0.00;
      }

      let randValue = faker.random.number({ max: max, min: min });

      return symbol + (Math.round(randValue * Math.pow(10, dec)) / Math.pow(10, dec)).toFixed(dec);
  };

  /*
fn categories(&self, num: &str) -> String {
      let categories = [];

      do {
          let category = thread_rng().choose(self.faker.commerce_department()).unwrap();
          if(categories.indexOf(category) == -1) {
              categories.push(category);
          }
      } while(categories.length < num);

      return categories;
  };

  */
  /*
fn merge_categories(&self, categories: &str) -> String {
      let separator = self.faker.separator() || " &";
      // TODO: find undefined here
      categories = categories || self.faker.commerce_categories();
      let commaSeparated = categories.slice(0, -1).join(', ');

      return [commaSeparated, categories[categories.length - 1]].join(separator + " ");
  };
  */

  /**
   * productAdjective
   *
   * @method faker.commerce.productAdjective
   */
fn product_adjective(&self) -> String {
      return thread_rng().choose(self.faker.commerce_product_name_adjective()).unwrap();
  };

  /**
   * productMaterial
   *
   * @method faker.commerce.productMaterial
   */
fn product_material(&self) -> String {
      return thread_rng().choose(self.faker.commerce_product_name_material()).unwrap();
  };

  /**
   * product
   *
   * @method faker.commerce.product
   */
fn product(&self) -> String {
      return thread_rng().choose(self.faker.commerce_product_name_product()).unwrap();
  };

  return self;
};

module['exports'] = Commerce;
