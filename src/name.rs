use crate::faker::Faker;
use crate::*;
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

#[test]
fn name() {
    let facker = Faker::new();
    let name = Name::new(&facker);
    // println!("{:?}", name.full_name());
    // println!("JOB TITLE: {:?}", name.job_title());
    // println!("{:?}", Faker::new().name().full_name());
    // println!("first_name: {:?}", name.first_name(None));
    // println!("last_name: {:?}", name.last_name(None));
    // println!("middle_name: {:?}", name.middle_name(None));
    // println!("full_name: {:?}", name.full_name());
    // println!("prefix: {:?}", name.prefix(None));
    // println!("job_title: {:?}", name.job_title());
    // println!("suffix: {:?}", name.suffix());
    // println!("job_descriptor: {:?}", name.job_descriptor());
    // println!("job_area: {:?}", name.job_area());
    // println!("job_type: {:?}", name.job_type());
}

impl<'a> Name<'a> {
    pub fn new(faker: &'a Faker) -> Self {
        Name { faker }
    }

    /// e.g. "Miriam"
    pub fn first_name(&self, gender: Option<Gender>) -> &'static str {
        let gender = gender.unwrap_or_else(|| rand_cloned!(GENDERS));

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

    /// e.g. "Rowe"
    pub fn last_name(&self, gender: Option<Gender>) -> &'static str {
        let gender = gender.unwrap_or_else(|| rand_cloned!(GENDERS));
        if self.faker.name_male_last_name.is_some() && self.faker.name_female_last_name.is_some() {
            if gender == Gender::Male {
                return self.faker.name_male_last_name.rand();
            } else {
                return self.faker.name_female_last_name.rand();
            }
        }
        return self.faker.name_last_name.rand();
    }

    /// returns empty string if middle names don't exist in the locale
    pub fn middle_name(&self, gender: Option<Gender>) -> &'static str {
        let gender = gender.unwrap_or_else(|| rand_cloned!(GENDERS));
        if self.faker.name_male_middle_name.is_some() && self.faker.name_female_middle_name.is_some() {
            if gender == Gender::Male {
                return self.faker.name_male_middle_name.rand();
            } else {
                return self.faker.name_female_middle_name.rand();
            }
        }
        ""
    }

    /// e.g. "Miss Janet Dare"
    pub fn full_name(&self) -> String {
        let gender = rand_cloned!(GENDERS);
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

    /// e.g. "Dr."
    pub fn prefix(&self, gender: Option<Gender>) -> &'static str {
        let gender = gender.unwrap_or_else(|| rand_cloned!(GENDERS));
        if self.faker.name_male_prefix.is_some() && self.faker.name_female_prefix.is_some() {
            if gender == Gender::Male {
                return self.faker.name_male_prefix.rand();
            } else {
                return self.faker.name_female_prefix.rand();
            }
        }
        return self.faker.name_prefix.rand();
    }

    /// generates a title via job_descriptor +  job_area + job_type
    /// e.g. Central Data Administrator
    /// e.g. "Principal Metrics Developer"
    pub fn job_title(&self) -> String {
        return self.job_descriptor().to_owned() + " " + self.job_area() + " " + self.job_type();
    }

    /// e.g. "I"
    pub fn suffix(&self) -> &'static str {
        return self.faker.name_suffix.rand();
    }

    /// e.g. "Corporate"
    pub fn job_descriptor(&self) -> &'static str {
        return self.faker.name_title_descriptor.rand();
    }

    /// e.g. "Interactions"
    pub fn job_area(&self) -> &'static str {
        return self.faker.name_title_level.rand();
    }

    /// e.g. "Director"
    pub fn job_type(&self) -> &'static str {
        return self.faker.name_title_job.rand();
    }
}
