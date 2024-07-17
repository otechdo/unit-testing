use std::panic::UnwindSafe;
use std::path::Path;
use std::{io, panic};

use crate::output::{
    ASSERT_LENGTH_EQUALS, ASSERT_LENGTH_UN0EQUALS, ASSERT_NOT_PANIC, ASSERT_PANIC, IS_CONTAINS,
    IS_EQUALS, IS_EXISTS, IS_INFERIOR, IS_KO, IS_NOT_CONTAINS, IS_NOT_EXISTS, IS_OK, IS_SUPERIOR,
    IS_UNEQUALS, THEORY_IS_FALSE, THEORY_IS_TRUE,
};
use crate::run;
///
/// # Represent a test suite
///
#[derive(Clone, Copy)]
pub struct Suite {
    before_each: Option<fn()>,
    after_each: Option<fn()>,
}

impl Suite {
    ///
    /// # Initialize the suite
    ///
    /// - `before_each` The callback to execute before each test
    /// - `after_each` The callback to execute after each test
    ///
    #[must_use]
    pub fn new(before_each: Option<fn()>, after_each: Option<fn()>) -> Self {
        Self {
            before_each,
            after_each,
        }
    }
    ///
    /// # Run a test
    ///
    /// - `x` The test
    /// - `s` The success message
    /// - `e` The error message
    ///
    /// # Panics
    ///
    /// if test fail
    ///
    #[must_use]
    pub fn run(self, test: bool, success: &str, error: &str) -> Self {
        let after = self.after_each;
        let before = self.before_each;
        run!(test, success, error, before, after);
        self
    }

    ///
    /// # End of the test suite
    ///
    /// # Errors
    ///
    pub fn end(&mut self) -> io::Result<()> {
        Ok(())
    }
    ///
    /// # Check equality
    ///
    /// - `actual`      The actual value
    /// - `expected`    The expected value
    #[must_use]
    pub fn eq<X: PartialEq>(self, actual: &X, expected: &X) -> Self {
        self.run(actual.eq(expected), IS_EQUALS, IS_UNEQUALS)
    }
    ///
    ///  - `a` The result to check if match Ok
    ///
    #[must_use]
    pub fn ok<X, Y>(self, a: &Result<X, Y>) -> Self {
        self.run(a.is_ok(), IS_OK, IS_KO)
    }

    ///
    /// # Check if a callback panic
    ///
    /// - `c` The callback to check
    ///
    #[must_use]
    pub fn panic(self, c: impl FnOnce() + UnwindSafe) -> Self {
        let result = panic::catch_unwind(c).is_ok();
        self.run(result.eq(&false), ASSERT_PANIC, ASSERT_NOT_PANIC)
    }

    ///
    /// # Check if a callback don't panic
    ///
    /// - `c` The callback to check
    ///
    #[must_use]
    pub fn not_panic(self, c: impl FnOnce() + UnwindSafe) -> Self {
        let result = panic::catch_unwind(c);
        self.run(result.is_ok(), ASSERT_NOT_PANIC, ASSERT_PANIC)
    }

    ///
    /// - `a` The data to check if X match Err
    ///
    #[must_use]
    pub fn ko<X, Y>(self, a: &Result<X, Y>) -> Self {
        self.run(a.is_err(), IS_KO, IS_OK)
    }
    ///
    /// # Check the len
    ///
    /// - `actual` The actual len
    /// - `expected`The expected len
    ///
    #[must_use]
    pub fn len<X: ExactSizeIterator>(self, actual: &X, expected: &usize) -> Self {
        self.run(
            actual.len().eq(expected),
            ASSERT_LENGTH_EQUALS,
            ASSERT_LENGTH_UN0EQUALS,
        )
    }

    ///
    /// # Check inequality
    ///
    /// - `actual`      The actual value
    /// - `expected`    The expected value
    ///
    #[must_use]
    pub fn ne<X: PartialEq>(self, actual: &X, expected: &X) -> Self {
        self.run(actual.ne(expected), IS_UNEQUALS, IS_EQUALS)
    }
    ///
    /// # Check if actual is greater than expected
    ///
    /// - `actual` The actual value
    /// - `expected` The expected value
    ///
    #[must_use]
    pub fn gt<X: PartialOrd>(self, actual: &X, expected: &X) -> Self {
        self.run(actual.gt(expected), IS_SUPERIOR, IS_INFERIOR)
    }
    ///
    /// # Check if actual is greater or equal than expected
    ///
    /// - `actual` The actual value
    /// - `expected` The expected value
    ///
    #[must_use]
    pub fn ge<X: PartialOrd>(self, actual: &X, expected: &X) -> Self {
        self.run(actual.ge(expected), IS_SUPERIOR, IS_INFERIOR)
    }

    ///
    /// # Check if actual is containing expected
    ///
    /// - `actual` The actual value
    /// - `expected` The expected value
    ///
    #[must_use]
    pub fn str_contains(self, actual: &str, expected: &str) -> Self {
        self.run(actual.contains(expected), IS_CONTAINS, IS_NOT_CONTAINS)
    }

