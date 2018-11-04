use crate::locales;
use crate::name;
use crate::*;
use rand::{thread_rng, Rng};
use std::borrow::Cow;
#[derive(Debug, Clone)]
pub struct Faker {
    pub(crate) phone_number_formats: Option<&'static [&'static str]>,
    pub(crate) phone_number_exchange_code: Option<&'static [&'static str]>,
    pub(crate) phone_number_area_code: Option<&'static [&'static str]>,
    pub(crate) name_male_first_name: Option<&'static [&'static str]>,
    pub(crate) name_masculine_name: Option<&'static [&'static str]>,
    pub(crate) name_feminine_name: Option<&'static [&'static str]>,
    pub(crate) name_male_prefix: Option<&'static [&'static str]>,
    pub(crate) name_title_level: Option<&'static [&'static str]>,
    pub(crate) name_last_name: Option<&'static [&'static str]>,
    pub(crate) name_first_name_men: Option<&'static [&'static str]>,
    pub(crate) name_gender: Option<&'static [&'static str]>,
    pub(crate) name_male_middle_name: Option<&'static [&'static str]>,
    pub(crate) name_title_descriptor: Option<&'static [&'static str]>,
    pub(crate) name_male_title: Option<&'static [&'static str]>,
    pub(crate) name_female_last_name: Option<&'static [&'static str]>,
    pub(crate) name_name: Option<&'static [&'static str]>,
    pub(crate) name_female_prefix: Option<&'static [&'static str]>,
    pub(crate) name_ocker_first_name: Option<&'static [&'static str]>,
    pub(crate) name_suffix: Option<&'static [&'static str]>,
    pub(crate) name_nobility_title_prefix: Option<&'static [&'static str]>,
    pub(crate) name_male_last_name: Option<&'static [&'static str]>,
    pub(crate) name_title_job: Option<&'static [&'static str]>,
    pub(crate) name_female_first_name: Option<&'static [&'static str]>,
    pub(crate) name_female_middle_name: Option<&'static [&'static str]>,
    pub(crate) name_tussenvoegsel: Option<&'static [&'static str]>,
    pub(crate) name_prefix: Option<&'static [&'static str]>,
    pub(crate) name_first_name: Option<&'static [&'static str]>,
    pub(crate) name_first_name_women: Option<&'static [&'static str]>,
    pub(crate) name_female_title: Option<&'static [&'static str]>,
    pub(crate) company_prefix: Option<&'static [&'static str]>,
    pub(crate) company_name: Option<&'static [&'static str]>,
    pub(crate) company_noun: Option<&'static [&'static str]>,
    pub(crate) company_bs_noun: Option<&'static [&'static str]>,
    pub(crate) company_suffix: Option<&'static [&'static str]>,
    pub(crate) company_legal_form: Option<&'static [&'static str]>,
    pub(crate) company_bs_verb: Option<&'static [&'static str]>,
    pub(crate) company_adjective: Option<&'static [&'static str]>,
    pub(crate) company_adjetive: Option<&'static [&'static str]>,
    pub(crate) company_descriptor: Option<&'static [&'static str]>,
    pub(crate) company_bs_adjective: Option<&'static [&'static str]>,
    pub(crate) cell_phone_common_cell_prefix: Option<&'static [&'static str]>,
    pub(crate) cell_phone_formats: Option<&'static [&'static str]>,
    pub(crate) business_credit_card_expiry_dates: Option<&'static [&'static str]>,
    pub(crate) business_credit_card_numbers: Option<&'static [&'static str]>,
    pub(crate) business_credit_card_types: Option<&'static [&'static str]>,
    pub(crate) hacker_adjective: Option<&'static [&'static str]>,
    pub(crate) hacker_abbreviation: Option<&'static [&'static str]>,
    pub(crate) hacker_phrase: Option<&'static [&'static str]>,
    pub(crate) hacker_ingverb: Option<&'static [&'static str]>,
    pub(crate) hacker_noun: Option<&'static [&'static str]>,
    pub(crate) hacker_verb: Option<&'static [&'static str]>,
    pub(crate) internet_free_email: Option<&'static [&'static str]>,
    pub(crate) internet_example_email: Option<&'static [&'static str]>,
    pub(crate) internet_avatar_uri: Option<&'static [&'static str]>,
    pub(crate) internet_domain_suffix: Option<&'static [&'static str]>,
    pub(crate) team_name: Option<&'static [&'static str]>,
    pub(crate) team_creature: Option<&'static [&'static str]>,
    pub(crate) team_suffix: Option<&'static [&'static str]>,
    pub(crate) finance_account_type: Option<&'static [&'static str]>,
    pub(crate) finance_transaction_type: Option<&'static [&'static str]>,
    pub(crate) lorem_words: Option<&'static [&'static str]>,
    pub(crate) lorem_supplemental: Option<&'static [&'static str]>,
    pub(crate) commerce_color: Option<&'static [&'static str]>,
    pub(crate) commerce_department: Option<&'static [&'static str]>,
    pub(crate) commerce_product_name_adjective: Option<&'static [&'static str]>,
    pub(crate) commerce_product_name_product: Option<&'static [&'static str]>,
    pub(crate) commerce_product_name_material: Option<&'static [&'static str]>,
    pub(crate) date_month_wide_context: Option<&'static [&'static str]>,
    pub(crate) date_weekday_abbr: Option<&'static [&'static str]>,
    pub(crate) date_weekday_wide: Option<&'static [&'static str]>,
    pub(crate) date_month_wide: Option<&'static [&'static str]>,
    pub(crate) date_month_abbr: Option<&'static [&'static str]>,
    pub(crate) date_weekday_abbr_context: Option<&'static [&'static str]>,
    pub(crate) date_month_abbr_context: Option<&'static [&'static str]>,
    pub(crate) date_weekday_wide_context: Option<&'static [&'static str]>,
    // pub(crate) credit_card_american_express: Option<&'static [&'static str]>,
    // pub(crate) credit_card_mastercard: Option<&'static [&'static str]>,
    // pub(crate) credit_card_maestro: Option<&'static [&'static str]>,
    // pub(crate) credit_card_visa: Option<&'static [&'static str]>,
    // pub(crate) credit_card_discover: Option<&'static [&'static str]>,
    pub(crate) database_column: Option<&'static [&'static str]>,
    pub(crate) database_engine: Option<&'static [&'static str]>,
    pub(crate) database_data_type: Option<&'static [&'static str]>,
    pub(crate) database_collation: Option<&'static [&'static str]>,
    pub(crate) address_state_abbr: Option<&'static [&'static str]>,
    pub(crate) address_state: Option<&'static [&'static str]>,
    pub(crate) address_streets: Option<&'static [&'static str]>,
    pub(crate) address_city_root: Option<&'static [&'static str]>,
    pub(crate) address_city_suffix: Option<&'static [&'static str]>,
    pub(crate) address_uk_country: Option<&'static [&'static str]>,
    pub(crate) address_postcode_by_state: Option<&'static [&'static str]>,
    pub(crate) address_secondary_address: Option<&'static [&'static str]>,
    pub(crate) address_country: Option<&'static [&'static str]>,
    pub(crate) address_direction_abbr: Option<&'static [&'static str]>,
    pub(crate) address_default_country: Option<&'static [&'static str]>,
    pub(crate) address_region: Option<&'static [&'static str]>,
    pub(crate) address_street_root: Option<&'static [&'static str]>,
    pub(crate) address_common_street_suffix: Option<&'static [&'static str]>,
    pub(crate) address_province: Option<&'static [&'static str]>,
    pub(crate) address_time_zone: Option<&'static [&'static str]>,
    pub(crate) address_city_name: Option<&'static [&'static str]>,
    pub(crate) address_street_address: Option<&'static [&'static str]>,
    pub(crate) address_country_code: Option<&'static [&'static str]>,
    pub(crate) address_direction: Option<&'static [&'static str]>,
    pub(crate) address_building_number: Option<&'static [&'static str]>,
    pub(crate) address_street_prefix: Option<&'static [&'static str]>,
    pub(crate) address_postcode: Option<&'static [&'static str]>,
    pub(crate) address_city_prefix: Option<&'static [&'static str]>,
    pub(crate) address_street_suffix: Option<&'static [&'static str]>,
    pub(crate) address_city: Option<&'static [&'static str]>,
    pub(crate) address_street_title: Option<&'static [&'static str]>,
    pub(crate) address_street: Option<&'static [&'static str]>,
    pub(crate) address_street_name: Option<&'static [&'static str]>,
    pub(crate) address_county: Option<&'static [&'static str]>,
    pub(crate) title: String,
    pub(crate) separator: String,
}

