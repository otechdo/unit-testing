# Lib unit

> To run unit test for your rust applications

* [Documentation](https://docs.rs/unit-testing/)
* [Report a bugs](https://github.com/taishingi/zuu/issues)
* [Source code](https://github.com/taishingi/zuu/tree/master/src/unit-testing)
* [Donate](https://www.paypal.com/donate/?hosted_button_id=LTYH2BXQF57AA)
* [Crate](https://crates.io/crates/unit-testing)
* [Getting cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
* [Rust guide](https://doc.rust-lang.org/cargo/guide/)

## Installation

![bencmark](badges/benchmark.svg)

![build](badges/build.svg)

![check](badges/check.svg)

![clippy](badges/clippy.svg)

![documentation](badges/documentation.svg)

![format](badges/format.svg)

![tests](badges/tests.svg)

![tests](badges/tree.svg)

![tests](badges/verify.svg)

```shell
cargo add unit-testing
```

```rust
#[cfg(test)]
mod tests {
    use crate::{
        assert_contains, assert_directory_exist, assert_equals, assert_false, assert_files_exist,
        assert_not_contains, assert_true, assert_unequals,
        tdd::unit::{Style::POINT, Unit, NO_PROGRESS},
    };
    use std::env::consts::OS;

    #[test]
    pub fn unit() {
        fn battery_full() -> usize {
            100
        }
        fn battery_not_full() -> usize {
            50
        }

        let mut u = Unit::new("Test the unit framework", NO_PROGRESS, POINT);

        u.ok(true).ko(false);
        u.is_directory("/");
        u.is_file("README.md");
        u.not_full(battery_not_full, 100).full(battery_full, 100);
        u.equals("a", "a").unequals("a", "b");
        u.chaos(false, true);
        u.inferior(50, 500).superior(50, 10);
        u.prime(1).prime(7).prime(11);
        u.pair(2).pair(4).pair(6);
        u.impair(3).impair(9);
        u.contains(OS, "linux").not_contains(OS, "windows");
        u.empty("").not_empty(OS);
        u.end().expect("failed");
    }

    #[test]
    pub fn test_macros() {
        assert_true!("All values must matches true", vec![true, true, true]);
        assert_false!("All values must matches false", vec![false, false, false]);
        assert_directory_exist!(
            "Check if user use linux",
            vec!["/", "/home", "/etc", ".", ".."]
        );
        assert_files_exist!(
            "Check if user use linux",
            vec!["/etc/hosts", "/etc/locale.gen"]
        );

        assert_contains!("Check if user use linux", vec!["linux"], OS);
        assert_not_contains!(
            "Check if user use linux",
            vec!["windows", "ios", "freebsd", "openbsd", "android", "solaris", "netbsd", "macos"],
            OS
        );

        assert_equals!(
            "All value must be equals to linux",
            vec!["linux", "linux", "linux"],
            OS
        );

        assert_unequals!(
            "All os must be only equals to linux",
            vec!["windows", "ios", "freebsd", "openbsd", "android", "solaris", "netbsd", "macos"],
            OS
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        assert_contains, assert_directory_exist, assert_equals, assert_false, assert_files_exist,
        assert_not_contains, assert_true, assert_unequals,
        tdd::unit::{Unit, NO_PROGRESS},
    };
    use std::env::consts::OS;

    #[test]
    pub fn unit() {
        fn battery_full() -> usize {
            100
        }
        fn battery_not_full() -> usize {
            50
        }

        let mut u = Unit::new("Test the unit framework", NO_PROGRESS);

        u.ok(true).ko(false);
        u.is_directory("/");
        u.is_file("README.md");
        u.not_full(battery_not_full, 100).full(battery_full, 100);
        u.equals("a", "a").unequals("a", "b");
        u.chaos(false, true);
        u.inferior(50, 500).superior(50, 10);
        u.prime(1).prime(7).prime(11);
        u.pair(2).pair(4).pair(6);
        u.impair(3).impair(9);
        u.contains(OS, "linux").not_contains(OS, "windows");
        u.empty("").not_empty(OS);
        u.end().expect("failed");
    }


    #[test]
    pub fn test_macros() {
        assert_true!("All values must matches true", vec![true, true, true]);
        assert_false!("All values must matches false", vec![false, false, false]);
        assert_directory_exist!(
            "Check if user use linux",
            vec!["/", "/home", "/etc", ".", ".."]
        );
        assert_files_exist!(
            "Check if user use linux",
            vec!["/etc/hosts", "/etc/locale.gen"]
        );

        assert_contains!("Check if user use linux", vec!["linux"], OS);
        assert_not_contains!(
            "Check if user use linux",
            vec!["windows", "ios", "freebsd", "openbsd", "android", "solaris", "netbsd", "macos"],
            OS
        );

        assert_equals!(
            "All value must be equals to linux",
            vec!["linux", "linux", "linux"],
            OS
        );

        assert_unequals!(
            "All os must be only equals to linux",
            vec!["windows", "ios", "freebsd", "openbsd", "android", "solaris", "netbsd", "macos"],
            OS
        );
    }
}
```

```shell
cargo test -- --show-output
```
