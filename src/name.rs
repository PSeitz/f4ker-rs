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

// trait RandArray {
//     fn rand(&self) -> String;
// }

// impl RandArray for &'static [&'static str] {
//     fn rand(&self) -> String {
//         thread_rng().choose(&self).unwrap().to_string()
//     }
// }

// impl RandArray for Option<&'static [&'static str]> {
//     fn rand(&self) -> String {
//         if let Some(arr) = self {
//             return thread_rng().choose(&arr).unwrap().to_string();
//         }
//         "".to_string()
//     }
// }

trait RandArray {
    fn rand(&self) -> &str;
}
impl RandArray for &'static [&'static str] {
    fn rand(&self) -> &str {
        thread_rng().choose(&self).unwrap()
    }
}

impl RandArray for Option<&'static [&'static str]> {
    fn rand(&self) -> &str {
        if let Some(arr) = self {
            return thread_rng().choose(&arr).unwrap();
        }
        ""
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

    pub(crate) fn first_name(&self, gender: Option<Gender>) -> &'a str {
        let gender = gender.unwrap_or_else(|| thread_rng().choose(&GENDERS).cloned().unwrap());

        if let (Some(_name_male_first_name), Some(_name_female_first_name)) =
            (self.faker.name_male_first_name, self.faker.name_female_first_name)
        {
            // some locale datasets ( like ru ) have first_name split by gender. since the name.first_name field does not exist in these datasets,
            // we must randomly pick a name from either gender array so faker.name.first_name will return the correct locale data ( and not fallback )
            if gender == Gender::Male {
                return self.faker.name_male_first_name.rand();
            } else {
                return self.faker.name_female_first_name.rand();
            }
        }
        return self.faker.name_first_name.rand();
    }

    pub(crate) fn last_name(&self, gender: Option<Gender>) -> &'a str {
        let gender = gender.unwrap_or_else(|| thread_rng().choose(&GENDERS).cloned().unwrap());
        if self.faker.name_male_last_name.is_some() && self.faker.name_female_last_name.is_some() {
            if gender == Gender::Male {
                return self.faker.name_male_last_name.rand();
            } else {
                return self.faker.name_female_last_name.rand();
            }
        }
        return self.faker.name_last_name.rand();
    }

    pub(crate) fn middle_name(&self, gender: Option<Gender>) -> &'a str {
        let gender = gender.unwrap_or_else(|| thread_rng().choose(&GENDERS).cloned().unwrap());
        if self.faker.name_male_middle_name.is_some() && self.faker.name_female_middle_name.is_some() {
            if gender == Gender::Male {
                return self.faker.name_male_middle_name.rand();
            } else {
                return self.faker.name_female_middle_name.rand();
            }
        }
        panic!("no middle name found");
        // return self.faker.name_middle_name.rand();
    }

    pub(crate) fn find_name(&self) -> String {
        let gender = thread_rng().choose(&GENDERS).cloned().unwrap();
        let r = thread_rng().gen_range(0, 8);
        // in particular locales first and last names split by gender,
        // thus we keep consistency by passing 0 as male and 1 as female
        let first_name = self.first_name(Some(gender));
        let last_name = self.last_name(Some(gender));
        match r {
            0 => self.prefix(Some(gender)).to_owned() + " " + &first_name + " " + &last_name,
            1 => first_name.to_owned() + " " + &last_name + " " + &self.suffix(),
            _ => first_name.to_owned() + " " + &last_name,
        }
    }

    pub(crate) fn job_title(&self) -> String {
        return self.job_descriptor().to_owned() + " " + &self.job_area() + " " + &self.job_type();
    }

    pub(crate) fn prefix(&self, gender: Option<Gender>) -> &'a str {
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

    pub(crate) fn suffix(&self) -> &'a str {
        return self.faker.name_suffix.rand();
    }

    pub(crate) fn title(&self) -> String {
        let descriptor = self.faker.name_title_descriptor.rand();
        let level = self.faker.name_title_level.rand();
        let job = self.faker.name_title_job.rand();

        return descriptor.to_string() + " " + &level + " " + &job;
    }

    pub(crate) fn job_descriptor(&self) -> &'a str {
        return self.faker.name_title_descriptor.rand();
    }

    pub(crate) fn job_area(&self) -> &'a str {
        return self.faker.name_title_level.rand();
    }

    pub(crate) fn job_type(&self) -> &'a str {
        return self.faker.name_title_job.rand();
    }
}
