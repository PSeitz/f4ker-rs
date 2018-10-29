use rand::{thread_rng, Rng};
struct Name {
{
  faker: Faker
}

#[derive(Debug)]
pub enum Gender {
  Male,
  Female,
}

const GENDERS: [Gender;2] = [Gender::Male, Gender::Female];

/**
 *
 * @namespace faker.name
 */
impl Name {
    fn new(faker: Faker) -> Self {
      Name{faker}
    }

  /**
   * firstName
   *
   * @method firstName
   * @param {mixed} gender
   * @memberof faker.name
   */
fn firstName(&self, gender: Option<Gender>) -> String {
    let gender = gender.unwrap_or_else(||thread_rng().choose(&GENDERS));
    if (self.faker.name_male_first_name.is_some() && self.faker.name_female_first_name.is_some()) {
      // some locale datasets ( like ru ) have first_name split by gender. since the name.first_name field does not exist in these datasets,
      // we must randomly pick a name from either gender array so faker.name.firstName will return the correct locale data ( and not fallback )
      if (typeof gender !== 'number') {
        if(self.faker.name_first_name().is_none()) {
          gender = faker.random.number(1);
        }
        else {
          //Fall back to non-gendered names if they exist and gender wasn't specified
          return thread_rng().choose(self.faker.name_first_name());
        }
      }
      if (gender == 0) {
        return thread_rng().choose(self.faker.name_male_first_name())
      } else {
        return thread_rng().choose(self.faker.name_female_first_name());
      }
    }
    return thread_rng().choose(self.faker.name_first_name());
  };

  /**
   * lastName
   *
   * @method lastName
   * @param {mixed} gender
   * @memberof faker.name
   */
fn lastName(&self, gender: Option<Gender>) -> String {
    let gender = gender.unwrap_or_else(||thread_rng().choose(&GENDERS));
    if (self.faker.name_male_last_name.is_some() && self.faker.name_female_last_name.is_some()) {
      // some locale datasets ( like ru ) have last_name split by gender. i have no idea how last names can have GENDERS, but also i do not speak russian
      // see above comment of firstName method
      if (typeof gender !== 'number') {
        gender = faker.random.number(1);
      }
      if (gender == 0) {
        return thread_rng().choose(faker.locales[faker.locale].name.male_last_name);
      } else {
        return thread_rng().choose(faker.locales[faker.locale].name.female_last_name);
      }
    }
    return thread_rng().choose(self.faker.name_last_name());
  };

  /**
   * findName
   *
   * @method findName
   * @param {string} firstName
   * @param {string} lastName
   * @param {mixed} gender
   * @memberof faker.name
   */
fn findName(&self, firstName: &str,  lastName: &str,  gender: Option<Gender>) -> String {
    let gender = gender.unwrap_or_else(||thread_rng().choose(&GENDERS));
      let r = faker.random.number(8);
      let prefix, suffix;
      // in particular locales first and last names split by gender,
      // thus we keep consistency by passing 0 as male and 1 as female
      if (typeof gender !== 'number') {
        gender = faker.random.number(1);
      }
      firstName = firstName || faker.name.firstName(gender);
      lastName = lastName || faker.name.lastName(gender);
      switch (r) {
      case 0:
          prefix = faker.name.prefix(gender);
          if (prefix) {
              return prefix + " " + firstName + " " + lastName;
          }
      case 1:
          suffix = faker.name.suffix(gender);
          if (suffix) {
              return firstName + " " + lastName + " " + suffix;
          }
      }

      return firstName + " " + lastName;
  };

  /**
   * jobTitle
   *
   * @method jobTitle
   * @memberof faker.name
   */
fn jobTitle(&self) -> String {
    return  faker.name.jobDescriptor() + " " +
      faker.name.jobArea() + " " +
      faker.name.jobType();
  };

  /**
   * gender
   *
   * @method gender
   * @memberof faker.name
   */
fn gender(&self) -> String {
    return thread_rng().choose(self.faker.name_gender());
  }
  
  /**
   * prefix
   *
   * @method prefix
   * @param {mixed} gender
   * @memberof faker.name
   */
fn prefix(&self, gender: Option<Gender>) -> String {
    let gender = gender.unwrap_or_else(||thread_rng().choose(&GENDERS));
    if (self.faker.name_male_prefix.is_some() && self.faker.name_female_prefix.is_some()) {
      if (typeof gender !== 'number') {
        gender = faker.random.number(1);
      }
      if (gender == 0) {
        return thread_rng().choose(faker.locales[faker.locale].name.male_prefix);
      } else {
        return thread_rng().choose(faker.locales[faker.locale].name.female_prefix);
      }
    }
    return thread_rng().choose(self.faker.name_prefix());
  };

  /**
   * suffix
   *
   * @method suffix
   * @memberof faker.name
   */
fn suffix(&self) -> String {
      return thread_rng().choose(self.faker.name_suffix());
  };

  /**
   * title
   *
   * @method title
   * @memberof faker.name
   */
fn title(&self) -> String {
      let descriptor  = thread_rng().choose(self.faker.name_title_descriptor()),
          level       = thread_rng().choose(self.faker.name_title_level()),
          job         = thread_rng().choose(self.faker.name_title_job());

      return descriptor + " " + level + " " + job;
  };

  /**
   * jobDescriptor
   *
   * @method jobDescriptor
   * @memberof faker.name
   */
fn jobDescriptor(&self) -> String {
    return thread_rng().choose(self.faker.name_title_descriptor());
  };

  /**
   * jobArea
   *
   * @method jobArea
   * @memberof faker.name
   */
fn jobArea(&self) -> String {
    return thread_rng().choose(self.faker.name_title_level());
  };

  /**
   * jobType
   *
   * @method jobType
   * @memberof faker.name
   */
fn jobType(&self) -> String {
    return thread_rng().choose(self.faker.name_title_job());
  };

}

module['exports'] = Name;