    ///
    /// # Check if an actual path matches the expected value
    ///
    /// - `actual` The actual value
    /// - `expected` The expected value
    ///
    #[must_use]
    pub fn path_exists(self, actual: &str, expected: bool) -> Self {
        self.run(Path::new(actual).exists().eq(&expected), IS_OK, IS_KO)
    }

    ///
    /// # Check if an actual path exists
    ///
    /// - `actual` The actual path
    ///
    #[must_use]
    pub fn exists(self, actual: &str) -> Self {
        self.run(Path::new(actual).exists(), IS_EXISTS, IS_NOT_EXISTS)
    }

    ///
    /// # Check if actual is not containing expected
    ///
    /// - `actual` The actual value
    /// - `expected` The expected value
    ///
    #[must_use]
    pub fn str_not_contains(self, actual: &str, expected: &str) -> Self {
        self.run(
            actual.contains(expected).eq(&false),
            IS_NOT_CONTAINS,
            IS_CONTAINS,
        )
    }

    ///
    /// # Check if actual is lower or equal than expected
    ///
    /// - `actual` The actual value
    /// - `expected` The expected value
    ///
    #[must_use]
    pub fn le<X: PartialOrd>(self, actual: &X, expected: &X) -> Self {
        self.run(actual.le(expected), IS_INFERIOR, IS_SUPERIOR)
    }
    ///
    /// # Check if actual is lower than expected
    ///
    /// - `actual` The actual value
    /// - `expected` The expected value
    ///
    #[must_use]
    pub fn lt<X: PartialOrd>(self, actual: &X, expected: &X) -> Self {
        self.run(actual.lt(expected), IS_INFERIOR, IS_SUPERIOR)
    }

    ///
    /// # Check if the callback exit code match the expected exit code
    ///
    /// - `actual` The callback to check
    /// - `expected` The expected code
    ///
    #[must_use]
    pub fn response<X: PartialEq>(
        self,
        title: &str,
        description: &str,
        c: &dyn Fn(X) -> X,
        x: X,
        expected: &X,
    ) -> Self {
        self.title(title, description)
            .run(c(x).eq(expected), IS_EQUALS, IS_UNEQUALS)
    }

    ///
    /// # Check if a theorem is equal to expected value
    ///
    /// - `c` The theorem callback
    /// - `expected` The expected value
    ///
    #[must_use]
    pub fn theorem<X: PartialEq>(
        self,
        title: &str,
        description: &str,
        c: &dyn Fn() -> X,
        expected: &X,
    ) -> Self {
        self.sub_title(title, description)
            .run(c().eq(expected), IS_OK, IS_KO)
    }

    ///
    /// # Check if a theorem is equal to expected value
    ///
    /// - `c` The theorem callback
    /// - `expected` The expected value
    ///
    #[must_use]
    pub fn theory<X: PartialEq>(
        self,
        title: &str,
        description: &str,
        callback: &X,
        expected: &X,
    ) -> Self {
        self.sub_title(title, description).run(
            callback.eq(expected),
            THEORY_IS_TRUE,
            THEORY_IS_FALSE,
        )
    }

    ///
    /// # Check if a theorem is different to expected
    ///
    /// - `c` The theorem callback
    /// - `expected` The expected value
    ///
    #[must_use]
    pub fn chaos<X: PartialEq>(
        self,
        title: &str,
        description: &str,
        c: &dyn Fn() -> X,
        expected: &X,
    ) -> Self {
        self.title(title, description)
            .run(c().ne(expected), THEORY_IS_TRUE, THEORY_IS_FALSE)
    }
    fn title(self, title: &str, description: &str) -> Self {
        println!("\n{title}\n\n\t{description}\n");
        self
    }

    fn sub_title(self, title: &str, description: &str) -> Self {
        println!("\t{title}\n\n\t{description}\n");
        self
    }

    ///
    /// # Check if actual is lower than expected
    ///
    /// - `description` The actual value
    /// - `expected` The expected value
    ///
    #[must_use]
    pub fn group(self, title: &str, description: &str, callback: fn(Self) -> Self) -> Self {
        callback(self.title(title, description))
    }

    ///
    /// # Check if actual is lower than expected
    ///
    /// - `description` The actual value
    /// - `expected` The expected value
    ///
    #[must_use]
    pub fn sure(
        self,
        title: &str,
        description: &str,
        callback: &dyn Fn(Self) -> Self,
        x: usize,
    ) -> Self {
        for _i in 0..x {
            let _ = callback(self.title(title, description));
        }
        self
    }
}

