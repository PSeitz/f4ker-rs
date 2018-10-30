use crate::faker::Faker;
use rand::{thread_rng, Rng};
#[derive(Debug, Clone)]
pub struct Name<'a> {
    faker: &'a Faker,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Gender {
    Male,
    Female,
}

const GENDERS: [Gender; 2] = [Gender::Male, Gender::Female];

trait RandArray {
    fn rand(&self) -> String;
}

impl RandArray for &'static [&'static str] {
    fn rand(&self) -> String {
        thread_rng().choose(&self).unwrap().to_string()
    }
}

impl RandArray for Option<&'static [&'static str]> {
    fn rand(&self) -> String {
        if let Some(arr) = self {
            return thread_rng().choose(&arr).unwrap().to_string();
        }
        "".to_string()
    }
}

#[test]
fn name() {
    let facker = Faker::new();
    let name = Name::new(&facker);
    println!("{:?}", name.find_name());
    println!("{:?}", Faker::new().name().find_name());
}

impl<'a> Name<'a> {
    pub(crate) fn new(faker: &'a Faker) -> Self {
        Name { faker }
    }

    pub(crate) fn first_name(&self, gender: Option<Gender>) -> String {
        let gender = gender.unwrap_or_else(|| thread_rng().choose(&GENDERS).cloned().unwrap());

        if let (Some(name_male_first_name), Some(name_female_first_name)) =
            (self.faker.name_male_first_name, self.faker.name_female_first_name)
        {
            // if (self.faker.name_male_first_name.is_some() && self.faker.name_female_first_name.is_some()) {
            // some locale datasets ( like ru ) have first_name split by gender. since the name.first_name field does not exist in these datasets,
            // we must randomly pick a name from either gender array so faker.name.first_name will return the correct locale data ( and not fallback )
            if gender == Gender::Male {
                return name_male_first_name.rand();
            } else {
                return name_female_first_name.rand();
            }
        }
        return self.faker.name_first_name.rand();
    }

    pub(crate) fn last_name(&self, gender: Option<Gender>) -> String {
        let gender = gender.unwrap_or_else(|| thread_rng().choose(&GENDERS).cloned().unwrap());
        if self.faker.name_male_last_name.is_some() && self.faker.name_female_last_name.is_some() {
            // some locale datasets ( like ru ) have last_name split by gender. i have no idea how last names can have GENDERS, but also i do not speak russian
            // see above comment of first_name method
            if gender == Gender::Male {
                return self.faker.name_male_last_name.rand();
            } else {
                return self.faker.name_female_last_name.rand();
            }
        }
        return self.faker.name_last_name.rand();
    }

    pub(crate) fn find_name(&self) -> String {
        let gender = thread_rng().choose(&GENDERS).cloned().unwrap();
        let r = thread_rng().gen_range(0, 8);
        // in particular locales first and last names split by gender,
        // thus we keep consistency by passing 0 as male and 1 as female
        let first_name = self.first_name(Some(gender));
        let last_name = self.last_name(Some(gender));
        match r {
            0 => self.prefix(Some(gender)) + " " + &first_name + " " + &last_name,
            1 => first_name + " " + &last_name + " " + &self.suffix(),
            _ => first_name + " " + &last_name,
        }
    }

    pub(crate) fn job_title(&self) -> String {
        return self.job_descriptor() + " " + &self.job_area() + " " + &self.job_type();
    }

    pub(crate) fn prefix(&self, gender: Option<Gender>) -> String {
        let gender = gender.unwrap_or_else(|| thread_rng().choose(&GENDERS).cloned().unwrap());
        if self.faker.name_male_prefix.is_some() && self.faker.name_female_prefix.is_some() {
            if gender == Gender::Male {
                return self.faker.name_male_prefix.rand();
            } else {
                return self.faker.name_female_prefix.rand();
            }
        }
        return self.faker.name_prefix.rand();
    }

    pub(crate) fn suffix(&self) -> String {
        return self.faker.name_suffix.rand();
    }

    pub(crate) fn title(&self) -> String {
        let descriptor = self.faker.name_title_descriptor.rand();
        let level = self.faker.name_title_level.rand();
        let job = self.faker.name_title_job.rand();

        return descriptor.to_string() + " " + &level + " " + &job;
    }

    pub(crate) fn job_descriptor(&self) -> String {
        return self.faker.name_title_descriptor.rand();
    }

    pub(crate) fn job_area(&self) -> String {
        return self.faker.name_title_level.rand();
    }

    pub(crate) fn job_type(&self) -> String {
        return self.faker.name_title_job.rand();
    }
}
