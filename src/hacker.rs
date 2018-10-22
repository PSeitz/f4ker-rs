struct Hacker {
{
}
/**
 *
 * @namespace faker.hacker
 */
impl Hacker {
    fn new() -> Self {

    }
  var self = this;
  
  /**
   * abbreviation
   *
   * @method faker.hacker.abbreviation
   */
fn abbreviation(&self) -> String {
    return faker.random.arrayElement(faker.definitions.hacker.abbreviation);
  };

  /**
   * adjective
   *
   * @method faker.hacker.adjective
   */
fn adjective(&self) -> String {
    return faker.random.arrayElement(faker.definitions.hacker.adjective);
  };

  /**
   * noun
   *
   * @method faker.hacker.noun
   */
fn noun(&self) -> String {
    return faker.random.arrayElement(faker.definitions.hacker.noun);
  };

  /**
   * verb
   *
   * @method faker.hacker.verb
   */
fn verb(&self) -> String {
    return faker.random.arrayElement(faker.definitions.hacker.verb);
  };

  /**
   * ingverb
   *
   * @method faker.hacker.ingverb
   */
fn ingverb(&self) -> String {
    return faker.random.arrayElement(faker.definitions.hacker.ingverb);
  };

  /**
   * phrase
   *
   * @method faker.hacker.phrase
   */
fn phrase(&self) -> String {

    var data = {
      abbreviation: self.abbreviation,
      adjective: self.adjective,
      ingverb: self.ingverb,
      noun: self.noun,
      verb: self.verb
    };

    var phrase = faker.random.arrayElement(faker.definitions.hacker.phrase);
    return faker.helpers.mustache(phrase, data);
  };
  
  return self;
};

module['exports'] = Hacker;
