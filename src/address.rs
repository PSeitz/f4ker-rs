use rand::{thread_rng, Rng};
use crate::faker::Faker;
use crate::*;
#[derive(Debug, Clone)]
pub struct Address<'a> {
    faker: &'a Faker,
}

fn to_precision(val: f32, precision: u8) -> String {
    match precision {
        0 => format!("{:.0}", val),
        1 => format!("{:.1}", val),
        2 => format!("{:.2}", val),
        3 => format!("{:.3}", val),
        4 => format!("{:.4}", val),
        5 => format!("{:.5}", val),
        6 => format!("{:.6}", val),
        7 => format!("{:.7}", val),
        8 => format!("{:.8}", val),
        9 => format!("{:.9}", val),
        _ => panic!("precision 0-9 supported, but got {:?}", precision),
    }
}

#[test]
fn name() {
    let faker = Faker::new();
    let address = faker.address();
    println!("\nzip_code {:?}", address.zip_code(None));
    println!("zip_code_by_state {:?}", "address.zip_code_by_state()");
    println!("city {:?}", address.city());
    println!("city_prefix {:?}", address.city_prefix());
    println!("city_suffix {:?}", address.city_suffix());
    println!("street_name {:?}", address.street_name());
    println!("street_address {:?}", address.street_address(true));
    println!("street_suffix {:?}", address.street_suffix());
    println!("street_prefix {:?}", address.street_prefix());
    println!("secondary_address {:?}", address.secondary_address());
    println!("county {:?}", address.county());
    println!("country {:?}", address.country());
    println!("country_code {:?}", address.country_code());
    println!("state {:?}", address.state());
    println!("state_abbr {:?}", address.state_abbr());
    println!("latitude {:?}", address.latitude());
    println!("lat_long_with_opt {:?}", Address::lat_long_with_opt(-90., 90., 4));
    println!("longitude {:?}", address.longitude());
    println!("direction {:?}", address.direction());
    println!("direction_abbr {:?}", address.direction_abbr());
    println!("cardinal_direction {:?}", address.cardinal_direction());
    println!("cardinal_direction_abbr {:?}", address.cardinal_direction_abbr());
    println!("ordinal_direction {:?}", address.ordinal_direction());
    println!("ordinal_direction_abbr {:?}", address.ordinal_direction_abbr());
    println!(
        "nearby_gps_coordinate {:?}",
        address.nearby_gps_coordinate([49.250990, 8.517120], 10., 20.)
    );
}

impl<'a> Address<'a> {
    pub fn new(faker: &'a Faker) -> Self {
        Self { faker }
    }

    ///
    /// Generates random zipcode from format. If format is not specified, the
    /// locale's zip format is used.
    ///
    /// e.g. "88855"
    pub fn zip_code(&self, format: Option<&str>) -> String {
        // if zip format is not specified, use the zip format defined for the locale
        let locale_format = format
            .or(self.faker.address_postcode.map(|el| el.rand()))
            .expect("no format in zip code provided and not zip code in locale found");

        replace_symbols(locale_format)
    }

    /**
     * Generates random zipcode from state abbreviation. If state abbreviation is
     * not specified, a random zip code is generated according to the locale's zip format.
     * Only works for locales with postcode_by_state definition. If a locale does not
     * have a postcode_by_state definition, a random zip code is generated according
     * to the locale's zip format.
     *
     * @method faker.address.zipCodeByState
     * @param {String} state
     */
    /// e.g. "address.zip_code_by_state()"
    // pub fn zip_code_by_state(&self, state: &str) -> &'static str { //TODO
    //     let zipRange = self.faker.address_postcode_by_state()[state];
    //     if (zipRange) {
    //         return faker.random.number(zipRange);
    //     }
    //     return this.zipCode();
    // }

    ///
    /// Generates a random localized city name.
    ///
    /// One of the following is randomly used:
    ///
    /// * `{{address.cityPrefix}} {{name.first_name}}{{address.citySuffix}}`
    /// * `{{address.cityPrefix}} {{name.first_name}}`
    /// * `{{name.first_name}}{{address.citySuffix}}`
    /// * `{{name.last_name}}{{address.citySuffix}}`
    ///
    ///
    /// e.g. "New Kayport"
    pub fn city(&self) -> String {
        let formats = [
            "{{address.city_prefix}} {{name.first_name}}{{address.city_suffix}}",
            "{{address.city_prefix}} {{name.first_name}}",
            "{{name.first_name}}{{address.city_suffix}}",
            "{{name.last_name}}{{address.city_suffix}}",
        ];

        self.faker.fake(rand!(formats))
    }