macro_rules! faker_constr {
    () => {{
        Faker {
            phone_number_formats: phone_number::formats(),
            phone_number_exchange_code: phone_number::exchange_code(),
            phone_number_area_code: phone_number::area_code(),
            name_male_first_name: name::male_first_name(),
            name_masculine_name: name::masculine_name(),
            name_feminine_name: name::feminine_name(),
            name_male_prefix: name::male_prefix(),
            name_title_level: name::title_level(),
            name_last_name: name::last_name(),
            name_first_name_men: name::first_name_men(),
            name_gender: name::gender(),
            name_male_middle_name: name::male_middle_name(),
            name_title_descriptor: name::title_descriptor(),
            name_male_title: name::male_title(),
            name_female_last_name: name::female_last_name(),
            name_name: name::name(),
            name_female_prefix: name::female_prefix(),
            name_ocker_first_name: name::ocker_first_name(),
            name_suffix: name::suffix(),
            name_nobility_title_prefix: name::nobility_title_prefix(),
            name_male_last_name: name::male_last_name(),
            name_title_job: name::title_job(),
            name_female_first_name: name::female_first_name(),
            name_female_middle_name: name::female_middle_name(),
            name_tussenvoegsel: name::tussenvoegsel(),
            name_prefix: name::prefix(),
            name_first_name: name::first_name(),
            name_first_name_women: name::first_name_women(),
            name_female_title: name::female_title(),
            company_prefix: company::prefix(),
            company_name: company::name(),
            company_noun: company::noun(),
            company_bs_noun: company::bs_noun(),
            company_suffix: company::suffix(),
            company_legal_form: company::legal_form(),
            company_bs_verb: company::bs_verb(),
            company_adjective: company::adjective(),
            company_adjetive: company::adjetive(),
            company_descriptor: company::descriptor(),
            company_bs_adjective: company::bs_adjective(),
            cell_phone_common_cell_prefix: self::locales::en::cell_phone::common_cell_prefix(),
            cell_phone_formats: self::locales::en::cell_phone::formats(),
            business_credit_card_expiry_dates: self::locales::en::business::credit_card_expiry_dates(),
            business_credit_card_numbers: self::locales::en::business::credit_card_numbers(),
            business_credit_card_types: self::locales::en::business::credit_card_types(),
            hacker_adjective: self::locales::en::hacker::adjective(),
            hacker_abbreviation: self::locales::en::hacker::abbreviation(),
            hacker_phrase: self::locales::en::hacker::phrase(),
            hacker_ingverb: self::locales::en::hacker::ingverb(),
            hacker_noun: self::locales::en::hacker::noun(),
            hacker_verb: self::locales::en::hacker::verb(),
            internet_free_email: internet::free_email(),
            internet_example_email: internet::example_email(),
            internet_avatar_uri: internet::avatar_uri(),
            internet_domain_suffix: internet::domain_suffix(),
            team_name: self::locales::en::team::name(),
            team_creature: self::locales::en::team::creature(),
            team_suffix: self::locales::en::team::suffix(),
            finance_account_type: self::locales::en::finance::account_type(),
            finance_transaction_type: self::locales::en::finance::transaction_type(),
            lorem_words: self::locales::en::lorem::words(),
            lorem_supplemental: self::locales::en::lorem::supplemental(),
            commerce_color: commerce::color(),
            commerce_department: commerce::department(),
            commerce_product_name_adjective: commerce::product_name_adjective(),
            commerce_product_name_product: commerce::product_name_product(),
            commerce_product_name_material: commerce::product_name_material(),
            date_month_wide_context: date::month_wide_context(),
            date_weekday_abbr: date::weekday_abbr(),
            date_weekday_wide: date::weekday_wide(),
            date_month_wide: date::month_wide(),
            date_month_abbr: date::month_abbr(),
            date_weekday_abbr_context: date::weekday_abbr_context(),
            date_month_abbr_context: date::month_abbr_context(),
            date_weekday_wide_context: date::weekday_wide_context(),
            // credit_card_american_express: credit_card::american_express(),
            // credit_card_mastercard: credit_card::mastercard(),
            // credit_card_maestro: credit_card::maestro(),
            // credit_card_visa: credit_card::visa(),
            // credit_card_discover: credit_card::discover(),
            database_column: self::locales::en::database::column(),
            database_engine: self::locales::en::database::engine(),
            database_data_type: self::locales::en::database::data_type(),
            database_collation: self::locales::en::database::collation(),
            address_state_abbr: address::state_abbr(),
            address_state: address::state(),
            address_streets: address::streets(),
            address_city_root: address::city_root(),
            address_city_suffix: address::city_suffix(),
            address_uk_country: address::uk_country(),
            address_postcode_by_state: address::postcode_by_state(),
            address_secondary_address: address::secondary_address(),
            address_country: address::country(),
            address_direction_abbr: address::direction_abbr(),
            address_default_country: address::default_country(),
            address_region: address::region(),
            address_street_root: address::street_root(),
            address_common_street_suffix: address::common_street_suffix(),
            address_province: address::province(),
            address_time_zone: address::time_zone(),
            address_city_name: address::city_name(),
            address_street_address: address::street_address(),
            address_country_code: address::country_code(),
            address_direction: address::direction(),
            address_building_number: address::building_number(),
            address_street_prefix: address::street_prefix(),
            address_postcode: address::postcode(),
            address_city_prefix: address::city_prefix(),
            address_street_suffix: address::street_suffix(),
            address_city: address::city(),
            address_street_title: address::street_title(),
            address_street: address::street(),
            address_street_name: address::street_name(),
            address_county: address::county(),
            title: "".to_string(),
            separator: "".to_string(),
        }
    }};
}

