use crate::locales;

struct Faker {
    name_first_name: Option<&'static [&'static str]>,
    name_last_name: Option<&'static [&'static str]>,
    name_prefix: Option<&'static [&'static str]>,
    name_suffix: Option<&'static [&'static str]>,
    name_gender: Option<&'static [&'static str]>,
    name_title: Option<&'static [&'static str]>,
    name_male_prefix: Option<&'static [&'static str]>,
    name_female_prefix: Option<&'static [&'static str]>,
    name_male_first_name: Option<&'static [&'static str]>,
    name_female_first_name: Option<&'static [&'static str]>,
    // name_male_middle_name: Option<&'static [&'static str]>,
    // name_female_middle_name: Option<&'static [&'static str]>,
    // name_male_last_name: Option<&'static [&'static str]>,
    // name_female_last_name: Option<&'static [&'static str]>,
    // address_city_prefix: Option<&'static [&'static str]>,
    // address_city_suffix: Option<&'static [&'static str]>,
    // address_street_suffix: Option<&'static [&'static str]>,
    // address_county: Option<&'static [&'static str]>,
    // address_country: Option<&'static [&'static str]>,
    // address_country_code: Option<&'static [&'static str]>,
    // address_state: Option<&'static [&'static str]>,
    // address_state_abbr: Option<&'static [&'static str]>,
    // address_street_prefix: Option<&'static [&'static str]>,
    // address_postcode: Option<&'static [&'static str]>,
    // address_postcode_by_state: Option<&'static [&'static str]>,
    // address_direction: Option<&'static [&'static str]>,
    // address_direction_abbr: Option<&'static [&'static str]>,
    // company_adjective: Option<&'static [&'static str]>,
    // company_noun: Option<&'static [&'static str]>,
    // company_descriptor: Option<&'static [&'static str]>,
    // company_bs_adjective: Option<&'static [&'static str]>,
    // company_bs_noun: Option<&'static [&'static str]>,
    // company_bs_verb: Option<&'static [&'static str]>,
    // company_suffix: Option<&'static [&'static str]>,
    // lorem_words: Option<&'static [&'static str]>,
    // hacker_abbreviation: Option<&'static [&'static str]>,
    // hacker_adjective: Option<&'static [&'static str]>,
    // hacker_noun: Option<&'static [&'static str]>,
    // hacker_verb: Option<&'static [&'static str]>,
    // hacker_ingverb: Option<&'static [&'static str]>,
    // hacker_phrase: Option<&'static [&'static str]>,
    // phone_number_formats: Option<&'static [&'static str]>,
    // finance_account_type: Option<&'static [&'static str]>,
    // finance_transaction_type: Option<&'static [&'static str]>,
    // finance_currency: Option<&'static [&'static str]>,
    // finance_iban: Option<&'static [&'static str]>,
    // finance_credit_card: Option<&'static [&'static str]>,
    // internet_avatar_uri: Option<&'static [&'static str]>,
    // internet_domain_suffix: Option<&'static [&'static str]>,
    // internet_free_email: Option<&'static [&'static str]>,
    // internet_example_email: Option<&'static [&'static str]>,
    // internet_password: Option<&'static [&'static str]>,
    // commerce_color: Option<&'static [&'static str]>,
    // commerce_department: Option<&'static [&'static str]>,
    // commerce_product_name: Option<&'static [&'static str]>,
    // commerce_price: Option<&'static [&'static str]>,
    // commerce_categories: Option<&'static [&'static str]>,
    // database_collation: Option<&'static [&'static str]>,
    // database_column: Option<&'static [&'static str]>,
    // database_engine: Option<&'static [&'static str]>,
    // database_type: Option<&'static [&'static str]>,
    // system_mimeTypes: Option<&'static [&'static str]>,
    // system_directoryPaths: Option<&'static [&'static str]>,
    // date_month: Option<&'static [&'static str]>,
    // date_weekday: Option<&'static [&'static str]>,
    title: String,
    separator: String
}

