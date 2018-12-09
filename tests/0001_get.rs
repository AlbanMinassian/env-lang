extern crate env_lang;
use env_lang::{to_struct, EnvLang, EnvLangError, EnvLangErrorKind};

#[test]
fn to_struct_0001_empty_string() {
    let lang_env = "";
    let lang: EnvLang = to_struct(&lang_env).unwrap();
    assert!(lang == EnvLang{language: None, localisation: None, charset: None, variant: None});
}

#[test]
fn to_struct_0002_full_lang() {
    let lang_env = "fr_FR.UTF-8@euro";
    let lang: EnvLang = to_struct(&lang_env).unwrap();
    assert!(lang == EnvLang{language: Some("fr"), localisation: Some("FR"), charset: Some("UTF-8"), variant: Some("euro")});
}

#[test]
fn to_struct_0004_langue_localisation() {
    let lang_env = "fr_FR";
    let lang: EnvLang = to_struct(&lang_env).unwrap();
    assert!(lang == EnvLang{language: Some("fr"), localisation: Some("FR"), charset: None, variant: None});
}

#[test]
fn to_struct_0005_langue_localisation_charset() {
    let lang_env = "fr_FR.UTF-8";
    let lang: EnvLang = to_struct(&lang_env).unwrap();
    assert!(lang == EnvLang{language: Some("fr"), localisation: Some("FR"), charset: Some("UTF-8"), variant: None});
}


#[test]
fn to_struct_0005_too_many_point() {
    let lang_env = "fr_FR.UTF-8.euro";
    let result = to_struct(&lang_env);
    assert!(result.is_err() == true);
    let error = result.err().unwrap();
    let message = error.inner.to_string();
    assert_eq!(message, "split by point return more than 2 chunks".to_string());
    assert_eq!(error.kind(), &EnvLangErrorKind::TooManyPoint);
}

#[test]
fn to_struct_0006_too_many_underscore() {
    let lang_env = "fr_F_R.UTF-8@euro";
    let result = to_struct(&lang_env);
    assert!(result.is_err() == true);
    let error = result.err().unwrap();
    let message = error.inner.to_string();
    assert_eq!(message, "split by underscore return more than 2 chunks".to_string());
    assert_eq!(error.kind(), &EnvLangErrorKind::TooManyUnderscore);
}

#[test]
fn to_struct_0007_too_many_arobase() {
    let lang_env = "fr_FR.UTF-8@eu@ro";
    let result = to_struct(&lang_env);
    assert!(result.is_err() == true);
    let error = result.err().unwrap();
    let message = error.inner.to_string();
    assert_eq!(message, "split by arobase return more than 2 chunks".to_string());
    assert_eq!(error.kind(), &EnvLangErrorKind::TooManyArobase);
}


#[test]
fn to_struct_0008_space() {
    let lang_env = "string with space";
    let result = to_struct(&lang_env);
    assert!(result.is_err() == true);
    let error = result.err().unwrap();
    let message = error.inner.to_string();
    assert_eq!(message, "space not allow".to_string());
    assert_eq!(error.kind(), &EnvLangErrorKind::SpaceNotAllow);
}

#[test]
fn to_struct_0009_display_error() { // and increase coverage :)
    assert_eq!("space not allow".to_string(), format!("{}", EnvLangError::from(EnvLangErrorKind::SpaceNotAllow)));
}

#[test]
fn to_struct_0010_display_struct() { // and increase coverage :)
    assert_eq!("EnvLang{ language: Some(\"fr\") localisation: None charset: None variant: None }".to_string(), format!("{}", EnvLang{language: Some("fr"), localisation: None, charset: None, variant: None}));
}
