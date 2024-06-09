#![allow(clippy::multiple_crate_versions)]

pub mod unit {
    ///
    /// # mod to describe functions
    ///  
    pub mod describe;

    ///
    /// # All macros
    ///
    pub mod mac;

    ///
    /// # All trait available
    ///
    pub mod object;

    ///
    /// # All trait
    ///
    pub mod check;

    ///
    /// # The console output text
    ///  
    pub mod output;
    use colored_truecolor::Colorize;
    use is_executable::IsExecutable;
    use progress_bar::{
        finalize_progress_bar, inc_progress_bar, init_progress_bar_with_eta,
        print_progress_bar_final_info, print_progress_bar_info, set_progress_bar_action, Color,
        Style,
    };
    use regex::Regex;
    use std::cell::Cell;
    use std::collections::{HashMap, HashSet};
    use std::fs;
    use std::io::Error;
    use std::path::Path;
    use std::process::ExitStatus;
    use std::thread::sleep;
    use std::time::{Duration, Instant};

    use self::describe::It;
    use self::object::{Failure, Success, Take, Testable, Theory};
    use self::output::{
        ASSERT_BEGIN, ASSERT_BETWEEN, ASSERT_CONTAINS, ASSERT_EQUALS, ASSERT_EXISTS, ASSERT_FAIL,
        ASSERT_FINNISH, ASSERT_INFERIOR, ASSERT_IS_EXECUTABLE, ASSERT_IS_NOT_EXECUTABLE, ASSERT_KO,
        ASSERT_MATCH, ASSERT_NOT_CONTAINS, ASSERT_NOT_EXISTS, ASSERT_OK, ASSERT_SHOULD_BE_BEGIN,
        ASSERT_SHOULD_BE_BETWEEN, ASSERT_SHOULD_BE_EQUALS, ASSERT_SHOULD_BE_EXECUTABLE,
        ASSERT_SHOULD_BE_EXISTS, ASSERT_SHOULD_BE_FAIL, ASSERT_SHOULD_BE_FINNISH,
        ASSERT_SHOULD_BE_INFERIOR, ASSERT_SHOULD_BE_KO, ASSERT_SHOULD_BE_NOT_CONTAINS,
        ASSERT_SHOULD_BE_NOT_EXECUTABLE, ASSERT_SHOULD_BE_NOT_EXISTS, ASSERT_SHOULD_BE_OK,
        ASSERT_SHOULD_BE_SUCCESS, ASSERT_SHOULD_BE_SUPERIOR, ASSERT_SHOULD_BE_UNEQUALS,
        ASSERT_SHOULD_CONTAINS, ASSERT_SHOULD_MATCH, ASSERT_SUCCESS, ASSERT_SUPERIOR,
        ASSERT_THEORY_IS_FALSE, ASSERT_THEORY_IS_TRUE, ASSERT_THEORY_SHOULD_BE_FALSE,
        ASSERT_THEORY_SHOULD_BE_TRUE, ASSERT_UNEQUALS, IS_BEGIN, IS_BETWEEN, IS_CONTAINS,
        IS_EQUALS, IS_EXECUTABLE, IS_EXISTS, IS_FAIL, IS_FINNISH, IS_INFERIOR, IS_KO, IS_MATCH,
        IS_NOT_BEGIN, IS_NOT_BETWEEN, IS_NOT_CONTAINS, IS_NOT_EXECUTABLE, IS_NOT_EXISTS,
        IS_NOT_FAIL, IS_NOT_FINNISH, IS_NOT_MATCH, IS_NOT_SUCCESS, IS_OK, IS_SUCCESS, IS_SUPERIOR,
        IS_UNEQUALS, THEORY_IS_FALSE, THEORY_IS_TRUE,
    };

    ///
    /// # To run assertions tests
    ///
    pub struct Assert {
        c: Cell<usize>,
        sleep: u64,
        messages: HashMap<usize, String>,
        take: HashMap<usize, u128>,
    }
    ///
    /// # To run units tests
    ///
    pub struct Unit {
        s: Cell<usize>,
        f: Cell<usize>,
        sleep: u64,
        success_take: HashMap<usize, u128>,
        failure_take: HashMap<usize, u128>,
        success: HashMap<usize, String>,
        failure: HashMap<usize, String>,
    }
    ///
    /// # The tests suite
    ///
    pub struct Describe {}