impl Faker {
    fn new() -> Self {
        use self::locales::en;
        Faker {
            name_first_name: en::name::first_name(),
            name_last_name: en::name::last_name(),
            name_prefix: en::name::prefix(),
            name_suffix: en::name::suffix(),
            name_gender: en::name::gender(),
            name_title: en::name::title(),
            name_male_prefix: en::name::male_prefix(),
            name_female_prefix: en::name::female_prefix(),
            name_male_first_name: en::name::male_first_name(),
            name_female_first_name: en::name::female_first_name(),
            // name_male_middle_name: en::name::male_middle_name(),
            // name_female_middle_name: en::name::female_middle_name(),
            // name_male_last_name: en::name::male_last_name(),
            // name_female_last_name: en::name::female_last_name(),
            // address_city_prefix: en::address::city_prefix::city_prefix(),
            // address_city_suffix: en::address::city_suffix::city_suffix(),
            // address_street_suffix: en::address::street_suffix::street_suffix(),
            // address_county: en::address::county::county(),
            // address_country: en::address::country::country(),
            // address_country_code: en::address::country_code::country_code(),
            // address_state: en::address::state::state(),
            // address_state_abbr: en::address::state_abbr::state_abbr(),
            // address_street_prefix: en::address::street_prefix::street_prefix(),
            // address_postcode: en::address::postcode::postcode(),
            // address_postcode_by_state: en::address::postcode_by_state::postcode_by_state(),
            // address_direction: en::address::direction::direction(),
            // address_direction_abbr: en::address::direction_abbr::direction_abbr(),
            // company_adjective: en::company::adjective::adjective(),
            // company_noun: en::company::noun::noun(),
            // company_descriptor: en::company::descriptor::descriptor(),
            // company_bs_adjective: en::company::bs_adjective::bs_adjective(),
            // company_bs_noun: en::company::bs_noun::bs_noun(),
            // company_bs_verb: en::company::bs_verb::bs_verb(),
            // company_suffix: en::company::suffix::suffix(),
            // lorem_words: en::lorem_words::lorem_words(),
            // hacker_abbreviation: en::abbreviation::abbreviation(),
            // hacker_adjective: en::adjective::adjective(),
            // hacker_noun: en::noun::noun(),
            // hacker_verb: en::verb::verb(),
            // hacker_ingverb: en::ingverb::ingverb(),
            // hacker_phrase: en::phrase::phrase(),
            // phone_number_formats: en::phone_number_formats::phone_number_formats(),
            // finance_account_type: en::finance_account_type::finance_account_type(),
            // finance_transaction_type: en::finance_transaction_type::finance_transaction_type(),
            // finance_currency: en::finance_currency::finance_currency(),
            // finance_iban: en::finance_iban::finance_iban(),
            // finance_credit_card: en::finance_credit_card::finance_credit_card(),
            // internet_avatar_uri: en::internet_avatar_uri::internet_avatar_uri(),
            // internet_domain_suffix: en::internet_domain_suffix::internet_domain_suffix(),
            // internet_free_email: en::internet_free_email::internet_free_email(),
            // internet_example_email: en::internet_example_email::internet_example_email(),
            // internet_password: en::internet_password::internet_password(),
            // commerce_color: en::color::color(),
            // commerce_department: en::department::department(),
            // commerce_product_name: en::product_name::product_name(),
            // commerce_price: en::price::price(),
            // commerce_categories: en::categories::categories(),
            // database_collation: en::collation::collation(),
            // database_column: en::column::column(),
            // database_engine: en::engine::engine(),
            // database_type: &[](),
            // system_mimeTypes: en::system_mimeTypes(),
            // system_directoryPaths: en::system_directoryPaths(),
            // date_month: en::date_month(),
            // date_weekday: en::date_weekday(),
            title: "".to_string(),
            separator: "".to_string()
        }

    }
//     fn new_de() -> Self {
//         use self::locales::de;
//         Faker {
//             name_first_name: de::name::first_name::first_name,
//             name_last_name: de::name::last_name::last_name,
//             name_prefix: de::name::prefix::prefix,
//             name_suffix: &[],
//             name_gender: &[],
//             // name_title: de::name::title,
//             // name_male_prefix: de::name::male_prefix,
//             // name_female_prefix: de::name::female_prefix,
//             name_male_first_name: &[],
//             name_female_first_name: &[],
//             // name_male_middle_name: de::name::male_middle_name,
//             // name_female_middle_name: de::name::female_middle_name,
//             // name_male_last_name: de::name::male_last_name,
//             // name_female_last_name: de::name::female_last_name,
//             // address_city_prefix: de::address::city_prefix::city_prefix,
//             // address_city_suffix: de::address::city_suffix::city_suffix,
//             // address_street_suffix: de::address::street_suffix::street_suffix,
//             // address_county: de::address::county::county,
//             // address_country: de::address::country::country,
//             // address_country_code: de::address::country_code::country_code,
//             // address_state: de::address::state::state,
//             // address_state_abbr: de::address::state_abbr::state_abbr,
//             // address_street_prefix: de::address::street_prefix::street_prefix,
//             // address_postcode: de::address::postcode::postcode,
//             // address_postcode_by_state: de::address::postcode_by_state::postcode_by_state,
//             // address_direction: de::address::direction::direction,
//             // address_direction_abbr: de::address::direction_abbr::direction_abbr,
//             // company_adjective: de::company::adjective::adjective,
//             // company_noun: de::company::noun::noun,
//             // company_descriptor: de::company::descriptor::descriptor,
//             // company_bs_adjective: de::company::bs_adjective::bs_adjective,
//             // company_bs_noun: de::company::bs_noun::bs_noun,
//             // company_bs_verb: de::company::bs_verb::bs_verb,
//             // company_suffix: de::company::suffix::suffix,
//             // lorem_words: de::lorem_words::lorem_words,
//             // hacker_abbreviation: de::abbreviation::abbreviation,
//             // hacker_adjective: de::adjective::adjective,
//             // hacker_noun: de::noun::noun,
//             // hacker_verb: de::verb::verb,
//             // hacker_ingverb: de::ingverb::ingverb,
//             // hacker_phrase: de::phrase::phrase,
//             // phone_number_formats: de::phone_number_formats::phone_number_formats,
//             // finance_account_type: de::finance_account_type::finance_account_type,
//             // finance_transaction_type: de::finance_transaction_type::finance_transaction_type,
//             // finance_currency: de::finance_currency::finance_currency,
//             // finance_iban: de::finance_iban::finance_iban,
//             // finance_credit_card: de::finance_credit_card::finance_credit_card,
//             // internet_avatar_uri: de::internet_avatar_uri::internet_avatar_uri,
//             // internet_domain_suffix: de::internet_domain_suffix::internet_domain_suffix,
//             // internet_free_email: de::internet_free_email::internet_free_email,
//             // internet_example_email: de::internet_example_email::internet_example_email,
//             // internet_password: de::internet_password::internet_password,
//             // commerce_color: de::color::color,
//             // commerce_department: de::department::department,
//             // commerce_product_name: de::product_name::product_name,
//             // commerce_price: de::price::price,
//             // commerce_categories: de::categories::categories,
//             // database_collation: de::collation::collation,
//             // database_column: de::column::column,
//             // database_engine: de::engine::engine,
//             // database_type: &[],
//             // system_mimeTypes: de::system_mimeTypes,
//             // system_directoryPaths: de::system_directoryPaths,
//             // date_month: de::date_month,
//             // date_weekday: de::date_weekday,
//             title: "".to_string(),
//             separator: "".to_string()
//         }

//     }
}

