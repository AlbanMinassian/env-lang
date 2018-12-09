//! # env-lang
//!
//! [![Build Status](https://travis-ci.org/AlbanMinassian/env-lang.svg?branch=master)](https://travis-ci.org/AlbanMinassian/env-lang)
//! [![codecov](https://codecov.io/gh/AlbanMinassian/env-lang/branch/master/graph/badge.svg)](https://codecov.io/gh/AlbanMinassian/env-lang)
//! [![License:MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
//! [![env-lang Latest Version](https://img.shields.io/crates/v/env-lang.svg)](https://crates.io/crates/env-lang)
//!
//! return env LANG' struct with language, localisation, charset and variant
//!
//! ## Example
//!
//! ```rust
//! extern crate env_lang;
//! use env_lang::{to_struct, EnvLang};
//! fn main() {
//!     let lang_env = "fr_FR.UTF-8@euro"; // or std::env::var("LANG")
//!     let result: EnvLang = to_struct(&lang_env).unwrap();
//!     assert!(result == EnvLang{
//!         language: Some("fr"),
//!         localisation: Some("FR"),
//!         charset: Some("UTF-8"),
//!         variant: Some("euro")
//!     });
//! }
//! ```
//!
//! ## Links
//!
//! github: [https://github.com/AlbanMinassian/env-lang](https://github.com/AlbanMinassian/env-lang)
//!
//! ## license
//!
//! MIT

extern crate failure;
extern crate core;
use failure::Fail;

// struct
mod struct_envlang;
pub use self::struct_envlang::EnvLang;

// errors
mod struct_envlang_error;
pub use self::struct_envlang_error::EnvLangError;
pub use self::struct_envlang_error::EnvLangErrorKind;

// util to_struct
mod to_struct;
pub use self::to_struct::to_struct;