    /// e.g. "Port"
    pub fn city_prefix(&self) -> &'static str {
        return self.faker.address_city_prefix.rand();
    }

    /// e.g. "chester"
    pub fn city_suffix(&self) -> &'static str {
        return self.faker.address_city_suffix.rand();
    }

    /// e.g. "Paucek Landing"
    pub fn street_name(&self) -> String {
        let mut suffix = self.street_suffix().to_string();
        if suffix != "" {
            suffix.insert(0, ' ');
        }
        if rand::random() {
            self.faker.name().last_name(None).to_owned() + &suffix
        } else {
            self.faker.name().first_name(None).to_owned() + &suffix
        }
    }

    ///
    /// Returns a random localized street address, use_full_address adds `secondary_address`
    ///
    /// e.g. "26432 Corey Rapids Apt. 074"
    pub fn street_address(&self, use_full_address: bool) -> String {
        // if (use_full_address == undefined) { use_full_address = false; }
        let address = match thread_rng().gen_range::<u8>(0, 2) {
            0 => replace_symbol_with_number("#####") + " " + &self.street_name(),
            1 => replace_symbol_with_number("####") + " " + &self.street_name(),
            2 => replace_symbol_with_number("###") + " " + &self.street_name(),
            _ => panic!("thread_rng().gen_range::<u8>(0, 2) produced other value than 0, 1, 2"),
        };
        if use_full_address {
            (address + " " + &self.secondary_address())
        } else {
            address
        }
    }

    /// e.g. "Haven"
    pub fn street_suffix(&self) -> &'static str {
        return self.faker.address_street_suffix.rand();
    }

    /// e.g. "East"                                
    pub fn street_prefix(&self) -> &'static str {
        return self.faker.address_street_prefix.rand();
    }

    /// adds send adress in the format "Apt. ###" "Suite ###"
    /// e.g. "Apt. 757"
    pub fn secondary_address(&self) -> String {
        let options = ["Apt. ###", "Suite ###"];
        return replace_symbol_with_number(rand!(options));
    }

    /// e.g. "Borders"
    pub fn county(&self) -> &'static str {
        return self.faker.address_county.rand();
    }

    /// e.g. "Chad"
    pub fn country(&self) -> &'static str {
        return self.faker.address_country.rand();
    }

    /// e.g. "BQ"
    pub fn country_code(&self) -> &'static str {
        return self.faker.address_country_code.rand();
    }

    /// e.g. "New Hampshire"
    pub fn state(&self) -> &'static str {
        self.faker.address_state.rand()
    }

    /// e.g. "LA"
    pub fn state_abbr(&self) -> &'static str {
        return self.faker.address_state_abbr.rand();
    }

    ///
    /// longitude between -90 and 90 and precision 4
    ///
    /// e.g. "-13.9834"
    pub fn latitude(&self) -> String {
        Self::lat_long_with_opt(-90., 90., 4)
    }

    ///
    /// use for custom longitude or latitude
    ///
    /// e.g. "-72.0782"
    pub fn lat_long_with_opt(min: f32, max: f32, precision: u8) -> String {
        to_precision(thread_rng().gen_range(min, max), precision)
    }

    ///
    /// longitude between -180 and 180 and precision 4
    ///
    /// e.g. "36.2274"
    pub fn longitude(&self) -> String {
        Self::lat_long_with_opt(-180., 180., 4)
    }

    /// will return empty string if not existent in locale
    /// use `direction_abbr`  for a direction abbreviation.
    /// e.g. "East"
    pub fn direction(&self) -> &'static str {
        self.faker.address_direction.rand()
    }

    /// direction abbreviation, will return empty string if not existent in locale
    /// e.g. "W"
    pub fn direction_abbr(&self) -> &'static str {
        self.faker.address_direction_abbr.rand()
    }

    /// e.g. "West"
    pub fn cardinal_direction(&self) -> &'static str {
        self.faker.address_direction.rand()
    }

    /// e.g. "E"
    pub fn cardinal_direction_abbr(&self) -> &'static str {
        self.faker.address_direction_abbr.rand()
    }

    /// e.g. "Southwest"
    pub fn ordinal_direction(&self) -> &'static str {
        self.faker.address_direction.rand()
    }

    /// e.g. "N"
    pub fn ordinal_direction_abbr(&self) -> &'static str {
        self.faker.address_direction_abbr.rand()
    }

    /// e.g. [49.240253, 8.360744]
    pub fn nearby_gps_coordinate(&self, coordinate: [f32; 2], min_distance: f32, max_distance: f32) -> [f32; 2] {
        fn degrees_to_radians(degrees: f32) -> f32 {
            degrees * std::f32::consts::PI / 180.0
        }
        fn radians_to_degrees(radians: f32) -> f32 {
            radians * 180.0 / std::f32::consts::PI
        }
        fn coordinate_with_offset(coordinate: [f32; 2], bearing: f32, distance: f32) -> [f32; 2] {
            const R: f32 = 6378.137; // Radius of the Earth (http://nssdc.gsfc.nasa.gov/planetary/factsheet/earthfact.html)

            let d = distance; // Distance in km

            let lat1 = degrees_to_radians(coordinate[0]); //Current lat point converted to radians
            let lon1 = degrees_to_radians(coordinate[1]); //Current long point converted to radians

            let lat2 = ((lat1).sin() * (d / R).cos() + (lat1).cos() * (d / R).sin() * (bearing).cos()).asin();

            let mut lon2 = lon1
                + (bearing.sin() * (d / R).sin() * (lat1).cos()).atan2((d / R).cos() - (lat1).sin() * (lat2).sin());

            // Keep longitude in range [-180, 180]
            if lon2 > degrees_to_radians(180.0) {
                lon2 = lon2 - degrees_to_radians(360.0);
            } else if lon2 < degrees_to_radians(-180.0) {
                lon2 = lon2 + degrees_to_radians(360.0);
            }

            return [radians_to_degrees(lat2), radians_to_degrees(lon2)];
        }

        let distance = thread_rng().gen_range(min_distance, max_distance);

        let random_coord = coordinate_with_offset(
            coordinate,
            degrees_to_radians(thread_rng().gen_range(0., 360.)),
            distance,
        );
        return [random_coord[0], random_coord[1]];
    }
}