    impl It for Describe {
        fn it<T: PartialEq>(description: &str, expected: T, callback: &dyn Fn() -> T) {
            let i: Instant = Instant::now();
            if callback().eq(&expected) {
                println!(
                    "     {}",
                    format!(
                        "{} {} {} {} {}",
                        "[ ✓ ]".green().bold(),
                        description.blue().bold(),
                        "take".white().bold(),
                        i.elapsed().as_nanos().to_string().cyan().bold(),
                        "ns".blue().bold()
                    )
                    .as_str()
                );
            } else {
                println!(
                    "     {}",
                    format!(
                        "{} {} {} {} {}",
                        "[ ⨯ ]".red().bold(),
                        description.purple().bold(),
                        "take".white().bold(),
                        i.elapsed().as_nanos().to_string().cyan().bold(),
                        "ns".blue().bold()
                    )
                    .as_str()
                );
            }
        }
    }

    impl Success for Unit {
        fn run(&mut self, callbacks: Vec<&dyn Fn() -> Result<ExitStatus, Error>>) -> &mut Self {
            for &c in &callbacks {
                self.check(c().unwrap().success(), IS_SUCCESS, IS_NOT_SUCCESS);
            }
            self
        }

        fn success(&mut self, callbacks: Vec<&dyn Fn() -> bool>) -> &mut Self {
            for &c in &callbacks {
                self.check(c(), IS_SUCCESS, IS_FAIL);
            }
            self
        }
    }

    impl Success for Assert {
        fn run(&mut self, callbacks: Vec<&dyn Fn() -> Result<ExitStatus, Error>>) -> &mut Self {
            for &c in &callbacks {
                self.check(
                    c().unwrap().success(),
                    ASSERT_SUCCESS,
                    ASSERT_SHOULD_BE_SUCCESS,
                );
            }
            self
        }

        fn success(&mut self, callbacks: Vec<&dyn Fn() -> bool>) -> &mut Self {
            for &c in &callbacks {
                self.check(c(), ASSERT_SUCCESS, ASSERT_SHOULD_BE_SUCCESS);
            }
            self
        }
    }

    impl Take for Unit {
        fn assert_that(&mut self, t: bool) -> bool {
            self.assert(t)
        }

        fn take(&mut self, t: bool, s: &str, e: &str) -> &mut Self {
            let i: Instant = Instant::now();
            if self.assert_that(t) {
                assert_eq!(self.success.insert(self.s.get(), s.to_string()), None);
                assert_eq!(
                    self.success_take
                        .insert(self.s.get(), i.elapsed().as_nanos()),
                    None
                );
            } else {
                assert_eq!(self.failure.insert(self.f.get(), e.to_string()), None);
                assert_eq!(
                    self.failure_take
                        .insert(self.f.get(), i.elapsed().as_nanos()),
                    None
                );
            }

            self
        }

        fn check(&mut self, t: bool, s: &str, e: &str) {
            let i: Instant = Instant::now();
            if self.assert_that(t) {
                assert!(self.success.insert(self.s.get(), s.to_string()).is_some());
                assert!(self
                    .success_take
                    .insert(self.s.get(), i.elapsed().as_nanos())
                    .is_some());
            } else {
                assert!(self.failure.insert(self.f.get(), e.to_string()).is_some());
                assert!(self
                    .failure_take
                    .insert(self.f.get(), i.elapsed().as_nanos())
                    .is_some());
            }
        }
    }

    impl Failure for Assert {
        fn command_fail(
            &mut self,
            callbacks: Vec<&dyn Fn() -> Result<ExitStatus, Error>>,
        ) -> &mut Self {
            for &c in &callbacks {
                let status = c().unwrap();
                self.check(!status.success(), ASSERT_FAIL, ASSERT_SHOULD_BE_FAIL);
            }
            self
        }

        fn fail(&mut self, callbacks: Vec<&dyn Fn() -> bool>) -> &mut Self {
            for &c in &callbacks {
                self.check(!c(), ASSERT_FAIL, ASSERT_SHOULD_BE_FAIL);
            }
            self
        }
    }

    impl Failure for Unit {
        fn command_fail(
            &mut self,
            callbacks: Vec<&dyn Fn() -> Result<ExitStatus, Error>>,
        ) -> &mut Self {
            for &c in &callbacks {
                let status: ExitStatus = c().unwrap();
                self.check(!status.success(), IS_FAIL, IS_NOT_FAIL);
            }
            self
        }

