use rand::{thread_rng, Rng};
use crate::faker::Faker;
use crate::*;
#[derive(Debug, Clone)]
pub struct Company <'a> {
    faker: &'a Faker,
}
impl<'a> Company<'a> {
    pub fn new(faker: &'a Faker) -> Self {
        Self { faker }
    }
  
    ///
    /// Generates a random localized city name.
    ///
    /// One of the following is randomly used:
    ///
    /// * `"{{name.last_name}} {{company.company_suffix}}"`
    /// * `"{{name.last_name}} - {{name.last_name}}"`
    /// * `"{{name.last_name}}, {{name.last_name}} and {{name.last_name}}`
    /// e.g. company_name Krajcik - Satterfield///
    pub fn company_name(&self) -> String {

    let formats = [
      "{{name.last_name}} {{company.suffix}}",
      "{{name.last_name}} - {{name.last_name}}",
      "{{name.last_name}}, {{name.last_name}} and {{name.last_name}}"
    ];

    self.faker.fake(rand!(formats))
  }

/// e.g. company_suffix "and Sons"
    pub fn company_suffix(&self) -> &'static str {
      return self.faker.company_suffix.rand();
  }

  /// format "{{company.catch_phrase_adjective}} {{company.catch_phrase_descriptor}} {{company.catch_phrase_noun}}"
  /// e.g. catch_phrase Realigned didactic encryption
    pub fn catch_phrase(&self) -> String {
      format!("{} {} {}", self.catch_phrase_adjective(), self.catch_phrase_descriptor(), self.catch_phrase_noun()) 
  }

  //
  /// format "{{company.bs_buzz}} {{company.bs_adjective}} {{company.bs_noun}}"
  /// e.g. bs generate frictionless relationships/ 
    pub fn bs(&self) -> String {
    // self.faker.fake("{{company.bs_buzz}} {{company.bs_adjective}} {{company.bs_noun}}")
    format!("{} {} {}", self.bs_buzz(), self.bs_adjective(), self.bs_noun()) 
  }

/// e.g. catch_phrase_adjective Multi-lateral
    pub fn catch_phrase_adjective(&self) -> &'static str {
      self.faker.company_adjective.rand()
  }

/// e.g. catch_phrase_descriptor reciprocal
    pub fn catch_phrase_descriptor(&self) -> &'static str {
      self.faker.company_descriptor.rand()
  }

/// e.g. catch_phrase_noun success
    pub fn catch_phrase_noun(&self) -> &'static str {
      self.faker.company_noun.rand()
  }

/// e.g. bs_adjective global
    pub fn bs_adjective(&self) -> &'static str {
      self.faker.company_bs_adjective.rand()
  }

/// e.g. bs_buzz recontextualize
    pub fn bs_buzz(&self) -> &'static str {
      self.faker.company_bs_verb.rand()
  }

/// e.g. bs_noun paradigms
    pub fn bs_noun(&self) -> &'static str {
      self.faker.company_bs_noun.rand()
  }
  
}


#[test]
fn company() {
    let faker = Faker::new();
    let company = faker.company();
    println!("\ncompany_name {}", company.company_name());
    println!("company_suffix {}", company.company_suffix());
    println!("catch_phrase {}", company.catch_phrase());
    println!("bs {}", company.bs());
    println!("catch_phrase_adjective {}", company.catch_phrase_adjective());
    println!("catch_phrase_descriptor {}", company.catch_phrase_descriptor());
    println!("catch_phrase_noun {}", company.catch_phrase_noun());
    println!("bs_adjective {}", company.bs_adjective());
    println!("bs_buzz {}", company.bs_buzz());
    println!("bs_noun {}", company.bs_noun());
}
