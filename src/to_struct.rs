use crate::EnvLang;
use crate::EnvLangError;
use crate::EnvLangErrorKind;

// ------------------------------------------------------------------------------------
// to_struct()
// ------------------------------------------------------------------------------------
/// return struct EnvLang or an error EnvLangErrorKind
///
/// # Examples
/// ```rust
/// extern crate env_lang;
/// use env_lang::{to_struct, EnvLang};
/// fn main() {
///     let lang_env = "fr_FR.UTF-8@euro"; // or std::env::var("LANG")
///     let result: EnvLang = to_struct(&lang_env).unwrap();
///     assert!(result == EnvLang{
///         language: Some("fr"),
///         localisation: Some("FR"),
///         charset: Some("UTF-8"),
///         variant: Some("euro")
///     });
/// }
/// ```
// ------------------------------------------------------------------------------------
pub fn to_struct(lang_env: &str) -> Result<EnvLang, EnvLangError> {

    // init
    let mut result = EnvLang{
        language: None,
        localisation: None,
        charset: None,
        variant: None,
    };

    if lang_env.len() > 0 {

        // split by space  `` ``
        let split: Vec<&str> = lang_env.split(' ').collect();
        if split.len() > 1 {
            return Err(EnvLangError::from(EnvLangErrorKind::SpaceNotAllow))
        }

        // split by point ``.``
        let split: Vec<&str> = lang_env.split('.').collect();
        if split.len() > 2 {
            return Err(EnvLangError::from(EnvLangErrorKind::TooManyPoint))
        }

        // left and right point
        let left_point = split[0];
        let mut right_point = "";
        if split.len() == 2 {
            right_point = split[1]
        }

        // split left_point by underscore ``_``
        let split: Vec<&str> = left_point.split('_').collect();
        if split.len() > 2 {
            return Err(EnvLangError::from(EnvLangErrorKind::TooManyUnderscore))
        }

        // left and right underscore
        let left_underscore = split[0];
        let mut right_underscore = "";
        if split.len() == 2 {
            right_underscore = split[1]
        }

        // language and localisation
        if left_underscore.len() > 0 { result.language = Some(left_underscore); }
        if right_underscore.len() > 0 { result.localisation = Some(right_underscore); }

        // with string after point ?
        if right_point.len() > 0 {

            // split right_point by arobase ``@``
            let split: Vec<&str> = right_point.split('@').collect();
            if split.len() > 2 {
                return Err(EnvLangError::from(EnvLangErrorKind::TooManyArobase))
            }

            // left and right arobase
            let left_arobase = split[0];
            let mut right_arobase = "";
            if split.len() == 2 {
                right_arobase = split[1]
            }

            // charset and variant
            if left_arobase.len() > 0 { result.charset = Some(left_arobase); }
            if right_arobase.len() > 0 { result.variant = Some(right_arobase); }

        }

    }

    Ok(result)
}
