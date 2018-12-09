# env-lang #

[![Build Status](https://travis-ci.org/AlbanMinassian/env-lang.svg?branch=master)](https://travis-ci.org/AlbanMinassian/env-lang)
[![codecov](https://codecov.io/gh/AlbanMinassian/env-lang/branch/master/graph/badge.svg)](https://codecov.io/gh/AlbanMinassian/env-lang)
[![License:MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![env-lang Latest Version](https://img.shields.io/crates/v/env-lang.svg)](https://crates.io/crates/env-lang)

Return env LANG' struct with language, localisation, charset and variant.

## Usage ##

``Cargo.toml``

```rust
[dependencies]
env-lang = "1.0.0"
```

``src/main.rs``

```rust
extern crate env_lang;
use env_lang::{to_struct, EnvLang};
fn main() {
    let lang_env = "fr_FR.UTF-8@euro"; // or std::env::var("LANG")
    let result: EnvLang = to_struct(&lang_env).unwrap();
    assert!(result == EnvLang{
        language: Some("fr"),
        localisation: Some("FR"),
        charset: Some("UTF-8"),
        variant: Some("euro")
    });
}
```


## Links ##

- [documentation env-lang (docs.rs)](https://docs.rs/env-lang)

## License ##

Copyright © 2018, [Alban Minassian](https://github.com/AlbanMinassian)

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

The Software is provided “as is”, without warranty of any kind, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose and noninfringement. In no event shall the authors or copyright holders X be liable for any claim, damages or other liability, whether in an action of contract, tort or otherwise, arising from, out of or in connection with the software or the use or other dealings in the Software.
Except as contained in this notice, the name of the [Alban Minassian](https://github.com/AlbanMinassian) shall not be used in advertising or otherwise to promote the sale, use or other dealings in this Software without prior written authorization from the [Alban Minassian](https://github.com/AlbanMinassian).
