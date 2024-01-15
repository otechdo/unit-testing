use std::process::ExitStatus;
use std::{collections::HashSet, io};

///
/// # Calculate the time of the test function
///
pub trait Take {
    ///
    /// # run assertion
    ///
    /// - `t`   The test
    /// - `e`   The error message
    ///
    fn assert_that(&mut self, t: bool, e: &str) -> bool;

    ///
    /// # Run assert and measure execution time
    ///
    /// - `t` The test
    /// - `s` The success output message
    /// - `e` The error output message
    ///
    fn take(&mut self, t: bool, s: &str, e: &str) -> &mut Self;

    ///
    ///
    /// # Run a assert and mesure the time
    ///
    /// - `t` The test
    /// - `s` The success output message
    /// - `e` The error output message
    ///
    fn check(&mut self, t: bool, s: &str, e: &str);
}

///
/// # Add theory useful method
///
pub trait Theory {
    ///
    /// # A theory must be equal to false
    ///
    /// - `callback` The callback to execute
    ///
    fn chaos(&mut self, callback: &dyn Fn() -> bool) -> &mut Self;

    ///
    /// # Test a theory
    ///
    /// - `expected`    The expect callback result
    /// - `callback`    The callback to execute
    ///
    fn theory<T: PartialEq>(&mut self, expected: T, callback: &dyn Fn() -> T) -> &mut Self;
}

///
/// # Assertion to expect a failure
///
pub trait Failure {
    ///
    /// # Check if a command exit status is a failure code
    ///
    /// - `callbacks` The callbacks to check
    ///
    fn command_fail(
        &mut self,
        callbacks: Vec<&dyn Fn() -> Result<ExitStatus, io::Error>>,
    ) -> &mut Self;

    ///
    /// # Check if a callbacks return false
    ///
    /// - `callbacks` The callbacks to check
    ///
    fn fail(&mut self, callbacks: Vec<&dyn Fn() -> bool>) -> &mut Self;
}

///
/// # Expectations to expect a success
///
pub trait Success {
    ///
    /// # Check if a command success
    ///
    /// - `callbacks` The callbacks to check
    ///
    fn run(&mut self, callbacks: Vec<&dyn Fn() -> Result<ExitStatus, io::Error>>) -> &mut Self;

    ///
    /// # Check if a callbacks return true
    ///
    /// - `callbacks` The callbacks to check
    ///
    fn success(&mut self, callbacks: Vec<&dyn Fn() -> bool>) -> &mut Self;
}

///
/// # The method to implements for a new struct
///
pub trait Testable {
    ///
    /// - `sleep_time` The sleep time
    ///
    fn new(sleep_time: u64) -> Self;

    ///
    /// # Check if a pattern matches values
    ///
    /// - `pattern` The pattern to match
    /// - `values`  The values to check
    ///
    fn matches(&mut self, pattern: &str, values: Vec<String>) -> &mut Self;

    ///
    /// # check if a pattern a the x index equal a value listing in values
    ///
    /// - `pattern` The pattern to match
    /// - `x` The index to match
    /// - `values`  The values
    ///
    fn capture(&mut self, pattern: &str, x: &str, key: usize, values: Vec<String>) -> &mut Self;

    ///
    /// # Constructor
    ///
    /// - `callbacks`   The vec list of callback
    /// - `describe`    The description
    ///
    fn it(describe: &str, sleep_time: u64, callbacks: Vec<&dyn Fn(&mut Self) -> &mut Self>);

    ///
    /// # Assert if callback return true
    ///
    /// - `f` The callback
    ///
    fn ok(&mut self, f: &dyn Fn() -> bool) -> &mut Self;

    ///
    /// # Assert if callback return false
    ///
    /// - `f` The callback
    ///
    ///
    fn ko(&mut self, f: &dyn Fn() -> bool) -> &mut Self;

