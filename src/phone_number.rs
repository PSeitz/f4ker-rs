use rand::{thread_rng, Rng};
#[derive(Debug, Clone)]
pub struct Phone <'a> {
    faker: &'a Faker,
{
}
/**
 *
 * @namespace faker.phone
 */
impl Phone {
    pub fn new(faker: &'a Faker) -> Self {
        Self { faker }

    }
  let self = this;

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
      return faker.helpers.replaceSymbolWithNumber(self.faker.phone_number_formats()[phoneFormatsArrayIndex]);
  };

  /**
   * phoneFormats
   *
   * @method faker.phone.phoneFormats
   */
fn phoneFormats(&self) -> String {
    return thread_rng().choose(self.faker.phone_number_formats()).unwrap();
  };
  
  return self;

};

module['exports'] = Phone;