macro_rules! new_faker {
    ($constr_name:ident, $locale:ident) => (

        pub fn $constr_name() -> Self {
            use self::locales::$locale::*;

            faker_constr!()
        }
    )
}

#[test]
fn interpol_fake() {
    let facker = Faker::new();
    println!(
        "interpol_fake {:?}",
        facker.fake("{{name.last_name}}, {{name.first_name}} {{name.suffix}}")
    );
}

impl Faker {
    pub fn new() -> Self {
        use self::locales::en::*;

        faker_constr!()
    }

    new_faker!(new_en, en);


    // new_faker!(new_az, az);
    // new_faker!(new_ar, ar);
    // new_faker!(new_cz, cz);
    // new_faker!(new_de, de);
    // new_faker!(new_de_at, de_at);
    // new_faker!(new_de_ch, de_ch);
    // new_faker!(new_en, en);
    // new_faker!(new_en_au, en_au);
    // new_faker!(new_en_bork, en_bork);
    // new_faker!(new_en_ca, en_ca);
    // new_faker!(new_en_gb, en_gb);
    // new_faker!(new_en_ie, en_ie);
    // new_faker!(new_en_ind, en_ind);
    // new_faker!(new_en_us, en_us);
    // new_faker!(new_en_za, en_za);
    // new_faker!(new_en_au_ocker, en_au_ocker);
    // new_faker!(new_es, es);
    // new_faker!(new_es_mx, es_mx);
    // new_faker!(new_fa, fa);
    // new_faker!(new_fr, fr);
    // new_faker!(new_fr_ca, fr_ca);
    // new_faker!(new_fr_ch, fr_ch);
    // new_faker!(new_ge, ge);
    // new_faker!(new_id_id, id_id);
    // new_faker!(new_it, it);
    // new_faker!(new_ja, ja);
    // new_faker!(new_ko, ko);
    // new_faker!(new_nb_no, nb_no);
    // new_faker!(new_nep, nep);
    // new_faker!(new_nl_be, nl_be);
    // new_faker!(new_nl, nl);
    // new_faker!(new_pl, pl);
    // new_faker!(new_pt_br, pt_br);
    // new_faker!(new_pt_pt, pt_pt);
    // new_faker!(new_ro, ro);
    // new_faker!(new_ru, ru);
    // new_faker!(new_sk, sk);
    // new_faker!(new_sv, sv);
    // new_faker!(new_tr, tr);
    // new_faker!(new_uk, uk);
    // new_faker!(new_vi, vi);
    // new_faker!(new_zh_cn, zh_cn);
    // new_faker!(new_zh_tw, zh_tw);

