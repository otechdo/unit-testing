pub mod tdd {
    pub mod unit {
        pub const IS_OK: &str = "The result match true";
        pub const IS_KO: &str = "The result match false";
        pub const IS_EQUALS: &str = "The values are equals";
        pub const IS_UNEQUALS: &str = "The values are unequals";
        pub const IS_BETWEEN: &str = "The values are between the expected values";
        pub const IS_INFERIOR: &str = "The value are inferior to the expected value";
        pub const IS_SUPERIOR: &str = "The value are superior to the expected value";
        pub const IS_EMPTY: &str = "The value is empty";
        pub const IS_DIRECTORY: &str = "The directory exist";
        pub const IS_FILE: &str = "The file exist";
        pub const IS_FULL: &str = "The value is at this maximum value";
        pub const IS_PRIME: &str = "The value is a prime number";
        pub const IS_PAIR: &str = "The value is a pair number";
        pub const IS_START_WITH: &str = "The value start with the expected value";
        pub const IS_FINNISH_WITH: &str = "The value finnish with the expected value";
        pub const IS_IMPAIR: &str = "The value is not a pair number";
        pub const IS_CONTAINS: &str = "The value contains the expected value";
        pub const IS_ABSOLUTE: &str = "The path is absolute";
        pub const IS_EXISTS: &str = "The path exists";
        pub const IS_RELATIVE: &str = "The path is relative";
        pub const IS_SYMLINK: &str = "The path is a symlink";
        pub const IS_EXECUTABLE: &str = "The file is executable";
        pub const THEORY_IS_TRUE: &str = "The theory is true";
        pub const THEORY_IS_FALSE: &str = "The theory is false";
        pub const IS_NOT_EXECUTABLE: &str = "The file is not executable";
        pub const IS_NOT_SYMLINK: &str = "The path is a symlink";
        pub const IS_NOT_RELATIVE: &str = "The path is not relative";
        pub const IS_NOT_EXISTS: &str = "The path not exists";
        pub const IS_NOT_ABSOLUTE: &str = "The path is not absolute";
        pub const IS_NOT_FINNISH_WITH: &str = "The value don't finnish with the expected value";
        pub const IS_NOT_CONTAINS: &str = "The value not contains the expected value";
        pub const IS_NOT_PRIME: &str = "The value is not a prime number";
        pub const IS_NOT_FULL: &str = "The value is not at this maximum value";
        pub const IS_NOT_FILE: &str = "The file not exist";
        pub const IS_NOT_DIRECTORY: &str = "The directory not exist";
        pub const IS_NOT_EMPTY: &str = "The value is not empty";
        pub const IS_NOT_BETWEEN: &str = "The values aren't between the expected values";
        pub const IS_NOT_START_WITH: &str = "The value no start with the expected value";

        pub const PROGRESS: u64 = 500;
        pub const NO_PROGRESS: u64 = 0;

        pub enum Style {
            POINT,
            ASTRIX,
            OKAY,
        }

        use colored_truecolor::Colorize;
        use is_executable::IsExecutable;
        use scraper::{Html, Selector};
        use std::cell::Cell;
        use std::fs;
        use std::fs::File;
        use std::ops::Add;
        use std::path::Path;
        use std::thread::sleep;
        use std::time::{Duration, Instant};

        extern crate reqwest;

        pub struct Unit {
            description: String,
            assertions: Cell<usize>,
            failures: Cell<usize>,
            start: Instant,
            tdd: Cell<usize>,
            sleep_time: u64,
            style: Style,
            page: String,
        }

        ///
        /// # To run unit test
        ///
        impl Unit {
            pub fn see(&mut self, expected: &str) -> &mut Unit {
                self.page.contains(expected);
                self
            }

            pub fn visit(&mut self, url: &str) -> &mut Unit {
                self.page.clear();

                self.page = reqwest::blocking::get(url)
                    .expect("failed to parse url")
                    .text()
                    .expect("failed to get text");
                self
            }

            pub fn file_open(&mut self, f: &str) -> File {
                File::open(f).expect("failed to open filename")
            }

            ///
            /// # Open a path
            ///
            /// - `p` the path to open
            ///
            pub fn open<'a>(&'a mut self, p: &'a str) -> &'a Path {
                Path::new(p)
            }

            ///
            /// # Read a file to string
            ///
            /// - `f`   The file
            ///
            pub fn read(&mut self, f: &str) -> String {
                fs::read_to_string(f).expect("failed to read to string")
            }

            ///
            /// # Remove a file or directory
            ///
            /// - `f` file or directory
            ///
            pub fn rm(&mut self, f: &str) -> bool {
                if Path::new(f).is_dir() {
                    fs::remove_dir_all(f).expect("failed to remove the directory");
                    true
                } else {
                    fs::remove_file(f).expect("failed to remove the file");
                    true
                }
            }

            ///
            /// Get all links inside a page
            ///
            pub fn links(&mut self) -> Vec<String> {
                let mut r: Vec<String> = Vec::new();
                let links: Selector = Selector::parse("a").unwrap();
                let fragment: Html = Html::parse_fragment(self.page.as_str());
                for element in fragment.select(&links) {
                    r.push(element.value().attr("href").expect("msg").to_string());
                }
                r
            }

            pub fn select(&mut self, selector: &str, attr: &str) -> Vec<String> {
                let mut r: Vec<String> = Vec::new();
                let links: Selector = Selector::parse(selector).unwrap();
                let fragment: Html = Html::parse_fragment(self.page.as_str());
                for element in fragment.select(&links) {
                    r.push(element.value().attr(attr).expect("msg").to_string());
                }
                r
            }

            pub fn click(&mut self, url: &str) -> &mut Unit {
                self.visit(url)
            }

            pub fn reqwest_success(&mut self, href: &str, expected: bool) -> &mut Unit {
                self.equals(
                    reqwest::blocking::get(href)
                        .expect("failed to parse url")
                        .status()
                        .is_success(),
                    expected,
                )
            }

            pub fn reqwest_error(&mut self, href: &str, expected: bool) -> &mut Unit {
                self.equals(
                    reqwest::blocking::get(href)
                        .expect("failed to parse url")
                        .status()
                        .is_client_error(),
                    expected,
                )
            }

            pub fn reqwest_server_error(&mut self, href: &str, expected: bool) -> &mut Unit {
                self.equals(
                    reqwest::blocking::get(href)
                        .expect("failed to parse url")
                        .status()
                        .is_server_error(),
                    expected,
                )
            }

            pub fn reqwest_server_redirection(&mut self, href: &str, expected: bool) -> &mut Unit {
                self.equals(
                    reqwest::blocking::get(href)
                        .expect("failed to parse url")
                        .status()
                        .is_redirection(),
                    expected,
                )
            }

            ///
            /// Get all assertions number
            ///
            fn assertions(&mut self) -> usize {
                self.assertions.get()
            }

            pub fn equals_bytes(&mut self, actual: &str, expected: &[u8]) -> &mut Unit {
                self.assert(
                    String::from(actual).as_bytes().eq(expected),
                    IS_EQUALS,
                    IS_UNEQUALS,
                )
            }

            pub fn unequals_bytes(&mut self, actual: &str, expected: &[u8]) -> &mut Unit {
                self.assert(
                    String::from(actual).as_bytes().ne(expected),
                    IS_EQUALS,
                    IS_UNEQUALS,
                )
            }

            pub fn begin_with(&mut self, actual: &str, expected: &str) -> &mut Unit {
                self.assert(
                    String::from(actual).starts_with(expected),
                    IS_START_WITH,
                    IS_NOT_START_WITH,
                )
            }

            pub fn finnish_with(&mut self, actual: &str, expected: &str) -> &mut Unit {
                self.assert(
                    String::from(actual).ends_with(expected),
                    IS_FINNISH_WITH,
                    IS_NOT_FINNISH_WITH,
                )
            }

            ///
            /// Get all failures number
            ///
            fn failures(&mut self) -> usize {
                self.failures.get()
            }

            ///
            /// # Test if a file is an executable
            ///
            /// - `filename`    The filename to test
            ///
            pub fn exe(&mut self, filename: &str) -> &mut Unit {
                self.assert(
                    Path::new(filename).is_executable(),
                    IS_EXECUTABLE,
                    IS_NOT_EXECUTABLE,
                )
            }

            pub fn not_exe(&mut self, filename: &str) -> &mut Unit {
                self.assert(
                    !Path::new(filename).is_executable(),
                    IS_NOT_EXECUTABLE,
                    IS_EXECUTABLE,
                )
            }

            ///
            /// Run a test
            ///
            /// - `tdd` The test to execute
            /// - `s`   The success output
            /// - `f`   The failure output
            ///
            fn assert(&mut self, tdd: bool, s: &str, f: &str) -> &mut Unit {
                self.tdd.set(self.tdd.get() + 1);

                match self.style {
                    Style::POINT => {
                        if tdd {
                            self.assertions.set(self.assertions.get() + 1);
                            print!("{}", ".".white().bold());
                        } else {
                            self.failures.set(self.failures.get() + 1);
                            print!("{}", "F".red().bold());
                        }
                        sleep(Duration::from_millis(self.sleep_time));
                    }
                    Style::OKAY => {
                        if tdd {
                            self.assertions.set(self.assertions.get() + 1);
                            println!(
                                "{}  {}  {} {}\n",
                                "[".white().bold(),
                                "OK".green().bold(),
                                "]".white().bold(),
                                s.blue().bold()
                            );
                        } else {
                            self.failures.set(self.failures.get() + 1);
                            println!(
                                "{}  {}  {} {}\n",
                                "[".white().bold(),
                                "KO".red().bold(),
                                "]".white().bold(),
                                f.yellow().bold()
                            );
                        }
                        sleep(Duration::from_millis(self.sleep_time));
                    }
                    Style::ASTRIX => {
                        if tdd {
                            self.assertions.set(self.assertions.get() + 1);
                            println!("{} {}\n", "*".green().bold(), s.blue().bold());
                        } else {
                            self.failures.set(self.failures.get() + 1);
                            println!("{} {}\n", "*".red().bold(), f.yellow().bold());
                        }
                        sleep(Duration::from_millis(self.sleep_time));
                    }
                }
                sleep(Duration::from_millis(self.sleep_time));
                self
            }

            ///
            ///
            /// # Group test in a function
            ///
            /// - `description` The group test description
            /// - `it`          The callback to execute
            ///
            pub fn describe(
                &mut self,
                description: &str,
                it: fn(&mut Unit) -> &mut Unit,
            ) -> &mut Unit {
                self.title(description);
                it(self)
            }

            pub fn title(&mut self, description: &str) {
                if self.assertions() == 0 && self.failures() == 0 {
                    println!("{}\n", description.blue().bold());
                } else {
                    println!("\n\n{}\n", description.blue().bold());
                }
            }

            pub fn theory(&mut self, description: &str, it: fn() -> bool) -> &mut Unit {
                self.title(description);
                self.assert(it(), THEORY_IS_TRUE, THEORY_IS_FALSE)
            }

            ///
            /// # Check if a theory match false
            ///
            /// - `description` THe theory description
            /// - `it`          The callback to execute
            ///
            pub fn chaos(&mut self, description: &str, it: fn() -> bool) -> &mut Unit {
                self.title(description);
                self.assert(!it(), THEORY_IS_FALSE, THEORY_IS_TRUE)
            }

            ///
            ///
            /// # Unit constructor
            ///
            /// - `description`     The unit test description
            /// - `time`            The sleep time
            ///
            ///
            pub fn new(description: &str, time: u64, s: Style) -> Unit {
                println!(
                    "\n{} {} {}\n",
                    "[".blue().bold(),
                    description.cyan().bold(),
                    "]".blue().bold()
                );

                Self {
                    description: description.to_string(),
                    assertions: Cell::new(0),
                    failures: Cell::new(0),
                    start: Instant::now(),
                    tdd: Cell::new(0),
                    sleep_time: time,
                    style: s,
                    page: String::new(),
                }
            }

            ///
            ///
            /// End of the tests
            ///
            pub fn end(&mut self) -> Result<String, String> {
                let asserts = self.assertions();
                let fails = self.failures();

                println!(
                    "\n\n{} : {}  {} : {}  {} : {} {} : {}\n",
                    "Assertions".blue().bold(),
                    asserts.to_string().green().bold(),
                    "Failures".blue().bold(),
                    fails.to_string().red().bold(),
                    "Execution time".blue().bold(),
                    self.start
                        .elapsed()
                        .as_millis()
                        .to_string()
                        .add(" ms")
                        .magenta()
                        .bold(),
                    "All tests executed".blue().bold(),
                    self.tdd.get().to_string().white().bold()
                );

                return if self.failures() >= 1 {
                    Err(String::from(self.description.as_str()))
                } else {
                    Ok(String::from(self.description.as_str()))
                };
            }

            ///
            ///
            /// # Check if all values are equals to true
            ///
            /// - `b`   A value to check
            ///
            pub fn ok(&mut self, b: bool) -> &mut Unit {
                self.assert(b, IS_OK, IS_KO)
            }

            ///
            ///
            /// # Check if all values are equals to false
            ///
            /// - `b`   The value to check
            ///
            pub fn ko(&mut self, b: bool) -> &mut Unit {
                self.assert(!b, IS_KO, IS_OK)
            }

            ///
            /// # Check if two value are equals
            ///
            /// - `a`       The first value
            /// - `b`       The second value
            ///
            pub fn equals<T: PartialEq>(&mut self, a: T, b: T) -> &mut Unit {
                self.assert(a == b, IS_EQUALS, IS_UNEQUALS)
            }

            ///
            ///
            /// # Check if actual is in min and []ax value
            ///
            ///
            /// - `actual`  The actual value
            /// - `min`     The minimun value for actual
            /// - `max`     The maximum value for actual
            ///
            pub fn between<T: PartialOrd>(&mut self, actual: T, min: T, max: T) -> &mut Unit {
                self.assert(actual > min && actual < max, IS_BETWEEN, IS_NOT_BETWEEN)
            }

            ///
            ///
            /// Check if values are unequals
            ///
            /// - `a`   The first value
            /// - `b`   The second value
            ///
            pub fn unequals<T: PartialEq>(&mut self, a: T, b: T) -> &mut Unit {
                self.assert(a != b, IS_UNEQUALS, IS_EQUALS)
            }

            ///
            /// # Check if the value is inferior to the maximum value
            ///
            /// - `actual`      The actual value
            /// - `max`         The maximum value
            ///
            pub fn inferior<T: PartialOrd>(&mut self, actual: T, max: T) -> &mut Unit {
                self.assert(actual < max, IS_INFERIOR, IS_SUPERIOR)
            }

            ///
            /// # Check if the value is superior to the minimun value
            ///
            /// - `actual`      The actual value
            /// - `min`         The minimum value
            ///
            pub fn superior<T: PartialOrd>(&mut self, actual: T, min: T) -> &mut Unit {
                self.assert(actual > min, IS_SUPERIOR, IS_INFERIOR)
            }

            ///
            ///
            /// # Check if a string is not empty
            ///
            /// -  `x` The string to check
            ///
            pub fn not_empty(&mut self, x: &str) -> &mut Unit {
                self.assert(!x.to_string().is_empty(), IS_NOT_EMPTY, IS_EMPTY)
            }

            ///
            ///
            /// # Check if a string contains another string
            ///
            /// -  `x`          The string to check
            /// -  `expected`   The string to verify
            ///
            pub fn contains(&mut self, x: &str, expected: &str) -> &mut Unit {
                self.assert(
                    x.to_string().contains(expected),
                    IS_CONTAINS,
                    IS_NOT_CONTAINS,
                )
            }

            ///
            ///
            /// # Check if a string not contains another string
            ///
            /// -  `x`          The string to check
            /// -  `expected`   The string to verify
            ///
            pub fn not_contains(&mut self, x: &str, expected: &str) -> &mut Unit {
                self.assert(
                    !x.to_string().contains(expected),
                    IS_CONTAINS,
                    IS_NOT_CONTAINS,
                )
            }

            ///
            ///
            /// # Check if a string is empty
            ///
            /// -  `x` The string to check
            ///
            pub fn empty(&mut self, x: &str) -> &mut Unit {
                self.assert(x.to_string().is_empty(), IS_EMPTY, IS_NOT_EMPTY)
            }

            ///
            /// # Check if the given value is a directory
            ///
            /// - `d` The path to check
            ///
            pub fn directory(&mut self, d: &str) -> &mut Unit {
                self.assert(Path::new(d).is_dir(), IS_DIRECTORY, IS_NOT_DIRECTORY)
            }

            ///
            /// # Check if the given value is not a directory
            ///
            /// - `d` The path to check
            ///
            pub fn not_directory(&mut self, d: &str) -> &mut Unit {
                self.assert(!Path::new(d).is_dir(), IS_NOT_DIRECTORY, IS_DIRECTORY)
            }

            ///
            /// # Check if the given value is a file
            ///
            /// - `f` The path to check
            ///
            pub fn file(&mut self, f: &str) -> &mut Unit {
                self.assert(Path::new(f).is_file(), IS_FILE, IS_NOT_FILE)
            }

            ///
            ///
            /// # Check if a path start with a expected base
            ///
            ///
            /// - `path`    The path to check
            /// - `base`    The path's base
            ///
            ///
            pub fn path_start_with(&mut self, path: &str, base: &str) -> &mut Unit {
                self.assert(
                    Path::new(path).starts_with(base),
                    IS_START_WITH,
                    IS_NOT_START_WITH,
                )
            }

            ///
            ///
            /// # Check if a path is absolute
            ///
            /// - `path`    The path to check
            ///
            ///
            pub fn path_absolute(&mut self, path: &str) -> &mut Unit {
                self.assert(Path::new(path).is_absolute(), IS_ABSOLUTE, IS_NOT_ABSOLUTE)
            }

            ///
            ///
            /// # Check if a path is not absolute
            ///
            /// - `path`    The path to check
            ///
            ///
            pub fn path_not_absolute(&mut self, path: &str) -> &mut Unit {
                self.assert(!Path::new(path).is_absolute(), IS_NOT_ABSOLUTE, IS_ABSOLUTE)
            }

            ///
            ///
            /// # Check if a path exist
            ///
            /// - `path`    The path to check
            ///
            ///
            pub fn path_exists(&mut self, path: &str) -> &mut Unit {
                self.assert(Path::new(path).exists(), IS_EXISTS, IS_NOT_EXISTS)
            }

            ///
            ///
            /// # Check if a path is not relative
            ///
            /// - `path`    The path to check
            ///
            pub fn path_not_relative(&mut self, path: &str) -> &mut Unit {
                self.assert(!Path::new(path).is_relative(), IS_RELATIVE, IS_NOT_RELATIVE)
            }

            ///
            ///
            /// # Chek if a path is relative
            ///
            /// - `path`    The path to check
            ///
            pub fn path_symlink(&mut self, path: &str) -> &mut Unit {
                self.assert(!Path::new(path).is_symlink(), IS_SYMLINK, IS_NOT_SYMLINK)
            }

            ///
            ///
            /// # Chek if a path exist
            ///
            /// - `path`    The path to check
            ///
            ///
            pub fn path_relative(&mut self, path: &str) -> &mut Unit {
                self.assert(Path::new(path).is_relative(), IS_RELATIVE, IS_NOT_RELATIVE)
            }

            ///
            ///
            /// # Chek if a path is not absolute
            ///
            /// - `path`    The path to check
            ///
            ///
            pub fn path_no_exists(&mut self, path: &str) -> &mut Unit {
                self.assert(!Path::new(path).exists(), IS_NOT_EXISTS, IS_EXISTS)
            }

            ///
            /// # Check if a file contains an another string
            ///
            /// - `file`        The path to check
            /// - `expected`    The expected value
            ///
            ///
            pub fn file_contains(&mut self, file: &str, expected: &str) -> &mut Unit {
                let content = fs::read_to_string(file).expect("Failed to parse file");

                self.assert(content.contains(expected), IS_CONTAINS, IS_NOT_CONTAINS)
            }

            ///
            /// # Check if a file no contains an another string
            ///
            /// - `file`        The path to check
            /// - `expected`    The unexpected value
            ///
            ///
            pub fn file_no_contains(&mut self, file: &str, expected: &str) -> &mut Unit {
                let content = fs::read_to_string(file).expect("Failed to parse file");
                self.assert(content.contains(expected), IS_NOT_CONTAINS, IS_CONTAINS)
            }

            ///
            /// # Check if two vec length are equals
            ///
            /// - `a`   The first vector
            /// - `b`   The second vector
            ///
            pub fn vec_length_equals<T: PartialEq>(&mut self, a: Vec<T>, b: Vec<T>) -> &mut Unit {
                self.assert(a.len() == b.len(), IS_EQUALS, IS_UNEQUALS)
            }

            ///
            /// # Check if two vec length are unequals
            ///
            /// - `a`   The first vector
            /// - `b`   The second vector
            ///
            pub fn vec_length_unequals<T: PartialEq>(&mut self, a: Vec<T>, b: Vec<T>) -> &mut Unit {
                self.assert(a.len() != b.len(), IS_UNEQUALS, IS_EQUALS)
            }

            ///
            /// # Check if a vec contains a value
            ///
            /// - `a`   The vector
            /// - `b`   The value
            ///
            pub fn vec_contains<T: PartialEq>(&mut self, a: Vec<T>, b: T) -> &mut Unit {
                self.assert(a.contains(&b), IS_CONTAINS, IS_NOT_CONTAINS)
            }

            ///
            /// # Check if a vec not contains a value
            ///
            /// - `a`   The vector
            /// - `b`   The value
            ///
            pub fn vec_no_contains<T: PartialEq>(&mut self, a: Vec<T>, b: T) -> &mut Unit {
                self.assert(!a.contains(&b), IS_NOT_CONTAINS, IS_CONTAINS)
            }

            ///
            /// # Check if a vec start with a value
            ///
            /// - `a`   The vector
            /// - `b`   The value
            ///
            pub fn vec_start_with<T: PartialEq>(&mut self, a: Vec<T>, b: T) -> &mut Unit {
                self.assert(a.starts_with(&[b]), IS_START_WITH, IS_NOT_START_WITH)
            }

            ///
            /// # Check if a vec no start with a value
            ///
            /// - `a`   The vector
            /// - `b`   The value
            ///
            pub fn vec_no_start_with<T: PartialEq>(&mut self, a: Vec<T>, b: T) -> &mut Unit {
                self.assert(!a.starts_with(&[b]), IS_NOT_CONTAINS, IS_CONTAINS)
            }

            ///
            /// # Check if a vec finnish with a value
            ///
            /// - `a`   The vector
            /// - `b`   The value
            ///
            pub fn vec_end_with<T: PartialEq>(&mut self, a: Vec<T>, b: T) -> &mut Unit {
                self.assert(a.ends_with(&[b]), IS_FINNISH_WITH, IS_NOT_FINNISH_WITH)
            }

            ///
            /// # Check if a vec finnish with a value
            ///
            /// - `a`   The vector
            /// - `b`   The value
            ///
            pub fn vec_no_end_with<T: PartialEq>(&mut self, a: Vec<T>, b: T) -> &mut Unit {
                self.assert(!a.ends_with(&[b]), IS_NOT_FINNISH_WITH, IS_FINNISH_WITH)
            }

            ///
            /// # Check if a vec is empty
            ///
            /// - `a`   The vector
            ///
            pub fn vec_empty<T: PartialEq>(&mut self, a: Vec<T>) -> &mut Unit {
                self.assert(a.is_empty(), IS_EMPTY, IS_NOT_EMPTY)
            }

            ///
            /// # Check if a vec is not empty
            ///
            /// - `a`   The vector
            ///
            pub fn vec_not_empty<T: PartialEq>(&mut self, a: Vec<T>) -> &mut Unit {
                self.assert(!a.is_empty(), IS_NOT_EMPTY, IS_EMPTY)
            }

            //
            /// # Check if the return of the callback is not at this maximum value
            ///
            /// - `callback`        The callback to check
            /// - `max`             The callback maximum value
            ///
            pub fn full(&mut self, callback: fn() -> usize, max: usize) -> &mut Unit {
                self.assert((callback() / max) == 1, IS_FULL, IS_NOT_FULL)
            }

            //
            /// # Check if the return of the callback is not at this maximum value
            ///
            /// - `callback`        The callback to check
            /// - `max`             The callback maximum value
            ///
            pub fn not_full(&mut self, callback: fn() -> usize, max: usize) -> &mut Unit {
                self.assert((callback() / max) < 1, IS_NOT_FULL, IS_FULL)
            }

            ///
            /// # Check if the value is a prime number
            ///
            /// - `x`       The value to check
            ///
            pub fn prime(&mut self, x: usize) -> &mut Unit {
                self.assert(x % 2 != 0 && x % 3 != 0, IS_NOT_PRIME, IS_PRIME)
            }

            ///
            /// # Check if the value is a pair number
            ///
            /// - `x`       The value to check
            ///
            pub fn pair(&mut self, x: usize) -> &mut Unit {
                self.assert(x % 2 == 0, IS_PAIR, IS_IMPAIR)
            }

            ///
            /// # Check if the value is an impair number
            ///
            /// - `x`       The value to check
            ///
            pub fn impair(&mut self, x: usize) -> &mut Unit {
                if x % 2 == 0 {
                    return self.assert(false, IS_PAIR, IS_PAIR);
                }
                if x % 3 == 0 {
                    return self.assert(true, IS_IMPAIR, IS_PAIR);
                }
                return self.assert(false, IS_PRIME, IS_PRIME);
            }
        }

        ///
        /// # Check if all values matches true
        ///
        /// - `description`     The unit description
        /// - `items`           A vector with boolean values
        ///
        #[macro_export]
        macro_rules! assert_true {
            ($description:expr,$items:expr) => {
                let mut u = Unit::new($description, NO_PROGRESS, POINT);
                for &item in $items.iter() {
                    u.ok(item);
                }
                u.end().expect("A result not match true");
            };
        }

        ///
        /// # Check if all values matches false
        ///
        /// - `description`     The unit description
        /// - `items`           A vector with boolean values
        ///
        #[macro_export]
        macro_rules! assert_false {
            ($description:expr,$items:expr) => {
                let mut u = Unit::new($description, NO_PROGRESS, POINT);
                for &item in $items.iter() {
                    u.ko(item);
                }
                u.end().expect("A result not match false");
            };
        }

        ///
        /// # Check if all directories exist
        ///
        /// - `description`     The unit description
        /// - `items`           A vector with all paths
        ///
        #[macro_export]
        macro_rules! assert_directory_exist {
            ($description:expr,$items:expr) => {
                let mut u = Unit::new($description, NO_PROGRESS, POINT);
                for &item in $items.iter() {
                    u.directory(item);
                }
                u.end().expect("A directory has not been founded");
            };
        }

        ///
        /// # Check if all files exist
        ///
        /// - `description`     The unit description
        /// - `items`           A vector with all paths
        ///
        #[macro_export]
        macro_rules! assert_files_exist {
            ($description:expr,$items:expr) => {
                let mut u = Unit::new($description, NO_PROGRESS, POINT);
                for &item in $items.iter() {
                    u.file(item);
                }
                u.end().expect("A file has not been founded");
            };
        }

        ///
        /// # Check if a value contains an another
        ///
        /// - `description`     The unit description
        /// - `items`           A vector with all expected values
        /// - `v`               The actual value to check if contains data
        ///
        #[macro_export]
        macro_rules! assert_contains {
            ($description:expr,$items:expr,$v:expr) => {
                let mut u = Unit::new($description, NO_PROGRESS, POINT);
                for &item in $items.iter() {
                    u.contains($v, item);
                }
                u.end().expect("A value has not been founded");
            };
        }

        ///
        /// # Check if a value not contains an another
        ///
        /// - `description`     The unit description
        /// - `items`           A vector contending all expected values
        /// - `v`               The actual value to check if not contains data
        ///
        #[macro_export]
        macro_rules! assert_not_contains {
            ($description:expr,$items:expr,$v:expr) => {
                let mut u = Unit::new($description, NO_PROGRESS, POINT);
                for &item in $items.iter() {
                    u.not_contains($v, item);
                }
                u.end().expect("A value has not been founded");
            };
        }

        ///
        /// # Check if a value not contains an another
        ///
        /// - `description`     The unit description
        /// - `items`           A vector contending all expected values
        ///
        #[macro_export]
        macro_rules! assert_not_exe {
            ($description:expr,$items:expr) => {
                let mut u = Unit::new($description, NO_PROGRESS, POINT);
                for &item in $items.iter() {
                    u.not_exe(item);
                }
                u.end().expect("A value is not an executable");
            };
        }

        ///
        /// # Check if a value equal to an another
        ///
        /// - `description`     The unit description
        /// - `items`           A vector contending all values
        /// - `v`               The actual value to check equality
        ///
        #[macro_export]
        macro_rules! assert_equals {
            ($description:expr,$items:expr,$v:expr) => {
                let mut u = Unit::new($description, NO_PROGRESS, POINT);
                for &item in $items.iter() {
                    u.equals($v, item);
                }
                u.end().expect("A value not equal to the expected value");
            };
        }

        ///
        /// # Check if a value not equal to an another
        ///
        /// - `description`     The unit description
        /// - `items`           A vector contenting all values
        /// - `v`               The actual value to check inequality
        ///
        #[macro_export]
        macro_rules! assert_unequals {
            ($description:expr,$items:expr,$v:expr) => {
                let mut u = Unit::new($description, NO_PROGRESS, POINT);
                for &item in $items.iter() {
                    u.unequals($v, item);
                }
                u.end().expect("A value equal to the expected value");
            };
        }

        ///
        /// # Check if a value not equal to an another
        ///
        /// - `description`     The unit description
        ///
        #[macro_export]
        macro_rules! unit {
            ($description:expr) => {
                Unit::new($description, NO_PROGRESS, POINT)
            };
        }

        ///
        /// # Check if a value not contains an another
        ///
        /// - `description`     The unit description
        /// - `items`           A vector contending all expected values
        /// - `v`               The actual value to check if not contains data
        ///
        #[macro_export]
        macro_rules! assert_exe {
            ($description:expr,$items:expr) => {
                let mut u = Unit::new($description, NO_PROGRESS, POINT);
                for &item in $items.iter() {
                    u.exe(item);
                }
                u.end().expect("A value is not an executable");
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        assert_contains, assert_directory_exist, assert_equals, assert_false, assert_files_exist,
        assert_not_contains, assert_not_exe, assert_true, assert_unequals,
        tdd::unit::{Style::POINT, Unit, NO_PROGRESS},
        unit,
    };
    use std::env::consts::OS;

    fn battery_full() -> usize {
        100
    }

    fn battery_not_full() -> usize {
        50
    }

    fn together() -> bool {
        false
    }

    fn pythagore() -> bool {
        4 * 4 + 3 * 3 == 5 * 5
    }

    fn admin(u: &mut Unit) -> &mut Unit {
        u.inferior(1, 2).prime(1).superior(1, 0).between(1, 0, 2)
    }

    fn boolean(u: &mut Unit) -> &mut Unit {
        u.ok(pythagore())
            .ko(together())
            .theory("Check the theorem of pythagore", pythagore)
            .chaos("The big-bang", together)
            .directory(".")
            .file("README.md")
    }

    fn git_test_install(u: &mut Unit) -> &mut Unit {
        u.exe("/usr/bin/git").not_exe("/usr/bin")
    }

    fn e(u: &mut Unit) -> &mut Unit {
        u.empty("").not_empty(OS)
    }

    fn battery(u: &mut Unit) -> &mut Unit {
        u.full(battery_full, 100).not_full(battery_not_full, 100)
    }

    fn equals(u: &mut Unit) -> &mut Unit {
        u.equals(true, pythagore())
            .unequals(false, pythagore())
            .equals_bytes("hello", &[104, 101, 108, 108, 111])
            .unequals_bytes("hallo", &[104, 101, 108, 108, 111])
    }

    fn numbers(u: &mut Unit) -> &mut Unit {
        u.prime(1).prime(7).prime(11);
        u.pair(2).pair(4).pair(6);
        u.impair(3).impair(9)
    }

    #[test]
    pub fn test_see() {
        unit!("test see")
            .visit("https://github.com/taishingi/zuu")
            .see("Issues")
            .click("https://github.com/taishingi/zuu/issues")
            .see("Welcome to issues!")
            .end()
            .expect("failure");
    }

    #[test]
    pub fn test_selector() {
        assert!(unit!("test see")
            .visit("https://github.com/taishingi/zuu")
            .select("a", "href")
            .contains(&"/taishingi/zuu".to_string()));
    }

    #[test]
    pub fn unit() {
        unit!("Test the unit framework")
            .describe("Check theories", boolean)
            .describe("Test charge", battery)
            .describe("Check admin users account number", admin)
            .describe("Test number", numbers)
            .describe("They are equals", equals)
            .describe("Check empty", e)
            .describe("Check if git is installed", git_test_install)
            .end()
            .expect("failed");
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
            vec!["/etc/hosts", "/etc/locale.conf"]
        );

        assert_not_exe!(
            "Check if configugrations files is not an executable",
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
