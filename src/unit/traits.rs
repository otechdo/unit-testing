pub mod unit {
    use std::process::ExitStatus;
    use std::{collections::HashSet, io, process::ExitCode};

    pub trait Take {
        fn assert_that(&mut self, t: bool) -> bool;

        ///
        /// # Run assert and measure execution time
        ///
        fn take<'a>(&'a mut self, t: bool, s: &'a str, e: &'a str) -> &mut Self;
    }

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

    pub trait Failure {
        ///
        /// # Check if a command fail
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

    pub trait Testable {
        ///
        /// # Constructor
        ///
        /// - `callbacks` The vec list of callback
        ///
        ///
        fn it(callbacks: Vec<&dyn Fn(&mut Self) -> &mut Self>) -> ExitCode;

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
        ///
        ///
        fn assert(&mut self, test: bool) -> bool;

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
        /// # CHeck if a paths exists
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
        fn end(&mut self) -> Result<&mut Self, String>;
    }
}
