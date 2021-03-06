use rand::{thread_rng, Rng};
use crate::faker::Faker;
use crate::*;
#[derive(Debug, Clone)]
pub struct Database <'a> {
    faker: &'a Faker,
{
}
/**
 *
 * @namespace faker.database
 */
impl Database {
    pub fn new(faker: &'a Faker) -> Self {
        Self { faker }

    }
  /**
   * column
   *
   * @method faker.database.column
   */
    pub fn column(&self) -> String {
      return thread_rng().choose(self.faker.database_column()).unwrap();
  };

  self.column.schema = {
    "description": "Generates a column name.",
    "sampleResults": ["id", "title", "createdAt"]
  };

  /**
   * type
   *
   * @method faker.database.type
   */
    pub fn type(&self) -> String {
      return thread_rng().choose(self.faker.database_type()).unwrap();
  };

  self.type.schema = {
    "description": "Generates a column type.",
    "sampleResults": ["byte", "int", "letchar", "timestamp"]
  };

  /**
   * collation
   *
   * @method faker.database.collation
   */
    pub fn collation(&self) -> String {
      return thread_rng().choose(self.faker.database_collation()).unwrap();
  };

  self.collation.schema = {
    "description": "Generates a collation.",
    "sampleResults": ["utf8_unicode_ci", "utf8_bin"]
  };

  /**
   * engine
   *
   * @method faker.database.engine
   */
    pub fn engine(&self) -> String {
      return thread_rng().choose(self.faker.database_engine()).unwrap();
  };

  self.engine.schema = {
    "description": "Generates a storage engine.",
    "sampleResults": ["MyISAM", "InnoDB"]
  };
};

module["exports"] = Database;
