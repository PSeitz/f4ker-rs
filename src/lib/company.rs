struct Company {
{
}
/**
 *
 * @namespace faker.company
 */
impl Company {
    fn new() -> Self {

    }
  
  var self = this;
  var f = faker.fake;
  
  /**
   * suffixes
   *
   * @method faker.company.suffixes
   */
fn suffixes(&self) -> String {
    // Don't want the source array exposed to modification, so return a copy
    return faker.definitions.company.suffix.slice(0);
  }

  /**
   * companyName
   *
   * @method faker.company.companyName
   * @param {string} format
   */
fn companyName(&self, format: &str) -> String {

    var formats = [
      '{{name.lastName}} {{company.companySuffix}}',
      '{{name.lastName}} - {{name.lastName}}',
      '{{name.lastName}}, {{name.lastName}} and {{name.lastName}}'
    ];

    if (typeof format !== "number") {
      format = faker.random.number(formats.length - 1);
    }

    return f(formats[format]);
  }

  /**
   * companySuffix
   *
   * @method faker.company.companySuffix
   */
fn companySuffix(&self) -> String {
      return faker.random.arrayElement(faker.company.suffixes());
  }

  /**
   * catchPhrase
   *
   * @method faker.company.catchPhrase
   */
fn catchPhrase(&self) -> String {
    return f('{{company.catchPhraseAdjective}} {{company.catchPhraseDescriptor}} {{company.catchPhraseNoun}}')
  }

  /**
   * bs
   *
   * @method faker.company.bs
   */
fn bs(&self) -> String {
    return f('{{company.bsBuzz}} {{company.bsAdjective}} {{company.bsNoun}}');
  }

  /**
   * catchPhraseAdjective
   *
   * @method faker.company.catchPhraseAdjective
   */
fn catchPhraseAdjective(&self) -> String {
      return faker.random.arrayElement(faker.definitions.company.adjective);
  }

  /**
   * catchPhraseDescriptor
   *
   * @method faker.company.catchPhraseDescriptor
   */
fn catchPhraseDescriptor(&self) -> String {
      return faker.random.arrayElement(faker.definitions.company.descriptor);
  }

  /**
   * catchPhraseNoun
   *
   * @method faker.company.catchPhraseNoun
   */
fn catchPhraseNoun(&self) -> String {
      return faker.random.arrayElement(faker.definitions.company.noun);
  }

  /**
   * bsAdjective
   *
   * @method faker.company.bsAdjective
   */
fn bsAdjective(&self) -> String {
      return faker.random.arrayElement(faker.definitions.company.bs_adjective);
  }

  /**
   * bsBuzz
   *
   * @method faker.company.bsBuzz
   */
fn bsBuzz(&self) -> String {
      return faker.random.arrayElement(faker.definitions.company.bs_verb);
  }

  /**
   * bsNoun
   *
   * @method faker.company.bsNoun
   */
fn bsNoun(&self) -> String {
      return faker.random.arrayElement(faker.definitions.company.bs_noun);
  }
  
}

module['exports'] = Company;
