use rand::{thread_rng, Rng};
#[derive(Debug, Clone)]
pub struct Git <'a> {
    faker: &'a Faker,
{
}
/**
 * @namespace faker.git
 */

impl Git {
    pub fn new(faker: &'a Faker) -> Self {
        Self { faker }

    }

  let hexChars = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f"];

  /**
   * branch
   *
   * @method faker.git.branch
   */
fn branch(&self) -> String {
    let noun = faker.hacker.noun().replace(' ', '-');
    let verb = faker.hacker.verb().replace(' ', '-');
    return noun + '-' + verb;
  }

  /**
   * commitEntry
   *
   * @method faker.git.commitEntry
   * @param {object} options
   */
fn commitEntry(&self, options: &str) -> String {
    options = options || {};

    let entry = 'commit {{git.commitSha}}\r\n';

    if (options.merge || (faker.random.number({ min: 0, max: 4 }) == 0)) {
      entry += 'Merge: {{git.shortSha}} {{git.shortSha}}\r\n';
    }

    entry += 'Author: {{name.firstName}} {{name.lastName}} <{{internet.email}}>\r\n';
    entry += 'Date: ' + faker.date.recent().toString() + '\r\n';
    entry += '\r\n\xa0\xa0\xa0\xa0{{git.commitMessage}}\r\n';

    return f(entry);
  };

  /**
   * commitMessage
   *
   * @method faker.git.commitMessage
   */
fn commitMessage(&self) -> String {
    let format = '{{hacker.verb}} {{hacker.adjective}} {{hacker.noun}}';
    return f(format);
  };

  /**
   * commitSha
   *
   * @method faker.git.commitSha
   */
fn commitSha(&self) -> String {
    let commit = "";

    for (let i = 0; i < 40; i++) {
      commit += thread_rng().choose(hexChars).unwrap();
    }

    return commit;
  };

  /**
   * shortSha
   *
   * @method faker.git.shortSha
   */
fn shortSha(&self) -> String {
    let shortSha = "";

    for (let i = 0; i < 7; i++) {
      shortSha += thread_rng().choose(hexChars).unwrap();
    }

    return shortSha;
  };

  return self;
}

module['exports'] = Git;
