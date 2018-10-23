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
      return faker.random.arrayElement(faker.definitions.commerce.color);
  };

  /**
   * department
   *
   * @method faker.commerce.department
   */
fn department(&self) -> String {
      return faker.random.arrayElement(faker.definitions.commerce.department);
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
          var category = faker.random.arrayElement(faker.definitions.commerce.department);
          if(categories.indexOf(category) === -1) {
              categories.push(category);
          }
      } while(categories.length < num);

      return categories;
  };

  */
  /*
fn mergeCategories(&self, categories: &str) -> String {
      var separator = faker.definitions.separator || " &";
      // TODO: find undefined here
      categories = categories || faker.definitions.commerce.categories;
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
      return faker.random.arrayElement(faker.definitions.commerce.product_name.adjective);
  };

  /**
   * productMaterial
   *
   * @method faker.commerce.productMaterial
   */
fn productMaterial(&self) -> String {
      return faker.random.arrayElement(faker.definitions.commerce.product_name.material);
  };

  /**
   * product
   *
   * @method faker.commerce.product
   */
fn product(&self) -> String {
      return faker.random.arrayElement(faker.definitions.commerce.product_name.product);
  };

  return self;
};

module['exports'] = Commerce;