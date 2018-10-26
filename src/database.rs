use rand::{thread_rng, Rng};
struct Database {
{
}
/**
 *
 * @namespace faker.database
 */
impl Database {
    fn new() -> Self {

    }
  var self = this;
  /**
   * column
   *
   * @method faker.database.column
   */
fn column(&self) -> String {
      return thread_rng().choose(self.faker.database_column());
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
fn type(&self) -> String {
      return thread_rng().choose(self.faker.database_type());
  };

  self.type.schema = {
    "description": "Generates a column type.",
    "sampleResults": ["byte", "int", "varchar", "timestamp"]
  };

  /**
   * collation
   *
   * @method faker.database.collation
   */
fn collation(&self) -> String {
      return thread_rng().choose(self.faker.database_collation());
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
fn engine(&self) -> String {
      return thread_rng().choose(self.faker.database_engine());
  };

  self.engine.schema = {
    "description": "Generates a storage engine.",
    "sampleResults": ["MyISAM", "InnoDB"]
  };
};

module["exports"] = Database;
