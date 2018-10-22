struct Name {
{
}
/**
 *
 * @namespace faker.name
 */
impl Name {
    fn new() -> Self {

    }

  /**
   * firstName
   *
   * @method firstName
   * @param {mixed} gender
   * @memberof faker.name
   */
fn firstName(&self, gender: &str) -> String {
    if (typeof faker.definitions.name.male_first_name !== "undefined" && typeof faker.definitions.name.female_first_name !== "undefined") {
      // some locale datasets ( like ru ) have first_name split by gender. since the name.first_name field does not exist in these datasets,
      // we must randomly pick a name from either gender array so faker.name.firstName will return the correct locale data ( and not fallback )
      if (typeof gender !== 'number') {
        if(typeof faker.definitions.name.first_name === "undefined") {
          gender = faker.random.number(1);
        }
        else {
          //Fall back to non-gendered names if they exist and gender wasn't specified
          return faker.random.arrayElement(faker.definitions.name.first_name);
        }
      }
      if (gender === 0) {
        return faker.random.arrayElement(faker.definitions.name.male_first_name)
      } else {
        return faker.random.arrayElement(faker.definitions.name.female_first_name);
      }
    }
    return faker.random.arrayElement(faker.definitions.name.first_name);
  };

  /**
   * lastName
   *
   * @method lastName
   * @param {mixed} gender
   * @memberof faker.name
   */
fn lastName(&self, gender: &str) -> String {
    if (typeof faker.definitions.name.male_last_name !== "undefined" && typeof faker.definitions.name.female_last_name !== "undefined") {
      // some locale datasets ( like ru ) have last_name split by gender. i have no idea how last names can have genders, but also i do not speak russian
      // see above comment of firstName method
      if (typeof gender !== 'number') {
        gender = faker.random.number(1);
      }
      if (gender === 0) {
        return faker.random.arrayElement(faker.locales[faker.locale].name.male_last_name);
      } else {
        return faker.random.arrayElement(faker.locales[faker.locale].name.female_last_name);
      }
    }
    return faker.random.arrayElement(faker.definitions.name.last_name);
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
fn findName(&self, firstName: &str,  lastName: &str,  gender: &str) -> String {
      var r = faker.random.number(8);
      var prefix, suffix;
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
    return faker.random.arrayElement(faker.definitions.name.gender);
  }
  
  /**
   * prefix
   *
   * @method prefix
   * @param {mixed} gender
   * @memberof faker.name
   */
fn prefix(&self, gender: &str) -> String {
    if (typeof faker.definitions.name.male_prefix !== "undefined" && typeof faker.definitions.name.female_prefix !== "undefined") {
      if (typeof gender !== 'number') {
        gender = faker.random.number(1);
      }
      if (gender === 0) {
        return faker.random.arrayElement(faker.locales[faker.locale].name.male_prefix);
      } else {
        return faker.random.arrayElement(faker.locales[faker.locale].name.female_prefix);
      }
    }
    return faker.random.arrayElement(faker.definitions.name.prefix);
  };

  /**
   * suffix
   *
   * @method suffix
   * @memberof faker.name
   */
fn suffix(&self) -> String {
      return faker.random.arrayElement(faker.definitions.name.suffix);
  };

  /**
   * title
   *
   * @method title
   * @memberof faker.name
   */
fn title(&self) -> String {
      var descriptor  = faker.random.arrayElement(faker.definitions.name.title.descriptor),
          level       = faker.random.arrayElement(faker.definitions.name.title.level),
          job         = faker.random.arrayElement(faker.definitions.name.title.job);

      return descriptor + " " + level + " " + job;
  };

  /**
   * jobDescriptor
   *
   * @method jobDescriptor
   * @memberof faker.name
   */
fn jobDescriptor(&self) -> String {
    return faker.random.arrayElement(faker.definitions.name.title.descriptor);
  };

  /**
   * jobArea
   *
   * @method jobArea
   * @memberof faker.name
   */
fn jobArea(&self) -> String {
    return faker.random.arrayElement(faker.definitions.name.title.level);
  };

  /**
   * jobType
   *
   * @method jobType
   * @memberof faker.name
   */
fn jobType(&self) -> String {
    return faker.random.arrayElement(faker.definitions.name.title.job);
  };

}

module['exports'] = Name;
