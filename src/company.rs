use rand::{thread_rng, Rng};
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
  
  let self = this;
  let f = faker.fake;
  
  /**
   * suffixes
   *
   * @method faker.company.suffixes
   */
fn suffixes(&self) -> String {
    // Don't want the source array exposed to modification, so return a copy
    return self.faker.company_suffix_slice()(0);
  }

  /**
   * companyName
   *
   * @method faker.company.companyName
   * @param {string} format
   */
fn companyName(&self, format: &str) -> String {

    let formats = [
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
      return thread_rng().choose(faker.company.suffixes());
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
      return thread_rng().choose(self.faker.company_adjective());
  }

  /**
   * catchPhraseDescriptor
   *
   * @method faker.company.catchPhraseDescriptor
   */
fn catchPhraseDescriptor(&self) -> String {
      return thread_rng().choose(self.faker.company_descriptor());
  }

  /**
   * catchPhraseNoun
   *
   * @method faker.company.catchPhraseNoun
   */
fn catchPhraseNoun(&self) -> String {
      return thread_rng().choose(self.faker.company_noun());
  }

  /**
   * bsAdjective
   *
   * @method faker.company.bsAdjective
   */
fn bsAdjective(&self) -> String {
      return thread_rng().choose(self.faker.company_bs_adjective());
  }

  /**
   * bsBuzz
   *
   * @method faker.company.bsBuzz
   */
fn bsBuzz(&self) -> String {
      return thread_rng().choose(self.faker.company_bs_verb());
  }

  /**
   * bsNoun
   *
   * @method faker.company.bsNoun
   */
fn bsNoun(&self) -> String {
      return thread_rng().choose(self.faker.company_bs_noun());
  }
  
}

module['exports'] = Company;
