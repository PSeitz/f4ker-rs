use rand::{thread_rng, Rng};
use crate::faker::Faker;
#[derive(Debug, Clone)]
pub struct Name {
    faker: Faker
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Gender {
    Male,
    Female,
}

const GENDERS: [Gender;2] = [Gender::Male, Gender::Female];

impl Name {
    fn new(faker: Faker) -> Self {
        Name{faker}
    }

    fn firstName(&self, gender: Option<Gender>) -> String {
        let gender = gender.unwrap_or_else(||thread_rng().choose(&GENDERS));

        if let (Some(name_male_first_name),Some(name_female_first_name)) = (self.faker.name_male_first_name,self.faker.name_female_first_name) {
        // if (self.faker.name_male_first_name.is_some() && self.faker.name_female_first_name.is_some()) {
            // some locale datasets ( like ru ) have first_name split by gender. since the name.first_name field does not exist in these datasets,
            // we must randomly pick a name from either gender array so faker.name.firstName will return the correct locale data ( and not fallback )
            if gender == Gender::Male {
                return thread_rng().choose(name_male_first_name)
            } else {
                return thread_rng().choose(name_female_first_name);
            }
        }
        return thread_rng().choose(self.faker.name_first_name());
    }

    fn lastName(&self, gender: Option<Gender>) -> String {
        let gender = gender.unwrap_or_else(||thread_rng().choose(&GENDERS));
        if (self.faker.name_male_last_name.is_some() && self.faker.name_female_last_name.is_some()) {
            // some locale datasets ( like ru ) have last_name split by gender. i have no idea how last names can have GENDERS, but also i do not speak russian
            // see above comment of firstName method
            if gender == Gender::Male {
                return thread_rng().choose(self.faker.name_male_last_name());
            } else {
                return thread_rng().choose(self.faker.name_female_last_name());
            }
        }
        return thread_rng().choose(self.faker.name_last_name());
    }

    /**
     * findName
     *
     * @method findName
     * @param {string} firstName
     * @param {string} lastName
     * @param {mixed} gender
     * @memberof faker.name
     */
    fn findName(&self) -> String {
        let gender = gender.unwrap_or_else(||thread_rng().choose(&GENDERS));
        let r = thread_rng().gen_range(0, 8);
        // in particular locales first and last names split by gender,
        // thus we keep consistency by passing 0 as male and 1 as female
        let firstName = self.firstName(gender);
        let lastName = self.lastName(gender);
        match r {
            0 => {
                self.prefix(gender) + " " + &firstName + " " + &lastName;
            },
            1 => {
                firstName + " " + &lastName + " " + &self.suffix(gender);
            },
            _ => firstName + " " + &lastName
        }

    }

    /**
     * jobTitle
     *
     * @method jobTitle
     * @memberof self.faker.name
     */
    fn jobTitle(&self) -> String {
        return  self.jobDescriptor() + " " +
            self.jobArea() + " " +
            self.jobType();
    }

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
            if gender == Gender::Male {
                return thread_rng().choose(self.faker.name_male_prefix());
            } else {
                return thread_rng().choose(self.faker.name_female_prefix());
            }
        }
        return thread_rng().choose(self.faker.name_prefix());
    }

    /**
     * suffix
     *
     * @method suffix
     * @memberof faker.name
     */
    fn suffix(&self) -> String {
            return thread_rng().choose(self.faker.name_suffix());
    }

    /**
     * title
     *
     * @method title
     * @memberof faker.name
     */
    fn title(&self) -> String {
        let descriptor  = thread_rng().choose(self.faker.name_title_descriptor());
        let level       = thread_rng().choose(self.faker.name_title_level());
        let job         = thread_rng().choose(self.faker.name_title_job());

        return descriptor.to_string() + " " + level + " " + job;
    }

    /**
     * jobDescriptor
     *
     * @method jobDescriptor
     * @memberof faker.name
     */
    fn jobDescriptor(&self) -> String {
        return thread_rng().choose(self.faker.name_title_descriptor());
    }

    /**
     * jobArea
     *
     * @method jobArea
     * @memberof faker.name
     */
    fn jobArea(&self) -> String {
        return thread_rng().choose(self.faker.name_title_level());
    }

    /**
     * jobType
     *
     * @method jobType
     * @memberof faker.name
     */
    fn jobType(&self) -> String {
        return thread_rng().choose(self.faker.name_title_job());
    }

}