///
/// # Start a test suite
///
/// - `description`         The test suite description
/// - `after_all_hook`      A callback
/// - `after_each_hook`     A callback
/// - `before_all_hook`     A callback
/// - `before_each_hook`    A callback
/// - `main`                The main callback
///
pub fn describe(
    title: &str,
    description: &str,
    after_all_hook: Option<fn()>,
    after_each_hook: Option<fn()>,
    before_all_hook: Option<fn()>,
    before_each_hook: Option<fn()>,
    main: fn(Suite) -> Suite,
) -> Suite {
    if let Some(a) = before_all_hook {
        a();
    }
    println!("\n{title}\n\n{description}\n");
    let data: Suite = main(Suite::new(before_each_hook, after_each_hook));
    if let Some(b) = after_all_hook {
        b();
    }
    data
}

#[cfg(test)]
mod test {
    use crate::suite::Suite;
    use crate::{always_panic, it};
    use std::fs;
    use std::ops::Mul;

    fn sure(suite: Suite) -> Suite {
        suite.eq(&4, &4).ne(&3, &4)
    }
    fn main(s: Suite) -> Suite {
        s.group(
            "Should be contains",
            "All data string must be contains all expected strings",
            |s| {
                s.str_contains(
                    &fs::read_to_string("README.md").expect("Failed to parse README.md"),
                    "cargo add unit-testing",
                )
            },
        )
        .group(
            "Check is theorem are valid",
            "The triangle must be rectangle",
            |s| {
                s.theorem("ab = 3; bc = 4", "ac == 5", &ok, &true).theory(
                    "ab = 8; bc = 6",
                    "ac == 10",
                    &is_rect(8_f32, 6_f32),
                    &10_f32,
                )
            },
        )
        .group(
            "Check is theories are valid",
            "The triangle must be rectangle",
            |s| {
                s.theory(
                    "ab = 3; bc = 4",
                    "ac == 5",
                    &is_rect(3.0_f32, 4.0_f32),
                    &5.0_f32,
                )
            },
        )
        .group(
            "Check if path exist",
            "All given path must be exist on all Os",
            |s| {
                s.path_exists("README.md", true)
                    .path_exists(".", true)
                    .exists(".")
                    .exists("README.md")
            },
        )
        .group(
            "Should be not contains",
            "The README.md must be not contains expected data",
            |s| {
                s.str_not_contains(
                    &fs::read_to_string("README.md").expect("Failed to parse README.md"),
                    "cargo add continuous-testing",
                )
            },
        )
        .group("Should be equals", "All values mut be equals", |s| {
            s.eq(&1, &1)
                .eq(&2, &2)
                .response(
                    "Check if the callback no add a 0 before 10",
                    "Check if f(x) => 0",
                    &a,
                    10,
                    &0,
                )
                .response(
                    "Check if the callback is dividable by 2",
                    "Check if f(x) => x² % 2 == 0",
                    &b,
                    2,
                    &0,
                )
                .response(
                    "Check if the callback is dividable by 3",
                    "Check if f(x) => x² % 3 == 0",
                    &c,
                    3,
                    &0,
                )
        })
        .group("Should be unequal", "Check if a and b are different", |s| {
            s.ne(&1, &2).ne(&3, &2)
        })
        .group(
            "Should be math len",
            "All vec must be math the expected length",
            |s| s.len(&vec!["", "", ""].iter(), &3),
        )
        .group("Should be match Ok", "Callbacks mut be return Ok", |s| {
            s.group("Should be divisible by 2", "x % 2 == 0", |s| {
                s.ok(&data(2)).ok(&data(4))
            })
            .group("Should be divisible by 3", "x % 3 == 0", |s| {
                s.eq(&c(3).eq(&0), &true).eq(&c(12).eq(&0), &true)
            })
        })
        .group("Should be match Err", "Callbacks must be return Err", |s| {
            s.ko(&data(5)).ko(&data(15))
        })
        .group(
            "Should panic",
            "Callback should be panic if data is noty divisible by 2",
            |s| s.panic(panic),
        )
        .group("Should not panic", "The callback should never panic", |s| {
            s.not_panic(not_panic)
        })
        .sure(
            "Check the persistence",
            "Check if data return always the same result",
            &sure,
            5,
        )
    }
    fn panic() {
        always_panic!();
    }
    fn not_panic() {}
    fn a(x: i32) -> i32 {
        0.mul(x)
    }
    fn b(x: i32) -> i32 {
        x.pow(2) % 2
    }
    fn c(x: i32) -> i32 {
        x.pow(2) % 3
    }
    fn ok() -> bool {
        3.0_f32.hypot(4.0).eq(&5.0)
    }
    fn is_rect(a: f32, b: f32) -> f32 {
        a.hypot(b)
    }
    fn data(x: usize) -> Result<(), String> {
        if x % 2 == 0 {
            Ok(())
        } else {
            Err(String::from("not divisible by 2"))
        }
    }
    #[test]
    fn suite() {
        it!(
            "Check the suite it test case",
            "Suite test accept no test failure, for guaranty the source code.",
            None,
            None,
            None,
            None,
            main
        );
    }
}