/*

   this index.js file is used for including the faker library as a CommonJS module, instead of a bundle

   you can include the faker library into your existing node.js application by requiring the entire /faker directory

    var faker = require(./faker);
    var randomName = faker.name.findName();

   you can also simply include the "faker.js" file which is the auto-generated bundled version of the faker library

    var faker = require(./customAppPath/faker);
    var randomName = faker.name.findName();


  if you plan on modifying the faker library you should be performing your changes in the /lib/ directory

*/

// /**
//  *
//  * @namespace faker
//  */
// impl Faker {
//     fn new() -> Self {

//     }

//   var self = this;

//   opts = opts || {};

//   // assign options
//   var locales = self.locales || opts.locales || {};
//   var locale = self.locale || opts.locale || "en";
//   var localeFallback = self.localeFallback || opts.localeFallback || "en";

//   self.locales = locales;
//   self.locale = locale;
//   self.localeFallback = localeFallback;

//   self.definitions = {};

//   function bindAll(obj) {
//       Object.keys(obj).forEach(function(meth) {
//           if (typeof obj[meth] === 'function') {
//               obj[meth] = obj[meth].bind(obj);
//           }
//       });
//       return obj;
//   }

//   var Fake = require('./fake');
//   self.fake = new Fake(self).fake;

//   var Unique = require('./unique');
//   self.unique = bindAll(new Unique(self).unique);

//   var Random = require('./random');
//   self.random = bindAll(new Random(self));

//   var Helpers = require('./helpers');
//   self.helpers = new Helpers(self);


//   var Name = require('./name');
//   self.name = bindAll(new Name(self));

//   var Address = require('./address');
//   self.address = bindAll(new Address(self));

//   var Company = require('./company');
//   self.company = bindAll(new Company(self));

//   var Finance = require('./finance');
//   self.finance = bindAll(new Finance(self));

//   var Image = require('./image');
//   self.image = bindAll(new Image(self));

//   var Lorem = require('./lorem');
//   self.lorem = bindAll(new Lorem(self));

//   var Hacker = require('./hacker');
//   self.hacker = bindAll(new Hacker(self));

//   var Internet = require('./internet');
//   self.internet = bindAll(new Internet(self));

//   var Database = require('./database');
//   self.database = bindAll(new Database(self));

//   var Phone = require('./phone_number');
//   self.phone = bindAll(new Phone(self));

//   var _Date = require('./date');
//   self.date = bindAll(new _Date(self));

//   var Commerce = require('./commerce');
//   self.commerce = bindAll(new Commerce(self));

//   var System = require('./system');
//   self.system = bindAll(new System(self));

//   var Git = require('./git');
//   self.git = bindAll(new Git(self));

//   var _definitions = {
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
//     if (typeof self.definitions[d] === "undefined") {
//       self.definitions[d] = {};
//     }

//     if (typeof _definitions[d] === "string") {
//         self.definitions[d] = _definitions[d];
//       return;
//     }

//     _definitions[d].forEach(function(p){
//       Object.defineProperty(self.definitions[d], p, {
//         get: function () {
//           if (typeof self.locales[self.locale][d] === "undefined" || typeof self.locales[self.locale][d][p] === "undefined") {
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
//   var Random = require('./random');
//   this.seedValue = value;
//   this.random = new Random(this, this.seedValue);
// }
