use rand::{thread_rng, Rng};
#[derive(Debug, Clone)]
pub struct Image <'a> {
    faker: &'a Faker,
{
}
/**
 *
 * @namespace faker.image
 */
impl Image {
    pub fn new(faker: &'a Faker) -> Self {
        Self { faker }

    }


  /**
   * image
   *
   * @param {number} width
   * @param {number} height
   * @param {boolean} randomize
   * @method faker.image.image
   */
fn image(&self, width: &str,  height: &str,  randomize: &str) -> String {
    let categories = ["abstract", "animals", "business", "cats", "city", "food", "nightlife", "fashion", "people", "nature", "sports", "technics", "transport"];
    return self[thread_rng().choose(categories).unwrap()](width, height, randomize);
  };
  /**
   * avatar
   *
   * @method faker.image.avatar
   */
fn avatar(&self) -> String {
    return faker.internet.avatar();
  };
  /**
   * imageUrl
   *
   * @param {number} width
   * @param {number} height
   * @param {string} category
   * @param {boolean} randomize
   * @method faker.image.imageUrl
   */
fn imageUrl(&self, width: &str,  height: &str,  category: &str,  randomize: &str,  https: &str) -> String {
      let width = width || 640;
      let height = height || 480;
      let protocol = 'http://';
      if (https.is_some() && https == true) {
        protocol = 'https://';
      }
      let url = protocol + 'lorempixel.com/' + width + '/' + height;
      if (category.is_some()) {
        url += '/' + category;
      }

      if (randomize) {
        url += '?' + faker.random.number()
      }

      return url;
  };
  /**
   * abstract
   *
   * @param {number} width
   * @param {number} height
   * @param {boolean} randomize
   * @method faker.image.abstract
   */
fn abstract(&self, width: &str,  height: &str,  randomize: &str) -> String {
    return faker.image.imageUrl(width, height, 'abstract', randomize);
  };
  /**
   * animals
   *
   * @param {number} width
   * @param {number} height
   * @param {boolean} randomize
   * @method faker.image.animals
   */
fn animals(&self, width: &str,  height: &str,  randomize: &str) -> String {
    return faker.image.imageUrl(width, height, 'animals', randomize);
  };
  /**
   * business
   *
   * @param {number} width
   * @param {number} height
   * @param {boolean} randomize
   * @method faker.image.business
   */
fn business(&self, width: &str,  height: &str,  randomize: &str) -> String {
    return faker.image.imageUrl(width, height, 'business', randomize);
  };
  /**
   * cats
   *
   * @param {number} width
   * @param {number} height
   * @param {boolean} randomize
   * @method faker.image.cats
   */
fn cats(&self, width: &str,  height: &str,  randomize: &str) -> String {
    return faker.image.imageUrl(width, height, 'cats', randomize);
  };
  /**
   * city
   *
   * @param {number} width
   * @param {number} height
   * @param {boolean} randomize
   * @method faker.image.city
   */
fn city(&self, width: &str,  height: &str,  randomize: &str) -> String {
    return faker.image.imageUrl(width, height, 'city', randomize);
  };
  /**
   * food
   *
   * @param {number} width
   * @param {number} height
   * @param {boolean} randomize
   * @method faker.image.food
   */
fn food(&self, width: &str,  height: &str,  randomize: &str) -> String {
    return faker.image.imageUrl(width, height, 'food', randomize);
  };
  /**
   * nightlife
   *
   * @param {number} width
   * @param {number} height
   * @param {boolean} randomize
   * @method faker.image.nightlife
   */
fn nightlife(&self, width: &str,  height: &str,  randomize: &str) -> String {
    return faker.image.imageUrl(width, height, 'nightlife', randomize);
  };
  /**
   * fashion
   *
   * @param {number} width
   * @param {number} height
   * @param {boolean} randomize
   * @method faker.image.fashion
   */
fn fashion(&self, width: &str,  height: &str,  randomize: &str) -> String {
    return faker.image.imageUrl(width, height, 'fashion', randomize);
  };
  /**
   * people
   *
   * @param {number} width
   * @param {number} height
   * @param {boolean} randomize
   * @method faker.image.people
   */
fn people(&self, width: &str,  height: &str,  randomize: &str) -> String {
    return faker.image.imageUrl(width, height, 'people', randomize);
  };
  /**
   * nature
   *
   * @param {number} width
   * @param {number} height
   * @param {boolean} randomize
   * @method faker.image.nature
   */
fn nature(&self, width: &str,  height: &str,  randomize: &str) -> String {
    return faker.image.imageUrl(width, height, 'nature', randomize);
  };
  /**
   * sports
   *
   * @param {number} width
   * @param {number} height
   * @param {boolean} randomize
   * @method faker.image.sports
   */
fn sports(&self, width: &str,  height: &str,  randomize: &str) -> String {
    return faker.image.imageUrl(width, height, 'sports', randomize);
  };
  /**
   * technics
   *
   * @param {number} width
   * @param {number} height
   * @param {boolean} randomize
   * @method faker.image.technics
   */
fn technics(&self, width: &str,  height: &str,  randomize: &str) -> String {
    return faker.image.imageUrl(width, height, 'technics', randomize);
  };
  /**
   * transport
   *
   * @param {number} width
   * @param {number} height
   * @param {boolean} randomize
   * @method faker.image.transport
   */
fn transport(&self, width: &str,  height: &str,  randomize: &str) -> String {
    return faker.image.imageUrl(width, height, 'transport', randomize);
  };
  /**
   * dataUri
   *
   * @param {number} width
   * @param {number} height
   * @param {string} color
   * @method faker.image.dataUri
   */
fn dataUri(&self, width: &str,  height: &str,  color: &str) -> String {
    color = color || 'grey';
    let svgString = '<svg xmlns="http://www.w3.org/2000/svg" version="1.1" baseProfile="full" width="' + width + '" height="' + height + '"><rect width="100%" height="100%" fill="' + color + '"/><text x="' + width / 2 + '" y="' + height / 2 + '" font-size="20" alignment-baseline="middle" text-anchor="middle" fill="white">' + width + 'x' + height + '</text></svg>';
    let rawPrefix = 'data:image/svg+xml;charset=UTF-8,';
    return rawPrefix + encodeURIComponent(svgString);
  };
}

module["exports"] = Image;
