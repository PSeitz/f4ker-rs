use rand::{thread_rng, Rng};
use crate::faker::Faker;
use crate::*;
#[derive(Debug, Clone)]
pub struct Company <'a> {
    faker: &'a Faker,
{
}
/**
 *
 * @namespace faker.company
 */
impl Company {
    pub fn new(faker: &'a Faker) -> Self {
        Self { faker }

    }
  
  
  /**
   * suffixes
   *
   * @method faker.company.suffixes
   */
    pub fn suffixes(&self) -> String {
    // Don't want the source array exposed to modification, so return a copy
    return self.faker.company_suffix_slice()(0);
  }

  /**
   * companyName
   *
   * @method faker.company.companyName
   * @param {string} format
   */
    pub fn company_name(&self, format: &str) -> String {

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
    pub fn company_suffix(&self) -> String {
      return thread_rng().choose(faker.company.suffixes()).unwrap();
  }

  /**
   * catchPhrase
   *
   * @method faker.company.catchPhrase
   */
    pub fn catch_phrase(&self) -> String {
    return f('{{company.catchPhraseAdjective}} {{company.catchPhraseDescriptor}} {{company.catchPhraseNoun}}')
  }

  /**
   * bs
   *
   * @method faker.company.bs
   */
    pub fn bs(&self) -> String {
    return f('{{company.bsBuzz}} {{company.bsAdjective}} {{company.bsNoun}}');
  }

  /**
   * catchPhraseAdjective
   *
   * @method faker.company.catchPhraseAdjective
   */
    pub fn catch_phrase_adjective(&self) -> String {
      return thread_rng().choose(self.faker.company_adjective()).unwrap();
  }

  /**
   * catchPhraseDescriptor
   *
   * @method faker.company.catchPhraseDescriptor
   */
    pub fn catch_phrase_descriptor(&self) -> String {
      return thread_rng().choose(self.faker.company_descriptor()).unwrap();
  }

  /**
   * catchPhraseNoun
   *
   * @method faker.company.catchPhraseNoun
   */
    pub fn catch_phrase_noun(&self) -> String {
      return thread_rng().choose(self.faker.company_noun()).unwrap();
  }

  /**
   * bsAdjective
   *
   * @method faker.company.bsAdjective
   */
    pub fn bs_adjective(&self) -> String {
      return thread_rng().choose(self.faker.company_bs_adjective()).unwrap();
  }

  /**
   * bsBuzz
   *
   * @method faker.company.bsBuzz
   */
    pub fn bs_buzz(&self) -> String {
      return thread_rng().choose(self.faker.company_bs_verb()).unwrap();
  }

  /**
   * bsNoun
   *
   * @method faker.company.bsNoun
   */
    pub fn bs_noun(&self) -> String {
      return thread_rng().choose(self.faker.company_bs_noun()).unwrap();
  }
  
}

module['exports'] = Company;
