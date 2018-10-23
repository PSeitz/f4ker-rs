use crate::locales;

struct Faker {
    name_first_name: &'static [&'static str],
    name_last_name: &'static [&'static str],
    name_prefix: &'static [&'static str],
    name_suffix: &'static [&'static str],
    name_gender: &'static [&'static str],
    name_title: &'static [&'static str],
    name_male_prefix: &'static [&'static str],
    name_female_prefix: &'static [&'static str],
    name_male_first_name: &'static [&'static str],
    name_female_first_name: &'static [&'static str],
    name_male_middle_name: &'static [&'static str],
    name_female_middle_name: &'static [&'static str],
    name_male_last_name: &'static [&'static str],
    name_female_last_name: &'static [&'static str],
    address_city_prefix: &'static [&'static str],
    address_city_suffix: &'static [&'static str],
    address_street_suffix: &'static [&'static str],
    address_county: &'static [&'static str],
    address_country: &'static [&'static str],
    address_country_code: &'static [&'static str],
    address_state: &'static [&'static str],
    address_state_abbr: &'static [&'static str],
    address_street_prefix: &'static [&'static str],
    address_postcode: &'static [&'static str],
    address_postcode_by_state: &'static [&'static str],
    address_direction: &'static [&'static str],
    address_direction_abbr: &'static [&'static str],
    company_adjective: &'static [&'static str],
    company_noun: &'static [&'static str],
    company_descriptor: &'static [&'static str],
    company_bs_adjective: &'static [&'static str],
    company_bs_noun: &'static [&'static str],
    company_bs_verb: &'static [&'static str],
    company_suffix: &'static [&'static str],
    lorem_words: &'static [&'static str],
    hacker_abbreviation: &'static [&'static str],
    hacker_adjective: &'static [&'static str],
    hacker_noun: &'static [&'static str],
    hacker_verb: &'static [&'static str],
    hacker_ingverb: &'static [&'static str],
    hacker_phrase: &'static [&'static str],
    phone_number_formats: &'static [&'static str],
    finance_account_type: &'static [&'static str],
    finance_transaction_type: &'static [&'static str],
    finance_currency: &'static [&'static str],
    finance_iban: &'static [&'static str],
    finance_credit_card: &'static [&'static str],
    internet_avatar_uri: &'static [&'static str],
    internet_domain_suffix: &'static [&'static str],
    internet_free_email: &'static [&'static str],
    internet_example_email: &'static [&'static str],
    internet_password: &'static [&'static str],
    commerce_color: &'static [&'static str],
    commerce_department: &'static [&'static str],
    commerce_product_name: &'static [&'static str],
    commerce_price: &'static [&'static str],
    commerce_categories: &'static [&'static str],
    database_collation: &'static [&'static str],
    database_column: &'static [&'static str],
    database_engine: &'static [&'static str],
    database_type: &'static [&'static str],
    system_mimeTypes: &'static [&'static str],
    system_directoryPaths: &'static [&'static str],
    date_month: &'static [&'static str],
    date_weekday: &'static [&'static str],
    title: String,
    separator: String
}

impl Faker {
    fn new() -> Self {
        use self::locales::en;
        Faker {
            name_first_name: en::name::first_name,
            name_last_name: en::name::last_name,
            name_prefix: en::name::prefix,
            name_suffix: en::name::suffix,
            name_gender: en::name::gender,
            name_title: en::name::title,
            name_male_prefix: en::name::male_prefix,
            name_female_prefix: en::name::female_prefix,
            name_male_first_name: en::name::male_first_name,
            name_female_first_name: en::name::female_first_name,
            name_male_middle_name: en::name::male_middle_name,
            name_female_middle_name: en::name::female_middle_name,
            name_male_last_name: en::name::male_last_name,
            name_female_last_name: en::name::female_last_name,
            address_city_prefix: en::address::city_prefix,
            address_city_suffix: en::address::city_suffix,
            address_street_suffix: en::address::street_suffix,
            address_county: en::address::county,
            address_country: en::address::country,
            address_country_code: en::address::country_code,
            address_state: en::address::state,
            address_state_abbr: en::address::state_abbr,
            address_street_prefix: en::address::street_prefix,
            address_postcode: en::address::postcode,
            address_postcode_by_state: en::address::postcode_by_state,
            address_direction: en::address::direction,
            address_direction_abbr: en::address::direction_abbr,
            company_adjective: en::company::adjective,
            company_noun: en::company::noun,
            company_descriptor: en::company::descriptor,
            company_bs_adjective: en::company::bs_adjective,
            company_bs_noun: en::company::bs_noun,
            company_bs_verb: en::company::bs_verb,
            company_suffix: en::company::suffix,
            lorem_words: en::lorem_words,
            hacker_abbreviation: en::abbreviation,
            hacker_adjective: en::adjective,
            hacker_noun: en::noun,
            hacker_verb: en::verb,
            hacker_ingverb: en::ingverb,
            hacker_phrase: en::phrase,
            phone_number_formats: en::phone_number_formats,
            finance_account_type: en::finance_account_type,
            finance_transaction_type: en::finance_transaction_type,
            finance_currency: en::finance_currency,
            finance_iban: en::finance_iban,
            finance_credit_card: en::finance_credit_card,
            internet_avatar_uri: en::internet_avatar_uri,
            internet_domain_suffix: en::internet_domain_suffix,
            internet_free_email: en::internet_free_email,
            internet_example_email: en::internet_example_email,
            internet_password: en::internet_password,
            commerce_color: en::color,
            commerce_department: en::department,
            commerce_product_name: en::product_name,
            commerce_price: en::price,
            commerce_categories: en::categories,
            database_collation: en::collation,
            database_column: en::column,
            database_engine: en::engine,
            database_type: &[],
            system_mimeTypes: en::system_mimeTypes,
            system_directoryPaths: en::system_directoryPaths,
            date_month: en::date_month,
            date_weekday: en::date_weekday,
            title: "".to_string(),
            separator: "".to_string()
        }

    }
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
