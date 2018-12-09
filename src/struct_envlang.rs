use std::fmt;

// ------------------------------------------------------------------------------------
// struct EnvLang
// ------------------------------------------------------------------------------------
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// struct
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
#[derive(Clone, Debug, PartialEq)]
pub struct EnvLang<'a> {
    pub language: Option<&'a str>,
    pub localisation: Option<&'a str>,
    pub charset: Option<&'a str>,
    pub variant: Option<&'a str>,
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
// display
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
impl<'a> fmt::Display for EnvLang<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EnvLang{{ language: {:?} localisation: {:?} charset: {:?} variant: {:?} }}",self.language, self.localisation, self.charset, self.variant)
    }
}
