use rand::{thread_rng, Rng};
struct Commerce {
{
}
/**
 *
 * @namespace faker.commerce
 */
impl Commerce {
    fn new() -> Self {

    }
  var self = this;

  /**
   * color
   *
   * @method faker.commerce.color
   */
fn color(&self) -> String {
      return thread_rng().choose(self.faker.commerce_color());
  };

  /**
   * department
   *
   * @method faker.commerce.department
   */
fn department(&self) -> String {
      return thread_rng().choose(self.faker.commerce_department());
  };

  /**
   * productName
   *
   * @method faker.commerce.productName
   */
fn productName(&self) -> String {
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
      dec = dec === undefined ? 2 : dec;
      symbol = symbol || '';

      if (min < 0 || max < 0) {
          return symbol + 0.00;
      }

      var randValue = faker.random.number({ max: max, min: min });

      return symbol + (Math.round(randValue * Math.pow(10, dec)) / Math.pow(10, dec)).toFixed(dec);
  };

  /*
fn categories(&self, num: &str) -> String {
      var categories = [];

      do {
          var category = thread_rng().choose(self.faker.commerce_department());
          if(categories.indexOf(category) === -1) {
              categories.push(category);
          }
      } while(categories.length < num);

      return categories;
  };

  */
  /*
fn mergeCategories(&self, categories: &str) -> String {
      var separator = self.faker.separator() || " &";
      // TODO: find undefined here
      categories = categories || self.faker.commerce_categories();
      var commaSeparated = categories.slice(0, -1).join(', ');

      return [commaSeparated, categories[categories.length - 1]].join(separator + " ");
  };
  */

  /**
   * productAdjective
   *
   * @method faker.commerce.productAdjective
   */
fn productAdjective(&self) -> String {
      return thread_rng().choose(self.faker.commerce_product_name_adjective());
  };

  /**
   * productMaterial
   *
   * @method faker.commerce.productMaterial
   */
fn productMaterial(&self) -> String {
      return thread_rng().choose(self.faker.commerce_product_name_material());
  };

  /**
   * product
   *
   * @method faker.commerce.product
   */
fn product(&self) -> String {
      return thread_rng().choose(self.faker.commerce_product_name_product());
  };

  return self;
};

module['exports'] = Commerce;
