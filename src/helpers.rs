use rand::{thread_rng, Rng};
struct Helpers {
{
}
/**
 *
 * @namespace faker.helpers
 */
impl Helpers {
    fn new() -> Self {

    }

  let self = this;

  /**
   * backword-compatibility
   *
   * @method faker.helpers.randomize
   * @param {array} array
   */
fn randomize(&self, array: &str) -> String {
      array = array || ["a", "b", "c"];
      return thread_rng().choose(array).unwrap();
  };

  /**
   * slugifies string
   *
   * @method faker.helpers.slugify
   * @param {string} string
   */
fn slugify(&self, string: &str) -> String {
      string = string || "";
      return string.replace(/ /g, '-').replace(/[^\w\.\-]+/g, '');
  };

  /**
   * parses string for a symbol and replace it with a random number from 1-10
   *
   * @method faker.helpers.replaceSymbolWithNumber
   * @param {string} string
   * @param {string} symbol defaults to `"#"`
   */
fn replaceSymbolWithNumber(&self, string: &str,  symbol: &str) -> String {
      string = string || "";
      // default symbol is '#'
      if (symbol == undefined) {
          symbol = '#';
      }

      let str = '';
      for (let i = 0; i < string.length; i++) {
          if (string.charAt(i) == symbol) {
              str += faker.random.number(9);
          } else if (string.charAt(i) == "!"){
              str += faker.random.number({min: 2, max: 9});
          } else {
              str += string.charAt(i);
          }
      }
      return str;
  };

  /**
   * parses string for symbols (numbers or letters) and replaces them appropriately (# will be replaced with number,
   * ? with letter and * will be replaced with number or letter)
   *
   * @method faker.helpers.replaceSymbols
   * @param {string} string
   */
fn replaceSymbols(&self, string: &str) -> String {
      string = string || "";
      let alpha = ['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z']
      let str = '';

      for (let i = 0; i < string.length; i++) {
          if (string.charAt(i) == "#") {
              str += faker.random.number(9);
          } else if (string.charAt(i) == "?") {
              str += thread_rng().choose(alpha).unwrap();
          } else if (string.charAt(i) == "*") {
            str += faker.random.boolean() ? thread_rng().choose(alpha).unwrap() : faker.random.number(9);
          } else {
              str += string.charAt(i);
          }
      }
      return str;
  };

  /**
   * replace symbols in a credit card schems including Luhn checksum
   *
   * @method faker.helpers.replaceCreditCardSymbols
   * @param {string} string
   * @param {string} symbol
   */

fn replaceCreditCardSymbols(&self, string: &str,  symbol: &str) -> String {
     symbol = symbol || "#";

     // Function calculating the Luhn checksum of a number string
     let getCheckBit = function(number) {
       number.reverse();
       number = number.map(function(num, index){
         if(index%2 == 0) {
           num *= 2;
           if(num>9) {
             num -= 9;
           }
         }
         return num;
       });
       let sum = number.reduce(function(prev,curr){return prev + curr;});
       return sum % 10;
     };

     string = string || "";
     string = faker.helpers.regexpStyleStringParse(string); // replace [4-9] with a random number in range etc...
     string = faker.helpers.replaceSymbolWithNumber(string, symbol); // replace ### with random numbers

     let numberList = string.replace(/\D/g,"").split("").map(function(num){return parseInt(num);});
     let checkNum = getCheckBit(numberList);
     return string.replace("L",checkNum);
   };

   /** string repeat helper, alternative to String.prototype.repeat.... See PR #382
   *
   * @method faker.helpers.repeatString
   * @param {string} string
   * @param {number} num
   */
fn repeatString(&self, string: &str, num: &str) -> String {
     if(num.is_none()) {
       num = 0;
     }
     let text = "";
     for(let i = 0; i < num; i++){
       text += string.toString();
     }
     return text;
   };

   /**
    * parse string paterns in a similar way to RegExp
    *
    * e.g. "#{3}test[1-5]" -> "###test4"
    *
    * @method faker.helpers.regexpStyleStringParse
    * @param {string} string
    */
fn regexpStyleStringParse(&self, string: &str) -> String {
     string = string || "";
     // Deal with range repeat `{min,max}`
     let RANGE_REP_REG = /(.)\{(\d+)\,(\d+)\}/;
     let REP_REG = /(.)\{(\d+)\}/;
     let RANGE_REG = /\[(\d+)\-(\d+)\]/;
     let min, max, tmp, repetitions;
     let token = string.match(RANGE_REP_REG);
     while(token !== null){
       min = parseInt(token[2]);
       max =  parseInt(token[3]);
       // switch min and max
       if(min>max) {
         tmp = max;
         max = min;
         min = tmp;
       }
       repetitions = faker.random.number({min:min,max:max});
       string = string.slice(0,token.index) + faker.helpers.repeatString(token[1], repetitions) + string.slice(token.index+token[0].length);
       token = string.match(RANGE_REP_REG);
     }
     // Deal with repeat `{num}`
     token = string.match(REP_REG);
     while(token !== null){
       repetitions = parseInt(token[2]);
       string = string.slice(0,token.index)+ faker.helpers.repeatString(token[1], repetitions) + string.slice(token.index+token[0].length);
       token = string.match(REP_REG);
     }
     // Deal with range `[min-max]` (only works with numbers for now)
     //TODO: implement for letters e.g. [0-9a-zA-Z] etc.

     token = string.match(RANGE_REG);
     while(token !== null){
       min = parseInt(token[1]); // This time we are not capturing the char befor `[]`
       max =  parseInt(token[2]);
       // switch min and max
       if(min>max) {
         tmp = max;
         max = min;
         min = tmp;
       }
        string = string.slice(0,token.index) +
          faker.random.number({min:min, max:max}).toString() +
          string.slice(token.index+token[0].length);
        token = string.match(RANGE_REG);
     }
     return string;
   };

