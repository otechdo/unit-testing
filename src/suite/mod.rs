use crate::output::{
    IS_CONTAINS, IS_EQUALS, IS_EXISTS, IS_INFERIOR, IS_KO, IS_NOT_CONTAINS, IS_NOT_EXISTS, IS_OK,
    IS_SUPERIOR, IS_UNEQUALS,
};
use colored_truecolor::Colorize;
use std::io;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

///
/// # Represent a test suite
///
pub struct Suite {
    before_each: fn(),
    after_each: fn(),
}

impl Suite {
    ///
    /// # Initialize the suite
    ///
    /// - `before_each` The callback to execute before each test
    /// - `after_each` The callback to execute after each test
    ///
    pub fn new(before_each: fn(), after_each: fn()) -> Self {
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
    ///
    pub fn run(self, x: bool, s: &str, e: &str) -> Self {
        (self.before_each)();
        assert!(x, "{}", e);
        println!(
            "      {}",
            format_args!("{} {}", "✓".green().bold(), s.cyan().bold())
        );
        (self.after_each)();
        sleep(Duration::from_millis(50));
        self
    }

    ///
    /// # End of the test suite
    ///
    pub fn end(&mut self) -> io::Result<()> {
        Ok(())
    }
    ///
    /// # Check equality
    ///
    /// - `actual`      The actual value
    /// - `expected`    The expected value
    ///
    pub fn eq<X: PartialEq>(self, actual: X, expected: X) -> Self {
        self.run(actual.eq(&expected), IS_EQUALS, IS_UNEQUALS)
    }
    ///
    /// # Check inequality
    ///
    /// - `actual`      The actual value
    /// - `expected`    The expected value
    ///
    pub fn ne<X: PartialEq>(self, actual: X, expected: X) -> Self {
        self.run(actual.ne(&expected), IS_UNEQUALS, IS_EQUALS)
    }
    ///
    /// # Check if actual is greater than expected
    ///
    /// - `actual` The actual value
    /// - `expected` The expected value
    ///
    pub fn gt<X: PartialOrd>(self, actual: X, expected: X) -> Self {
        self.run(actual.gt(&expected), IS_SUPERIOR, IS_INFERIOR)
    }
    ///
    /// # Check if actual is greater or equal than expected
    ///
    /// - `actual` The actual value
    /// - `expected` The expected value
    ///
    pub fn ge<X: PartialOrd>(self, actual: X, expected: X) -> Self {
        self.run(actual.ge(&expected), IS_SUPERIOR, IS_INFERIOR)
    }

    ///
    /// # Check if actual is containing expected
    ///
    /// - `actual` The actual value
    /// - `expected` The expected value
    ///
    pub fn str_contains(self, actual: String, expected: &str) -> Self {
        self.run(actual.contains(expected), IS_CONTAINS, IS_NOT_CONTAINS)
    }

    ///
    /// # Check if an actual path matches the expected value
    ///
    /// - `actual` The actual value
    /// - `expected` The expected value
    ///
    pub fn path_exists(self, actual: &str, expected: bool) -> Self {
        self.run(Path::new(actual).exists().eq(&expected), IS_OK, IS_KO)
    }

    ///
    /// # Check if an actual path exists
    ///
    /// - `actual` The actual path
    ///
    pub fn exists(self, actual: &str) -> Self {
        self.run(Path::new(actual).exists(), IS_EXISTS, IS_NOT_EXISTS)
    }

    ///
    /// # Check if actual is not containing expected
    ///
    /// - `actual` The actual value
    /// - `expected` The expected value
    ///
    pub fn str_not_contains(self, actual: String, expected: &str) -> Self {
        self.run(!actual.contains(expected), IS_NOT_CONTAINS, IS_CONTAINS)
    }

    ///
    /// # Check if actual is lower or equal than expected
    ///
    /// - `actual` The actual value
    /// - `expected` The expected value
    ///
    pub fn le<X: PartialOrd>(self, actual: X, expected: X) -> Self {
        self.run(actual.le(&expected), IS_INFERIOR, IS_SUPERIOR)
    }
    ///
    /// # Check if actual is lower than expected
    ///
    /// - `actual` The actual value
    /// - `expected` The expected value
    ///
    pub fn lt<X: PartialOrd>(self, actual: X, expected: X) -> Self {
        self.run(actual.lt(&expected), IS_INFERIOR, IS_SUPERIOR)
    }

    ///
    /// # Check if actual is lower than expected
    ///
    /// - `description` The actual value
    /// - `expected` The expected value
    ///
    pub fn group(self, description: &str, callback: fn(Suite) -> Suite) -> Self {
        println!("\n{}\n", description.white().bold());
        callback(self)
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
    after_all_hook: fn(),
    after_each_hook: fn(),
    before_all_hook: fn(),
    before_each_hook: fn(),
    main: fn(Suite) -> Suite,
) -> Suite {
    before_all_hook();
    println!("\n{}\n", title.white().bold());
    println!("\n{}\n", description.white().bold());
    let data: Suite = main(Suite::new(before_each_hook, after_each_hook));
    after_all_hook();
    data
}

#[cfg(test)]
mod test {
    use crate::suite::describe;
    use std::fs;

    #[test]
    fn suite() -> std::io::Result<()> {
        describe(
            "Check the suite it test case",
            "Suite test accept no test failure, for guaranty the source code.",
            || {},
            || {},
            ||
            {},
            || {},
            |s| {
                s.group("Should be contains", |s| {
                    s.str_contains(
                        fs::read_to_string("README.md").expect("Failed to parse README.md"),
                        "cargo add unit-testing",
                    )
                })
                    .group("Check path", |s| {
                        s.path_exists("README.md", true)
                            .path_exists(".", true)
                            .path_exists("alexandrie", false)
                            .exists(".")
                            .exists("README.md")
                    })
                    .group("Should be not contains", |s| {
                        s.str_not_contains(
                            fs::read_to_string("README.md").expect("Failed to parse README.md"),
                            "cargo add continuous-testing",
                        )
                    })
                    .group("Should be equals", |s| s.eq(1, 1).eq(2, 2))
                    .group("Should be unequal", |s| s.ne(1, 2).ne(3, 2))
            },
        )
            .end()
    }
}