    // pub fn fake2(&self, templ: &str) -> String {
    //     let mut templ = templ.to_string();
    //     while let Some(start_bytes) = templ.find("{{") {
    //         if let Some(end) = &templ[start_bytes..].find("}}") {
    //             let pattern = &templ[start_bytes + 2..start_bytes + end];
    //             templ.replace_range(start_bytes..start_bytes + end + 2, &self.from_pattern(pattern));
    //         }
    //     }
    //     templ
    // }

    /// This will interpolate the format string with the value of methods
    /// '{{name.last_name}}, {{name.first_name}} {{name.suffix}}'
    pub fn fake(&self, templ: &str) -> String {
        let iter_pairs = templ
            .rmatch_indices("{{")
            .map(|el| el.0)
            .zip(templ.rmatch_indices("}}").map(|el| el.0));

        let mut oge = templ.to_string();
        for pair in iter_pairs {
            let pattern = &templ[pair.0 + 2..pair.1];
            oge.replace_range(pair.0..pair.1 + 2, &self.from_pattern(pattern));
        }
        oge
    }

    fn from_pattern(&self, pat: &str) -> Cow<'_, str> {
        match pat {
            "name.full_name" => Cow::from(self.name().full_name()),
            "name.first_name" => Cow::from(self.name().first_name(None)),
            "name.last_name" => Cow::from(self.name().last_name(None)),
            "name.prefix" => Cow::from(self.name().prefix(None)),
            "name.suffix" => Cow::from(self.name().suffix()),
            "name.job_title" => Cow::from(self.name().job_title()),
            "name.male_prefix" => Cow::from(self.name().prefix(Some(name::Gender::Male))),
            "name.female_prefix" => Cow::from(self.name().prefix(Some(name::Gender::Female))),
            "name.male_first_name" => Cow::from(self.name().first_name(Some(name::Gender::Male))),
            "name.female_first_name" => Cow::from(self.name().first_name(Some(name::Gender::Female))),
            "name.male_middle_name" => Cow::from(self.name().middle_name(Some(name::Gender::Male))),
            "name.female_middle_name" => Cow::from(self.name().middle_name(Some(name::Gender::Female))),
            "name.male_last_name" => Cow::from(self.name().last_name(Some(name::Gender::Male))),
            "name.female_last_name" => Cow::from(self.name().last_name(Some(name::Gender::Female))),
            "address.city_prefix" => Cow::from(self.address().city_prefix()),
            "address.city_suffix" => Cow::from(self.address().city_suffix()),
            "address.street_suffix" => Cow::from(self.address().street_suffix()),
            "address.county" => Cow::from(self.address().county()),
            "address.country" => Cow::from(self.address().country()),
            "address.country_code" => Cow::from(self.address().country_code()),
            "address.state" => Cow::from(self.address().state()),
            "address.state_abbr" => Cow::from(self.address().state_abbr()),
            "address.street_prefix" => Cow::from(self.address().street_prefix()),
            "address.zip_code" => Cow::from(self.address().zip_code(None)),
            // "address.postcode_by_state" => {Cow::from(self.address().postcode_by_state())},
            "address.direction" => Cow::from(self.address().direction()),
            "address.direction_abbr" => Cow::from(self.address().direction_abbr()),
            "company.adjective" => Cow::from(self.company().catch_phrase_adjective()),
            "company.noun" => Cow::from(self.company().catch_phrase_noun()),
            "company.descriptor" => Cow::from(self.company().catch_phrase_descriptor()),
            "company.bs" => Cow::from(self.company().bs()),
            "company.bs_adjective" => Cow::from(self.company().bs_adjective()),
            "company.bs_noun" => Cow::from(self.company().bs_noun()),
            "company.bs_verb" => Cow::from(self.company().bs_buzz()),
            "company.suffix" => Cow::from(self.company().company_suffix()),
            // "lorem.words" => {},
            // "hacker.abbreviation" => {},
            // "hacker.adjective" => {},
            // "hacker.noun" => {},
            // "hacker.verb" => {},
            // "hacker.ingverb" => {},
            // "hacker.phrase" => {},
            // "phone_number.formats" => {},
            // "finance.account_type" => {},
            // "finance.transaction_type" => {},
            // "finance.currency" => {},
            // "finance.iban" => {},
            // "finance.credit_card" => {},
            // "internet.avatar_uri" => {},
            // "internet.domain_suffix" => {},
            // "internet.free_email" => {},
            // "internet.example_email" => {},
            // "internet.password" => {},
            // "commerce.color" => {},
            // "commerce.department" => {},
            // "commerce.product_name" => {},
            // "commerce.price" => {},
            // "commerce.categories" => {},
            // "database.collation" => {},
            // "database.column" => {},
            // "database.engine" => {},
            // "database.data_type" => {},
            // "system.mimeTypes" => {},
            // "system.directoryPaths" => {},
            // "date.month" => {},
            // "date.weekday" => {},
            // "title": => {},
            // "separator": {},
            _ => panic!("{:?} not recognized", pat),
        }
    }

    pub fn name(&self) -> crate::Name {
        crate::Name::new(self)
    }

    pub fn address(&self) -> crate::Address {
        crate::Address::new(self)
    }

    pub fn company(&self) -> crate::Company {
        crate::Company::new(self)
    }
}

