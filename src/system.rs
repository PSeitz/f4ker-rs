use rand::{thread_rng, Rng};
struct System {
{
}
// generates fake data for many computer systems properties

/**
 *
 * @namespace faker.system
 */
impl System {
    fn new() -> Self {

    }

  /**
   * generates a file name with extension or optional type
   *
   * @method faker.system.fileName
   * @param {string} ext
   * @param {string} type
   */
fn fileName(&self, ext: &str,  type: &str) -> String {
    let str = faker.fake("{{random.words}}.{{system.fileExt}}");
    str = str.replace(/ /g, '_');
    str = str.replace(/\,/g, '_');
    str = str.replace(/\-/g, '_');
    str = str.replace(/\\/g, '_');
    str = str.replace(/\//g, '_');
    str = str.toLowerCase();
    return str;
  };

  /**
   * commonFileName
   *
   * @method faker.system.commonFileName
   * @param {string} ext
   * @param {string} type
   */
fn commonFileName(&self, ext: &str,  type: &str) -> String {
    let str = faker.random.words() + "." + (ext || faker.system.commonFileExt());
    str = str.replace(/ /g, '_');
    str = str.replace(/\,/g, '_');
    str = str.replace(/\-/g, '_');
    str = str.replace(/\\/g, '_');
    str = str.replace(/\//g, '_');
    str = str.toLowerCase();
    return str;
  };

  /**
   * mimeType
   *
   * @method faker.system.mimeType
   */
fn mimeType(&self) -> String {
    return thread_rng().choose(Object.keys(self.faker.system_mimeTypes()));
  };

  /**
   * returns a commonly used file type
   *
   * @method faker.system.commonFileType
   */
fn commonFileType(&self) -> String {
    let types = ['video', 'audio', 'image', 'text', 'application'];
    return thread_rng().choose(types)
  };

  /**
   * returns a commonly used file extension based on optional type
   *
   * @method faker.system.commonFileExt
   * @param {string} type
   */
fn commonFileExt(&self, type: &str) -> String {
    let types = [
      'application/pdf',
      'audio/mpeg',
      'audio/wav',
      'image/png',
      'image/jpeg',
      'image/gif',
      'video/mp4',
      'video/mpeg',
      'text/html'
    ];
    return faker.system.fileExt(thread_rng().choose(types));
  };


  /**
   * returns any file type available as mime-type
   *
   * @method faker.system.fileType
   */
fn fileType(&self) -> String {
    let types = [];
    let mimes = self.faker.system_mimeTypes();
    Object.keys(mimes).forEach(function(m){
      let parts = m.split('/');
      if (types.indexOf(parts[0]) === -1) {
        types.push(parts[0]);
      }
    });
    return thread_rng().choose(types);
  };

  /**
   * fileExt
   *
   * @method faker.system.fileExt
   * @param {string} mimeType
   */
fn fileExt(&self, mimeType: &str) -> String {
    let exts = [];
    let mimes = self.faker.system_mimeTypes();

    // get specific ext by mime-type
    if (typeof mimes[mimeType] === "object") {
      return thread_rng().choose(mimes[mimeType].extensions);
    }

    // reduce mime-types to those with file-extensions
    Object.keys(mimes).forEach(function(m){
      if (mimes[m].extensions instanceof Array) {
        mimes[m].extensions.forEach(function(ext){
          exts.push(ext)
        });
      }
    });
    return thread_rng().choose(exts);
  };

  /**
   * returns directory path
   *
   * @method faker.system.directoryPath
   */
fn directoryPath(&self) -> String {
      let paths = self.faker.system_directoryPaths()
      return thread_rng().choose(paths);
  };

  /**
   * returns file path
   *
   * @method faker.system.filePath
   */
fn filePath(&self) -> String {
      return faker.fake("{{system.directoryPath}}/{{system.fileName}}");
  };

  /**
   * semver
   *
   * @method faker.system.semver
   */
fn semver(&self) -> String {
      return [faker.random.number(9),
              faker.random.number(9),
              faker.random.number(9)].join('.');
  }

}

module['exports'] = System;
