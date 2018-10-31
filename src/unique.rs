use rand::{thread_rng, Rng};
use crate::faker::Faker;
use crate::*;
#[derive(Debug, Clone)]
pub struct Unique <'a> {
    faker: &'a Faker,
{
}
let uniqueExec = require('../vendor/unique');
/**
 *
 * @namespace faker.unique
 */
impl Unique {
    pub fn new(faker: &'a Faker) -> Self {
        Self { faker }

    }

  // initialize unique module class letiables

  // maximum time unique.exec will attempt to run before aborting
  let maxTime = 10;

  // maximum retries unique.exec will recurse before abortings ( max loop depth )
  let maxRetries = 10;

  // time the script started
  // let startTime = 0;

  /**
   * unique
   *
   * @method unique
   */
fn unique(&self, method: &str,  args: &str,  opts: &str) -> String {
    opts = opts || {};
    opts.startTime = new Date().getTime();
    if (typeof opts.maxTime !== 'number') {
      opts.maxTime = maxTime;
    }
    if (typeof opts.maxRetries !== 'number') {
      opts.maxRetries = maxRetries;
    }
    opts.currentIterations = 0;
    return uniqueExec.exec(method, args, opts);
  }
}

module['exports'] = Unique;