/*

   this index.js file is used for including the faker library as a CommonJS module, instead of a bundle

   you can include the faker library into your existing node.js application by requiring the entire /faker directory

    let faker = require(./faker);
    let randomName = faker.name.findName();

   you can also simply include the "faker.js" file which is the auto-generated bundled version of the faker library

    let faker = require(./customAppPath/faker);
    let randomName = faker.name.findName();


  if you plan on modifying the faker library you should be performing your changes in the /lib/ directory

*/

// /**
//  *
//  * @namespace faker
//  */
// impl Faker {
//     fn new() -> Self {

//     }

//   opts = opts || {};

//   // assign options
//   let locales = self.locales || opts.locales || {};
//   let locale = self.locale || opts.locale || "en";
//   let localeFallback = self.localeFallback || opts.localeFallback || "en";

//   self.locales = locales;
//   self.locale = locale;
//   self.localeFallback = localeFallback;

//   self.definitions = {};

//   function bindAll(obj) {
//       Object.keys(obj).forEach(function(meth) {
//           if (typeof obj[meth] == 'function') {
//               obj[meth] = obj[meth].bind(obj);
//           }
//       });
//       return obj;
//   }

//   let Fake = require('./fake');
//   self.fake = new Fake(self).fake;

//   let Unique = require('./unique');
//   self.unique = bindAll(new Unique(self).unique);

