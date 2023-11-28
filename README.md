# Lib unit

> To run unit test for your rust applications

* [Documentation](https://docs.rs/unit-testing/)
* [Report a bugs](https://github.com/taishingi/zuu/issues)
* [Source code](https://github.com/taishingi/zuu/tree/master/src/unit-testing)
* [Donate](https://www.paypal.com/donate/?hosted_button_id=LTYH2BXQF57AA)
* [Crate](https://crates.io/crates/unit-testing)
* [Getting cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
* [Rust guide](https://doc.rust-lang.org/cargo/guide/)

![bencmark](badges/benchmark.svg)

![build](badges/build.svg)

![check](badges/check.svg)

![clippy](badges/clippy.svg)

![documentation](badges/documentation.svg)

![format](badges/format.svg)

![tests](badges/tests.svg)

![tests](badges/tree.svg)

![tests](badges/verify.svg)

![see-unit](https://raw.githubusercontent.com/taishingi/unit-testing/master/unit-testing-take.gif)

![see-assert](https://raw.githubusercontent.com/taishingi/unit-testing/master/assert-testing-take.gif)

## Installation

```shell
cargo add unit-testing
```

## Assert

All tests in Assert use

```rust
assert!(test);
```

If you want to continue testing onf failures use [**Unit**](https://github.com/taishingi/unit-testing/blob/master/src/unit.rs#L40).

```rust
use std::process::ExitCode;
use unit::unit::{
    traits::unit::{Testable, Theory},
    Unit,
};

fn ok()-> bool
{
    true
}

fn ko()-> bool
{
    false
}

fn callback(u: &mut Unit)-> &mut Unit
{
    u.ko(&ok).ok(&ko)
}

fn main()-> ExitCode {
    Unit::it(vec[&callback])
}
```

## Recommended usage

```rust
use std::{collections::HashSet, env::consts::OS, process::ExitCode};

use num::Float;
use unit::unit::{
    traits::unit::{Testable, Theory},
    Assert,
};

fn ok() -> bool {
    true
}

fn ko() -> bool {
    false
}

fn must_pass(u: &mut Assert) -> &mut Assert {
    u.ok(&ok).ko(&ko)
}

fn must_exists(u: &mut Assert) -> &mut Assert {
    u.exists(".").exists("README.md").exists("/").exists("/lib")
}

fn must_linux(u: &mut Assert) -> &mut Assert {
    u.not_exists("C:\\Users")
        .not_exists("C:\\ProgramData")
        .not_exists("C:\\WINDOWS\\symtem32")
}

fn must_equals(u: &mut Assert) -> &mut Assert {
    u.equals("README.md", "README.md")
        .equals(4, 4)
        .equals(4.4, 4.4)
        .equals(true, true)
        .equals(false, false)
}

fn must_contains(u: &mut Assert) -> &mut Assert {
    let mut v: Vec<String> = Vec::new();
    let o = Option::Some("a".to_string());
    v.push("value".to_string());
    v.push("h".to_string());
    u.vec_contains(v, "h".to_string())
        .option_contains(o, "a".to_string())
        .string_contains(OS, "linux")
        .file_contains("README.md", "Installation")
        .hash_contains(&mut HashSet::from(["a".to_string()]), "a".to_string())
}

fn must_unequals(u: &mut Assert) -> &mut Assert {
    u.unequals("README.md", ".")
        .unequals(4, 6)
        .unequals(5.6, 4.4)
        .unequals(false, true)
        .unequals(false, true)
}

fn must_superior(u: &mut Assert) -> &mut Assert {
    u.superior(1, 0).superior(5, 2)
}

fn programs(u: &mut Assert) -> &mut Assert {
    u.is_program("/usr/bin/git").is_program("/usr/bin/curl")
}
fn no_programs(u: &mut Assert) -> &mut Assert {
    u.not_program("cmd")
}

fn must_inferior(u: &mut Assert) -> &mut Assert {
    u.inferior(10, 50).inferior(50, 200)
}

fn must_beetween(u: &mut Assert) -> &mut Assert {
    u.between(10, 5, 50).between(50, 10, 200)
}

fn pythagore() -> f32 {
    Float::hypot(3.0, 4.0)
}

fn pythagore_not_work() -> bool {
    Float::hypot(4.0, 4.0) == 5.0
}

fn must_theory(u: &mut Assert) -> &mut Assert {
    u.theory(5.0, &pythagore).chaos(&pythagore_not_work)
}

fn main() -> ExitCode {
    Assert::it(vec![
        &must_beetween,
        &programs,
        &must_theory,
        &no_programs,
        &must_unequals,
        &must_linux,
        &must_equals,
        &must_exists,
        &must_pass,
        &must_contains,
        &must_superior,
        &must_inferior,
    ])
}
```
