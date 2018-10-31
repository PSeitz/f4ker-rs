use rand::{thread_rng, Rng};
use crate::faker::Faker;
use crate::RandArray;
#[derive(Debug, Clone)]
pub struct Hacker <'a> {
    faker: &'a Faker,
{
}
/**
 *
 * @namespace faker.hacker
 */
impl Hacker {
    pub fn new(faker: &'a Faker) -> Self {
        Self { faker }

    }
  
  /**
   * abbreviation
   *
   * @method faker.hacker.abbreviation
   */
fn abbreviation(&self) -> String {
    return thread_rng().choose(self.faker.hacker_abbreviation()).unwrap();
  };

  /**
   * adjective
   *
   * @method faker.hacker.adjective
   */
fn adjective(&self) -> String {
    return thread_rng().choose(self.faker.hacker_adjective()).unwrap();
  };

  /**
   * noun
   *
   * @method faker.hacker.noun
   */
fn noun(&self) -> String {
    return thread_rng().choose(self.faker.hacker_noun()).unwrap();
  };

  /**
   * verb
   *
   * @method faker.hacker.verb
   */
fn verb(&self) -> String {
    return thread_rng().choose(self.faker.hacker_verb()).unwrap();
  };

  /**
   * ingverb
   *
   * @method faker.hacker.ingverb
   */
fn ingverb(&self) -> String {
    return thread_rng().choose(self.faker.hacker_ingverb()).unwrap();
  };

  /**
   * phrase
   *
   * @method faker.hacker.phrase
   */
fn phrase(&self) -> String {

    let data = {
      abbreviation: self.abbreviation,
      adjective: self.adjective,
      ingverb: self.ingverb,
      noun: self.noun,
      verb: self.verb
    };

    let phrase = thread_rng().choose(self.faker.hacker_phrase()).unwrap();
    return faker.helpers.mustache(phrase, data);
  };
  
  return self;
};

module['exports'] = Hacker;