        fn fail(&mut self, callbacks: Vec<&dyn Fn() -> bool>) -> &mut Self {
            for &c in &callbacks {
                self.check(!c(), IS_FAIL, IS_NOT_FAIL);
            }
            self
        }
    }

    impl Take for Assert {
        fn assert_that(&mut self, t: bool) -> bool {
            self.assert(t)
        }

        fn take(&mut self, t: bool, s: &str, e: &str) -> &mut Self {
            let i: Instant = Instant::now();

            if self.assert_that(t) {
                assert_eq!(self.messages.insert(self.c.get(), s.to_string()), None);
                assert_eq!(self.take.insert(self.c.get(), i.elapsed().as_nanos()), None);
            } else {
                panic!("{}", format_args!("{s} match {e}"))
            }
            self
        }

        fn check(&mut self, t: bool, s: &str, e: &str) {
            let i: Instant = Instant::now();

            if self.assert_that(t) {
                assert!(self.messages.insert(self.c.get(), s.to_string()).is_some());
                assert!(self
                    .take
                    .insert(self.c.get(), i.elapsed().as_nanos())
                    .is_some());
            } else {
                panic!("{}", format_args!("{s} match {e}"))
            }
        }
    }

    impl Theory for Unit {
        fn chaos(&mut self, callback: &dyn Fn() -> bool) -> &mut Self {
            self.theory(false, &callback)
        }

        fn theory<T: PartialEq>(&mut self, expected: T, callback: &dyn Fn() -> T) -> &mut Self {
            self.take(callback() == expected, THEORY_IS_TRUE, THEORY_IS_FALSE)
        }

        fn theorem<T: PartialEq>(&mut self, expected: T, actual: &dyn Fn() -> T) -> &mut Self {
            self.take(expected.eq(&actual()), THEORY_IS_TRUE, THEORY_IS_FALSE)
        }
    }

    impl Theory for Assert {
        fn chaos(&mut self, callback: &dyn Fn() -> bool) -> &mut Self {
            self.take(
                !callback(),
                ASSERT_THEORY_IS_FALSE,
                ASSERT_THEORY_SHOULD_BE_FALSE,
            )
        }
        fn theorem<T: PartialEq>(&mut self, expected: T, actual: &dyn Fn() -> T) -> &mut Self {
            self.take(expected.eq(&actual()), THEORY_IS_TRUE, THEORY_IS_FALSE)
        }
        fn theory<T: PartialEq>(&mut self, expected: T, callback: &dyn Fn() -> T) -> &mut Self {
            self.take(
                expected == callback(),
                ASSERT_THEORY_IS_TRUE,
                ASSERT_THEORY_SHOULD_BE_TRUE,
            )
        }
    }
    impl Testable for Unit {
        fn matches(&mut self, pattern: &str, values: Vec<String>) -> &mut Self {
            let r = Regex::new(pattern).unwrap();

            for x in &values {
                self.check(r.is_match(x.as_str()), IS_MATCH, IS_NOT_MATCH);
            }
            self
        }

        fn capture(
            &mut self,
            pattern: &str,
            x: &str,
            key: usize,
            values: Vec<String>,
        ) -> &mut Self {
            let r: Regex = Regex::new(pattern).unwrap();
            let caps = r.captures(x).unwrap();
            for v in &values {
                self.check(
                    caps.get(key)
                        .expect("failed to get key")
                        .as_str()
                        .eq(v.as_str()),
                    IS_MATCH,
                    IS_NOT_MATCH,
                );
            }
            self
        }

        fn it(describe: &str, sleep_time: u64, callbacks: Vec<&dyn Fn(&mut Self) -> &mut Self>) {
            println!(
                "      {}",
                format_args!("{} {}", "[ OK ]".green().bold(), describe.blue().bold())
            );
            let mut x = Self::new(sleep_time);

            let mut j = &mut x;
            for &c in &callbacks {
                j = c(j);
            }

            let _ = j.end();
        }

        fn ok(&mut self, f: &dyn Fn() -> bool) -> &mut Self {
            self.take(f(), IS_OK, IS_KO)
        }

        fn ko(&mut self, f: &dyn Fn() -> bool) -> &mut Self {
            self.take(!f(), IS_KO, IS_OK)
        }

        fn assert(&mut self, test: bool) -> bool {
            if test {
                self.s.set(self.s.get() + 1);
            } else {
                self.f.set(self.f.get() + 1);
            }
            test
        }

