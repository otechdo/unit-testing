- [Unit output](#unit-output)
- [Assertions output](#assertions-output)
- [Installation](#installation)
- [Assert](#assert)
- [Recommended usage](#recommended-usage)

[Documentation](https://docs.rs/unit-testing/)

[Report a bug](https://github.com/taishingi/unit-testing/issues)

[Source code](https://github.com/taishingi/unit-testing)

[Donate](https://www.paypal.com/donate/?hosted_button_id=LTYH2BXQF57AA)

[Crate](https://crates.io/crates/unit-testing)

## Unit output

![see-unit](https://raw.githubusercontent.com/taishingi/unit-testing/master/unit-testing-take.gif)

## Assertions output

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

If you want to continue testing onf failures use Unit.

## Testing

```rust
#[cfg(test)]
mod tests {

    use crate::{
        assert_that, check_that,
        unit::{
            object::{Testable, Theory},
            output::DISABLE_PROGRESS_TIME,
            Assert, Unit,
        },
    };
    use std::{collections::HashSet, env::consts::OS};

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
        u.program("/usr/bin/git").program("/usr/bin/curl")
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
        3.0_f32.hypot(4.0)
    }

    fn pythagore_not_work() -> bool {
        4.0_f32.hypot(4.0) == 5.0
    }

    fn must_theory(u: &mut Assert) -> &mut Assert {
        u.theory(5.0, &pythagore).chaos(&pythagore_not_work)
    }

    fn check_pass(u: &mut Unit) -> &mut Unit {
        u.ok(&ok).ko(&ko)
    }

    fn check_exists(u: &mut Unit) -> &mut Unit {
        u.exists(".").exists("README.md").exists("/").exists("/lib")
    }

    fn check_linux(u: &mut Unit) -> &mut Unit {
        u.not_exists("C:\\Users")
            .not_exists("C:\\ProgramData")
            .not_exists("C:\\WINDOWS\\symtem32")
    }

    fn check_equals(u: &mut Unit) -> &mut Unit {
        u.equals("README.md", "README.md")
            .equals(4, 4)
            .equals(4.4, 4.4)
            .equals(true, true)
            .equals(false, false)
    }

    fn check_contains(u: &mut Unit) -> &mut Unit {
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

    fn check_unequals(u: &mut Unit) -> &mut Unit {
        u.unequals("README.md", ".")
            .unequals(4, 6)
            .unequals(5.6, 4.4)
            .unequals(false, true)
            .unequals(false, true)
    }

    fn check_superior(u: &mut Unit) -> &mut Unit {
        u.superior(1, 0).superior(5, 2)
    }

    fn check_programs(u: &mut Unit) -> &mut Unit {
        u.program("/usr/bin/git").program("/usr/bin/curl")
    }

    fn check_no_programs(u: &mut Unit) -> &mut Unit {
        u.not_program("cmd")
    }

    fn check_inferior(u: &mut Unit) -> &mut Unit {
        u.inferior(10, 50).inferior(50, 200)
    }

    fn check_beetween(u: &mut Unit) -> &mut Unit {
        u.between(10, 5, 50).between(50, 10, 200)
    }

    fn check_theory(u: &mut Unit) -> &mut Unit {
        u.theory(5.0, &pythagore).chaos(&pythagore_not_work)
    }

    #[test]
    pub fn assert_that() {
        assert_that!(
            "Test the assert framework",
            DISABLE_PROGRESS_TIME,
            vec![
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
            ]
        );
    }

    #[test]
    pub fn check_that() {
        check_that!(
            "Test the unit framework",
            DISABLE_PROGRESS_TIME,
            vec![
                &check_beetween,
                &check_programs,
                &check_no_programs,
                &check_unequals,
                &check_linux,
                &check_equals,
                &check_exists,
                &check_pass,
                &check_contains,
                &check_superior,
                &check_inferior,
                &check_theory
            ]
        );
    }
}
```