  /**
   * takes an array and returns it randomized
   *
   * @method faker.helpers.shuffle
   * @param {array} o
   */
fn shuffle(&self, o: &str) -> String {
      if (o.is_none() || o.length == 0) {
        return [];
      }
      o = o || ["a", "b", "c"];
      for (let j, x, i = o.length-1; i; j = faker.random.number(i), x = o[--i], o[i] = o[j], o[j] = x);
      return o;
  };

  /**
   * mustache
   *
   * @method faker.helpers.mustache
   * @param {string} str
   * @param {object} data
   */
fn mustache(&self, str: &str,  data: &str) -> String {
    if (str.is_none()) {
      return '';
    }
    for(let p in data) {
      let re = new RegExp('{{' + p + '}}', 'g')
      str = str.replace(re, data[p]);
    }
    return str;
  };

  /**
   * createCard
   *
   * @method faker.helpers.createCard
   */
fn createCard(&self) -> String {
      return {
          "name": faker.name.findName(),
          "username": faker.internet.userName(),
          "email": faker.internet.email(),
          "address": {
              "streetA": faker.address.streetName(),
              "streetB": faker.address.streetAddress(),
              "streetC": faker.address.streetAddress(true),
              "streetD": faker.address.secondaryAddress(),
              "city": faker.address.city(),
              "state": faker.address.state(),
              "country": faker.address.country(),
              "zipcode": faker.address.zipCode(),
              "geo": {
                  "lat": faker.address.latitude(),
                  "lng": faker.address.longitude()
              }
          },
          "phone": faker.phone.phoneNumber(),
          "website": faker.internet.domainName(),
          "company": {
              "name": faker.company.companyName(),
              "catchPhrase": faker.company.catchPhrase(),
              "bs": faker.company.bs()
          },
          "posts": [
              {
                  "words": faker.lorem.words(),
                  "sentence": faker.lorem.sentence(),
                  "sentences": faker.lorem.sentences(),
                  "paragraph": faker.lorem.paragraph()
              },
              {
                  "words": faker.lorem.words(),
                  "sentence": faker.lorem.sentence(),
                  "sentences": faker.lorem.sentences(),
                  "paragraph": faker.lorem.paragraph()
              },
              {
                  "words": faker.lorem.words(),
                  "sentence": faker.lorem.sentence(),
                  "sentences": faker.lorem.sentences(),
                  "paragraph": faker.lorem.paragraph()
              }
          ],
          "accountHistory": [faker.helpers.createTransaction(), faker.helpers.createTransaction(), faker.helpers.createTransaction()]
      };
  };

  /**
   * contextualCard
   *
   * @method faker.helpers.contextualCard
   */
fn contextualCard(&self) -> String {
    let name = faker.name.firstName(),
        userName = faker.internet.userName(name);
    return {
        "name": name,
        "username": userName,
        "avatar": faker.internet.avatar(),
        "email": faker.internet.email(userName),
        "dob": faker.date.past(50, new Date("Sat Sep 20 1992 21:35:02 GMT+0200 (CEST)")),
        "phone": faker.phone.phoneNumber(),
        "address": {
            "street": faker.address.streetName(true),
            "suite": faker.address.secondaryAddress(),
            "city": faker.address.city(),
            "zipcode": faker.address.zipCode(),
            "geo": {
                "lat": faker.address.latitude(),
                "lng": faker.address.longitude()
            }
        },
        "website": faker.internet.domainName(),
        "company": {
            "name": faker.company.companyName(),
            "catchPhrase": faker.company.catchPhrase(),
            "bs": faker.company.bs()
        }
    };
  };


  /**
   * userCard
   *
   * @method faker.helpers.userCard
   */
fn userCard(&self) -> String {
      return {
          "name": faker.name.findName(),
          "username": faker.internet.userName(),
          "email": faker.internet.email(),
          "address": {
              "street": faker.address.streetName(true),
              "suite": faker.address.secondaryAddress(),
              "city": faker.address.city(),
              "zipcode": faker.address.zipCode(),
              "geo": {
                  "lat": faker.address.latitude(),
                  "lng": faker.address.longitude()
              }
          },
          "phone": faker.phone.phoneNumber(),
          "website": faker.internet.domainName(),
          "company": {
              "name": faker.company.companyName(),
              "catchPhrase": faker.company.catchPhrase(),
              "bs": faker.company.bs()
          }
      };
  };

  /**
   * createTransaction
   *
   * @method faker.helpers.createTransaction
   */
fn createTransaction(&self) -> String {
    return {
      "amount" : faker.finance.amount(),
      "date" : new Date(2012, 1, 2),  //TODO: add a ranged date method
      "business": faker.company.companyName(),
      "name": [faker.finance.accountName(), faker.finance.mask()].join(' '),
      "type" : self.randomize(self.faker.finance_transaction_type()),
      "account" : faker.finance.account()
    };
  };

  return self;

};


/*
String.prototype.capitalize = function () { //v1.0
    return this.replace(/\w+/g, function (a) {
        return a.charAt(0).toUpperCase() + a.substr(1).toLowerCase();
    });
};
*/

module['exports'] = Helpers;