    ///
    /// # Check if test pass
    ///
    /// - `test` The test assertion
    /// - `e`    The error message
    ///
    ///
    fn assert(&mut self, test: bool, e: &str) -> bool;

    ///
    /// # Check if a and b are equals
    ///
    /// - `a`   The first value
    /// - `b`   The second value
    ///
    fn equals<T: PartialEq>(&mut self, a: T, b: T) -> &mut Self;

    ///
    /// # Check if a and b are unequals
    ///
    /// - `a`   The first value
    /// - `b`   The second value
    ///
    fn unequals<T: PartialEq>(&mut self, a: T, b: T) -> &mut Self;

    ///
    /// # Check if a are superior to min
    ///
    /// - `a`   The first value
    /// - `min` The minimum value
    ///
    fn superior<T: PartialOrd>(&mut self, a: T, min: T) -> &mut Self;

    ///
    /// # Check if a are inferior to max
    ///
    /// - `a`   The first value
    /// - `max` The maximum value
    ///
    fn inferior<T: PartialOrd>(&mut self, a: T, max: T) -> &mut Self;

    ///
    /// # Check if a are between min and max
    ///
    /// - `a`   The first value
    /// - `min` The minimum value
    /// - `max` The maximum value
    ///
    fn between<T: PartialOrd>(&mut self, a: T, min: T, max: T) -> &mut Self;

    ///
    /// # Check if a vector contains a value
    ///
    /// - `a` The vector
    /// - `b` The value to check
    ///
    fn vec_contains<T: PartialEq>(&mut self, a: Vec<T>, b: T) -> &mut Self;

    ///
    /// # Check if p is a program
    ///
    /// - `p` The program path
    ///
    fn program(&mut self, p: &str) -> &mut Self;

    ///
    /// # Check if p is not a program
    ///
    /// - `p` The program to test
    ///
    ///
    fn not_program(&mut self, p: &str) -> &mut Self;

    ///
    /// # Check if a vector not contains a value
    ///
    /// - `a` The vector
    /// - `b` The value to check
    ///
    fn vec_no_contains<T: PartialEq>(&mut self, a: Vec<T>, b: T) -> &mut Self;

    ///
    /// # Check if a option contains a value
    ///
    /// - `a` The vector
    /// - `b` The value to check
    ///
    fn option_contains<T: PartialEq>(&mut self, a: Option<T>, b: T) -> &mut Self;

    ///
    /// # Check if a hash contains a string
    ///
    /// - `a` The hash
    /// - `b` The value to find
    ///
    fn hash_contains(&mut self, a: &mut HashSet<String>, b: String) -> &mut Self;

    ///
    /// # Check if a sting contains a substring
    ///
    /// - `a` The string
    /// - `b` The substring
    ///
    fn string_contains(&mut self, a: &str, b: &str) -> &mut Self;

    ///
    /// # Check if a file contains a value
    ///
    /// - `f` The file
    /// - `v` The value to check
    ///
    fn file_contains(&mut self, f: &str, v: &str) -> &mut Self;

    ///
    /// # Check if a paths exists
    ///
    /// - `p` The path to test
    ///
    fn exists(&mut self, p: &str) -> &mut Self;

    ///
    /// # Check if a path not exist
    ///
    /// - `p` The path to check the no existence
    ///
    fn not_exists(&mut self, p: &str) -> &mut Self;

    ///
    /// # Check if a string begin with the expected value
    ///
    /// - `actual`   The actual value
    /// - `expected` The expected value
    ///
    fn begin_with(&mut self, actual: &str, expected: &str) -> &mut Self;

    ///
    ///  # Check if a string finnish with the expected value
    ///
    /// - `actual`      The actual value
    /// - `expected`    The expected value
    ///
    fn end_with(&mut self, actual: &str, expected: &str) -> &mut Self;

    ///
    /// # Show assertions
    ///
    fn end(&mut self) -> bool;
}
