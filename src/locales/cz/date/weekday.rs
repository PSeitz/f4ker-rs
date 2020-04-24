// Source: http://unicode.org/cldr/trac/browser/tags/release-27/common/main/en.xml#L1847

pub static WIDE: &'static [&'static str] = &["Pondělí", "Úterý", "Středa", "čtvrtek", "Pátek", "Sobota", "Neděle"];
// Property "wide_context" is optional, if not set then "wide" will be used instead
// It is used to specify a word in context, which may differ from a stand-alone word
pub static WIDE_CONTEXT: &'static [&'static str] =
    &["Pondělí", "Úterý", "Středa", "čtvrtek", "Pátek", "Sobota", "Neděle"];
pub static ABBR: &'static [&'static str] = &["Po", "Út", "St", "čt", "Pá", "So", "Ne"];
// Property "abbr_context" is optional, if not set then "abbr" will be used instead
// It is used to specify a word in context, which may differ from a stand-alone word
pub static ABBR_CONTEXT: &'static [&'static str] = &["Po", "Út", "St", "čt", "Pá", "So", "Ne"];