        fn equals<T: PartialEq>(&mut self, a: T, b: T) -> &mut Self {
            self.take(a == b, IS_EQUALS, IS_UNEQUALS)
        }

        fn unequals<T: PartialEq>(&mut self, a: T, b: T) -> &mut Self {
            self.take(a != b, IS_UNEQUALS, IS_EQUALS)
        }

        fn superior<T: PartialOrd>(&mut self, a: T, min: T) -> &mut Self {
            self.take(a > min, IS_SUPERIOR, IS_INFERIOR)
        }

        fn inferior<T: PartialOrd>(&mut self, a: T, max: T) -> &mut Self {
            self.take(a < max, IS_INFERIOR, IS_SUPERIOR)
        }

        fn between<T: PartialOrd>(&mut self, a: T, min: T, max: T) -> &mut Self {
            self.take(a > min && a < max, IS_BETWEEN, IS_NOT_BETWEEN)
        }

        fn vec_contains<T: PartialEq>(&mut self, a: Vec<T>, b: T) -> &mut Self {
            self.take(a.contains(&b), IS_CONTAINS, IS_NOT_CONTAINS)
        }

        fn program(&mut self, p: &str) -> &mut Self {
            self.take(
                Path::new(p).is_executable(),
                IS_EXECUTABLE,
                IS_NOT_EXECUTABLE,
            )
        }

        fn not_program(&mut self, p: &str) -> &mut Self {
            self.take(
                !Path::new(p).is_executable(),
                IS_NOT_EXECUTABLE,
                IS_EXECUTABLE,
            )
        }

        fn vec_no_contains<T: PartialEq>(&mut self, a: Vec<T>, b: T) -> &mut Self {
            self.take(!a.contains(&b), IS_CONTAINS, IS_NOT_CONTAINS)
        }

        fn option_contains<T: PartialEq>(&mut self, a: Option<T>, b: T) -> &mut Self {
            self.take(a.expect("failed") == b, IS_CONTAINS, IS_NOT_CONTAINS)
        }

        fn hash_contains(&mut self, a: &mut HashSet<String>, b: String) -> &mut Self {
            self.take(a.contains(&b), IS_CONTAINS, IS_NOT_CONTAINS)
        }

        fn string_contains(&mut self, a: &str, b: &str) -> &mut Self {
            self.take(a.contains(b), IS_CONTAINS, IS_NOT_CONTAINS)
        }

        fn file_contains(&mut self, f: &str, v: &str) -> &mut Self {
            self.take(
                fs::read_to_string(f)
                    .unwrap_or_else(|_| panic!("The filename {f} has not been founded"))
                    .contains(v),
                IS_CONTAINS,
                IS_NOT_CONTAINS,
            )
        }

        fn exists(&mut self, p: &str) -> &mut Self {
            self.take(Path::new(p).exists(), IS_EXISTS, IS_NOT_EXISTS)
        }

        fn not_exists(&mut self, p: &str) -> &mut Self {
            self.take(!Path::new(p).exists(), IS_NOT_EXISTS, IS_EXISTS)
        }

        fn begin_with(&mut self, actual: &str, expected: &str) -> &mut Self {
            self.take(actual.starts_with(expected), IS_BEGIN, IS_NOT_BEGIN)
        }

        fn end_with(&mut self, actual: &str, expected: &str) -> &mut Self {
            self.take(actual.ends_with(expected), IS_FINNISH, IS_NOT_FINNISH)
        }

        fn end(&mut self) -> bool {
            let total: usize = self.f.get() + self.s.get();
            init_progress_bar_with_eta(total);
            set_progress_bar_action("[ :: ]", Color::Green, Style::Bold);

            let mut failure = self.failure.values();
            let mut success = self.success.values();
            let mut success_take = self.success_take.values();
            let mut failures_take = self.failure_take.values();

            for _i in 0..total {
                sleep(Duration::from_millis(self.sleep));

                if let Some(x) = success.next() {
                    print_progress_bar_info(
                        "[ OK ]",
                        format!(
                            "{} {} {} {}",
                            x.blue().bold(),
                            "take".white().bold(),
                            success_take.next().expect("").to_string().cyan().bold(),
                            "ns".blue().bold()
                        )
                        .as_str(),
                        Color::Green,
                        Style::Bold,
                    );
                }

                if let Some(x) = failure.next() {
                    print_progress_bar_info(
                        "[ KO ]",
                        format!(
                            "{} {} {} {}",
                            x.purple().bold(),
                            "take".white().bold(),
                            failures_take.next().expect("").to_string().cyan().bold(),
                            "ns".blue().bold()
                        )
                        .as_str(),
                        Color::Red,
                        Style::Bold,
                    );
                }
                inc_progress_bar();
            }

            print_progress_bar_final_info(
                "[ OK ]",
                format!(
                    "{} {} {} {}",
                    "Assertions :".blue().bold(),
                    self.s.get().to_string().green().bold(),
                    "Failures :".blue().bold(),
                    self.f.get().to_string().red().bold(),
                )
                .as_str(),
                Color::Green,
                Style::Bold,
            );
            finalize_progress_bar();
            true
        }