//   let Random = require('./random');
//   self.random = bindAll(new Random(self));

//   let Helpers = require('./helpers');
//   self.helpers = new Helpers(self);

//   let Name = require('./name');
//   self.name = bindAll(new Name(self));

//   let Address = require('./address');
//   self.address = bindAll(new Address(self));

//   let Company = require('./company');
//   self.company = bindAll(new Company(self));

//   let Finance = require('./finance');
//   self.finance = bindAll(new Finance(self));

//   let Image = require('./image');
//   self.image = bindAll(new Image(self));

//   let Lorem = require('./lorem');
//   self.lorem = bindAll(new Lorem(self));

//   let Hacker = require('./hacker');
//   self.hacker = bindAll(new Hacker(self));

//   let Internet = require('./internet');
//   self.internet = bindAll(new Internet(self));

//   let Database = require('./database');
//   self.database = bindAll(new Database(self));

//   let Phone = require('./phone_number');
//   self.phone = bindAll(new Phone(self));

//   let _Date = require('./date');
//   self.date = bindAll(new _Date(self));

//   let Commerce = require('./commerce');
//   self.commerce = bindAll(new Commerce(self));

//   let System = require('./system');
//   self.system = bindAll(new System(self));

//   let Git = require('./git');
//   self.git = bindAll(new Git(self));

