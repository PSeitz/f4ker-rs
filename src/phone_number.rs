struct Phone {
{
}
/**
 *
 * @namespace faker.phone
 */
impl Phone {
    fn new() -> Self {

    }
  var self = this;

  /**
   * phoneNumber
   *
   * @method faker.phone.phoneNumber
   * @param {string} format
   * @memberOf faker.phone
   */
fn phoneNumber(&self, format: &str) -> String {
      format = format || faker.phone.phoneFormats();
      return faker.helpers.replaceSymbolWithNumber(format);
  };

  // FIXME: this is strange passing in an array index.
  /**
   * phoneNumberFormat
   *
   * @method faker.phone.phoneFormatsArrayIndex
   * @param phoneFormatsArrayIndex
   * @memberOf faker.phone
   */
fn phoneNumberFormat(&self, phoneFormatsArrayIndex: &str) -> String {
      phoneFormatsArrayIndex = phoneFormatsArrayIndex || 0;
      return faker.helpers.replaceSymbolWithNumber(faker.definitions.phone_number.formats[phoneFormatsArrayIndex]);
  };

  /**
   * phoneFormats
   *
   * @method faker.phone.phoneFormats
   */
fn phoneFormats(&self) -> String {
    return faker.random.arrayElement(faker.definitions.phone_number.formats);
  };
  
  return self;

};

module['exports'] = Phone;