        fn new(sleep_time: u64) -> Self {
            Self {
                s: Cell::new(0),
                f: Cell::new(0),
                sleep: sleep_time,
                success_take: HashMap::new(),
                failure_take: HashMap::new(),
                success: HashMap::new(),
                failure: HashMap::new(),
            }
        }
    }

    impl Testable for Assert {
        fn matches(&mut self, pattern: &str, values: Vec<String>) -> &mut Self {
            let r = Regex::new(pattern).unwrap();

            for x in &values {
                self.check(r.is_match(x.as_str()), ASSERT_MATCH, ASSERT_SHOULD_MATCH);
            }
            self
        }

        fn capture(
            &mut self,
            pattern: &str,
            x: &str,
            key: usize,
            values: Vec<String>,
        ) -> &mut Self {
            let r = Regex::new(pattern).unwrap();
            let caps = r.captures(x).unwrap();
            for v in &values {
                self.check(
                    caps.get(key)
                        .expect("failed to get key")
                        .as_str()
                        .eq(v.as_str()),
                    ASSERT_MATCH,
                    ASSERT_SHOULD_MATCH,
                );
            }
            self
        }

        fn it(describe: &str, sleep_time: u64, callbacks: Vec<&dyn Fn(&mut Self) -> &mut Self>) {
            println!(
                "     {}",
                format_args!("{} {}", "[ + ]".green().bold(), describe.blue().bold())
            );
            let mut x = Self::new(sleep_time);

            let mut j = &mut x;
            for &c in &callbacks {
                j = c(j);
            }
            assert!(j.end());
        }

        fn ok(&mut self, f: &dyn Fn() -> bool) -> &mut Self {
            self.take(f(), ASSERT_OK, ASSERT_SHOULD_BE_OK)
        }

        fn ko(&mut self, f: &dyn Fn() -> bool) -> &mut Self {
            self.take(!f(), ASSERT_KO, ASSERT_SHOULD_BE_KO)
        }

        fn assert(&mut self, test: bool) -> bool {
            assert!(test);
            self.c.set(self.c.get() + 1);
            true
        }

        fn equals<T: PartialEq>(&mut self, a: T, b: T) -> &mut Self {
            self.take(a == b, ASSERT_EQUALS, ASSERT_SHOULD_BE_EQUALS)
        }

        fn unequals<T: PartialEq>(&mut self, a: T, b: T) -> &mut Self {
            self.take(a != b, ASSERT_UNEQUALS, ASSERT_SHOULD_BE_UNEQUALS)
        }
        fn superior<T: PartialOrd>(&mut self, a: T, min: T) -> &mut Self {
            self.take(a > min, ASSERT_SUPERIOR, ASSERT_SHOULD_BE_SUPERIOR)
        }

        fn inferior<T: PartialOrd>(&mut self, a: T, max: T) -> &mut Self {
            self.take(a < max, ASSERT_INFERIOR, ASSERT_SHOULD_BE_INFERIOR)
        }

        fn between<T: PartialOrd>(&mut self, a: T, min: T, max: T) -> &mut Self {
            self.take(a > min && a < max, ASSERT_BETWEEN, ASSERT_SHOULD_BE_BETWEEN)
        }

        fn vec_contains<T: PartialEq>(&mut self, a: Vec<T>, b: T) -> &mut Self {
            self.take(a.contains(&b), ASSERT_CONTAINS, ASSERT_SHOULD_CONTAINS)
        }

        fn program(&mut self, p: &str) -> &mut Self {
            self.take(
                Path::new(p).is_executable(),
                ASSERT_IS_EXECUTABLE,
                ASSERT_SHOULD_BE_EXECUTABLE,
            )
        }