//   let _definitions = {
//     // "name": ["first_name", "last_name", "prefix", "suffix", "gender", "title", "male_prefix", "female_prefix", "male_first_name", "female_first_name", "male_middle_name", "female_middle_name", "male_last_name", "female_last_name"],
//     "name.first_name",
//     "name.last_name",
//     "name.prefix",
//     "name.suffix",
//     "name.gender",
//     "name.title",
//     "name.male_prefix",
//     "name.female_prefix",
//     "name.male_first_name",
//     "name.female_first_name",
//     "name.male_middle_name",
//     "name.female_middle_name",
//     "name.male_last_name",
//     "name.female_last_name",
//     // "address": ["city_prefix", "city_suffix", "street_suffix", "county", "country", "country_code", "state", "state_abbr", "street_prefix", "postcode", "postcode_by_state", "direction", "direction_abbr"],
//     "address.city_prefix",
//     "address.city_suffix",
//     "address.street_suffix",
//     "address.county",
//     "address.country",
//     "address.country_code",
//     "address.state",
//     "address.state_abbr",
//     "address.street_prefix",
//     "address.postcode",
//     "address.postcode_by_state",
//     "address.direction",
//     "address.direction_abbr",
//     // "company": ["adjective", "noun", "descriptor", "bs_adjective", "bs_noun", "bs_verb", "suffix"],
//     "company.adjective",
//     "company.noun",
//     "company.descriptor",
//     "company.bs_adjective",
//     "company.bs_noun",
//     "company.bs_verb",
//     "company.suffix",
//     // "lorem": ["words"],
//     "lorem.words",
//     // "hacker": ["abbreviation", "adjective", "noun", "verb", "ingverb", "phrase"],
//     "hacker.abbreviation",
//     "hacker.adjective",
//     "hacker.noun",
//     "hacker.verb",
//     "hacker.ingverb",
//     "hacker.phrase",
//     // "phone_number": ["formats"],
//     "phone_number.formats",
//     // "finance": ["account_type", "transaction_type", "currency", "iban", "credit_card"],
//     "finance.account_type",
//     "finance.transaction_type",
//     "finance.currency",
//     "finance.iban",
//     "finance.credit_card",
//     // "internet": ["avatar_uri", "domain_suffix", "free_email", "example_email", "password"],
//     "internet.avatar_uri",
//     "internet.domain_suffix",
//     "internet.free_email",
//     "internet.example_email",
//     "internet.password",
//     // "commerce": ["color", "department", "product_name", "price", "categories"],
//     "commerce.color",
//     "commerce.department",
//     "commerce.product_name",
//     "commerce.price",
//     "commerce.categories",
//     // "database": ["collation", "column", "engine", "type"],
//     "database.collation",
//     "database.column",
//     "database.engine",
//     "database.type",
//     // "system": ["mimeTypes", "directoryPaths"],
//     "system.mimeTypes",
//     "system.directoryPaths",
//     // "date": ["month", "weekday"],
//     "date.month",
//     "date.weekday",
//     "title": "",
//     "separator": ""
//   };

//   // Create a Getter for all definitions.foo.bar properties
//   Object.keys(_definitions).forEach(function(d){
//     if (self.definitions[d].is_none()) {
//       self.definitions[d] = {};
//     }

//     if (typeof _definitions[d] == "string") {
//         self.definitions[d] = _definitions[d];
//       return;
//     }

//     _definitions[d].forEach(function(p){
//       Object.defineProperty(self.definitions[d], p, {
//         get: function () {
//           if (self.locales[self.locale][d].is_none() || self.locales[self.locale][d][p].is_none()) {
//             // certain localization sets contain less data then others.
//             // in the case of a missing definition, use the default localeFallback to substitute the missing set data
//             // throw new Error('unknown property ' + d + p)
//             return self.locales[localeFallback][d][p];
//           } else {
//             // return localized data
//             return self.locales[self.locale][d][p];
//           }
//         }
//       });
//     });
//   });

// };

// Faker.prototype.setLocale = function (locale) {
//   this.locale = locale;
// }

// Faker.prototype.seed = function(value) {
//   let Random = require('./random');
//   this.seedValue = value;
//   this.random = new Random(this, this.seedValue);
// }
