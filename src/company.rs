use rand::{thread_rng, Rng};
use crate::faker::Faker;
use crate::*;
#[derive(Debug, Clone)]
pub struct Company <'a> {
    faker: &'a Faker,
{
}
impl Company {
    pub fn new(faker: &'a Faker) -> Self {
        Self { faker }
    }
  

    pub fn company_name(&self, format: &str) -> String {

    let formats = [
      "{{name.last_name}} {{company.company_suffix}}",
      "{{name.last_name}} - {{name.last_name}}",
      "{{name.last_name}}, {{name.last_name}} and {{name.last_name}}"
    ];

    self.faker.fake(rand!(formats))
  }

    pub fn company_suffix(&self) -> &'static str {
      return self.faker.company_suffix.rand();
  }

  /// format "{{company.catchPhraseAdjective}} {{company.catchPhraseDescriptor}} {{company.catchPhraseNoun}}"
    pub fn catch_phrase(&self) -> String {
    self.faker.fake("{{company.catchPhraseAdjective}} {{company.catchPhraseDescriptor}} {{company.catchPhraseNoun}}")
  }

  /// format "{{company.bsBuzz}} {{company.bsAdjective}} {{company.bsNoun}}"
    pub fn bs(&self) -> String {
    self.faker.fake("{{company.bsBuzz}} {{company.bsAdjective}} {{company.bsNoun}}")
  }

    pub fn catch_phrase_adjective(&self) -> &'static str {
      self.faker.company_adjective.rand()
  }

    pub fn catch_phrase_descriptor(&self) -> &'static str {
      self.faker.company_descriptor.rand()
  }

    pub fn catch_phrase_noun(&self) -> &'static str {
      self.faker.company_noun.rand()
  }

    pub fn bs_adjective(&self) -> &'static str {
      self.faker.company_bs_adjective.rand()
  }

    pub fn bs_buzz(&self) -> &'static str {
      self.faker.company_bs_verb.rand()
  }

    pub fn bs_noun(&self) -> &'static str {
      return thread_rng().choose(self.faker.company_bs_noun()).unwrap();
  }
  
}