        fn not_program(&mut self, p: &str) -> &mut Self {
            self.take(
                !Path::new(p).is_executable(),
                ASSERT_IS_NOT_EXECUTABLE,
                ASSERT_SHOULD_BE_NOT_EXECUTABLE,
            )
        }

        fn vec_no_contains<T: PartialEq>(&mut self, a: Vec<T>, b: T) -> &mut Self {
            self.take(
                !a.contains(&b),
                ASSERT_NOT_CONTAINS,
                ASSERT_SHOULD_BE_NOT_CONTAINS,
            )
        }

        fn option_contains<T: PartialEq>(&mut self, a: Option<T>, b: T) -> &mut Self {
            self.take(a.expect("") == b, ASSERT_CONTAINS, ASSERT_SHOULD_CONTAINS)
        }

        fn hash_contains(&mut self, a: &mut HashSet<String>, b: String) -> &mut Self {
            self.take(a.contains(&b), ASSERT_CONTAINS, ASSERT_SHOULD_CONTAINS)
        }

        fn string_contains(&mut self, a: &str, b: &str) -> &mut Self {
            self.take(a.contains(b), ASSERT_CONTAINS, ASSERT_SHOULD_CONTAINS)
        }

        fn file_contains(&mut self, f: &str, v: &str) -> &mut Self {
            self.take(
                fs::read_to_string(f)
                    .unwrap_or_else(|_| panic!("The filename {f} has not been founded"))
                    .contains(v),
                ASSERT_CONTAINS,
                ASSERT_SHOULD_CONTAINS,
            )
        }

        fn exists(&mut self, p: &str) -> &mut Self {
            self.take(
                Path::new(p).exists(),
                ASSERT_EXISTS,
                ASSERT_SHOULD_BE_EXISTS,
            )
        }

        fn not_exists(&mut self, p: &str) -> &mut Self {
            self.take(
                !Path::new(p).exists(),
                ASSERT_NOT_EXISTS,
                ASSERT_SHOULD_BE_NOT_EXISTS,
            )
        }

        fn begin_with(&mut self, actual: &str, expected: &str) -> &mut Self {
            self.take(
                actual.starts_with(expected),
                ASSERT_BEGIN,
                ASSERT_SHOULD_BE_BEGIN,
            )
        }

        fn end_with(&mut self, actual: &str, expected: &str) -> &mut Self {
            self.take(
                actual.ends_with(expected),
                ASSERT_FINNISH,
                ASSERT_SHOULD_BE_FINNISH,
            )
        }

        fn end(&mut self) -> bool {
            let total: usize = self.c.get();
            init_progress_bar_with_eta(total);
            set_progress_bar_action("[ ✓ ]", Color::Green, Style::Bold);

            let mut take = self.take.values();
            let mut messages = self.messages.values();
            for _i in 0..total {
                sleep(Duration::from_millis(self.sleep));
                print_progress_bar_info(
                    "[ ✓ ]",
                    format!(
                        "{} {} {} {}",
                        messages.next().unwrap().to_string().blue().bold(),
                        "take".white().bold(),
                        take.next().unwrap().to_string().cyan().bold(),
                        "ns".blue().bold()
                    )
                    .as_str(),
                    Color::Green,
                    Style::Bold,
                );
                inc_progress_bar();
            }

            print_progress_bar_final_info(
                "[ ✓ ]",
                format!(
                    "{} {}",
                    total.to_string().blue().bold(),
                    "assertions".blue().bold()
                )
                .as_str(),
                Color::Green,
                Style::Bold,
            );
            finalize_progress_bar();
            true
        }

        fn new(sleep_time: u64) -> Self {
            Self {
                c: Cell::new(0),
                sleep: sleep_time,
                messages: HashMap::new(),
                take: HashMap::new(),
            }
        }
    }
}

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
    use std::collections::HashSet;

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
        u.exists(".").exists("README.md")
    }

    fn must_linux(u: &mut Assert) -> &mut Assert {
        u
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
            .string_contains("linux", "linux")
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
        u
    }

    fn no_programs(u: &mut Assert) -> &mut Assert {
        u
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
        u.exists(".").exists("README.md")
    }

    fn check_linux(u: &mut Unit) -> &mut Unit {
        u
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
            .string_contains("linux", "linux")
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

    fn check_no_programs(u: &mut Unit) -> &mut Unit {
        u
    }
    fn check_programs(u: &mut Unit) -> &mut Unit {
        u
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
