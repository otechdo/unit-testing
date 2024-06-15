use std::cell::Cell;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::process::ExitStatus;
use std::thread::sleep;
use std::time::{Duration, Instant};
use std::{fs, io};

use colored_truecolor::Colorize;
use is_executable::IsExecutable;
use progress_bar::{
    finalize_progress_bar, inc_progress_bar, init_progress_bar_with_eta,
    print_progress_bar_final_info, print_progress_bar_info, set_progress_bar_action, Color, Style,
};
use regex::Regex;

use crate::objects::{Failure, Success, Take, Testable, Theory};
use crate::output::{
    ASSERT_BEGIN, ASSERT_BETWEEN, ASSERT_CONTAINS, ASSERT_EQUALS, ASSERT_EXISTS, ASSERT_FAIL,
    ASSERT_FINNISH, ASSERT_IS_EXECUTABLE, ASSERT_KO, ASSERT_MATCH, ASSERT_NOT_CONTAINS,
    ASSERT_NOT_EXISTS, ASSERT_OK, ASSERT_SHOULD_BE_BEGIN, ASSERT_SHOULD_BE_BETWEEN,
    ASSERT_SHOULD_BE_EQUALS, ASSERT_SHOULD_BE_EXECUTABLE, ASSERT_SHOULD_BE_EXISTS,
    ASSERT_SHOULD_BE_FAIL, ASSERT_SHOULD_BE_FINNISH, ASSERT_SHOULD_BE_KO,
    ASSERT_SHOULD_BE_NOT_CONTAINS, ASSERT_SHOULD_BE_NOT_EXISTS, ASSERT_SHOULD_BE_OK,
    ASSERT_SHOULD_BE_SUCCESS, ASSERT_SHOULD_BE_SUPERIOR, ASSERT_SHOULD_BE_UNEQUALS,
    ASSERT_SHOULD_CONTAINS, ASSERT_SHOULD_MATCH, ASSERT_SUCCESS, ASSERT_SUPERIOR,
    ASSERT_THEORY_IS_FALSE, ASSERT_THEORY_IS_TRUE, ASSERT_THEORY_SHOULD_BE_FALSE,
    ASSERT_THEORY_SHOULD_BE_TRUE, ASSERT_UNEQUALS, IS_FAIL, IS_SUCCESS, THEORY_IS_FALSE,
    THEORY_IS_TRUE,
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

impl Success for Assert {
    fn run(&mut self, callbacks: Vec<&dyn Fn() -> Result<ExitStatus, io::Error>>) -> &mut Self {
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
            self.check(c(), IS_SUCCESS, IS_FAIL);
        }
        self
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

impl Failure for Assert {
    fn command_fail(
        &mut self,
        callbacks: Vec<&dyn Fn() -> Result<ExitStatus, io::Error>>,
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

impl Testable for Assert {
    fn matches(&mut self, pattern: &str, values: Vec<String>) -> &mut Self {
        let r = Regex::new(pattern).unwrap();

        for x in &values {
            self.check(r.is_match(x.as_str()), ASSERT_MATCH, ASSERT_SHOULD_MATCH);
        }
        self
    }

    fn capture(&mut self, pattern: &str, x: &str, key: usize, values: Vec<String>) -> &mut Self {
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

    fn it(
        title: &str,
        description: &str,
        sleep_time: u64,
        callbacks: Vec<&dyn Fn(&mut Self) -> &mut Self>,
    ) {
        println!("\n{}\n", description.white().bold());
        println!(
            "     {}",
            format_args!("{} {}", "[ + ]".green().bold(), title.blue().bold())
        );
        let mut x = Self::new(sleep_time);

        let mut j = &mut x;
        for &c in &callbacks {
            j = c(j);
        }
        assert!(j.end());
    }

    fn ok(&mut self, f: bool) -> &mut Self {
        self.take(f, ASSERT_OK, ASSERT_SHOULD_BE_OK)
    }

    fn ko(&mut self, f: bool) -> &mut Self {
        self.take(!f, ASSERT_KO, ASSERT_SHOULD_BE_KO)
    }

    fn assert(&mut self, test: bool) -> bool {
        assert!(test);
        self.c.set(self.c.get() + 1);
        true
    }

    fn eq<T: PartialEq>(&mut self, a: T, b: T) -> &mut Self {
        self.take(a.eq(&b), ASSERT_EQUALS, ASSERT_SHOULD_BE_EQUALS)
    }

    fn ne<T: PartialEq>(&mut self, a: T, b: T) -> &mut Self {
        self.take(a.ne(&b), ASSERT_UNEQUALS, ASSERT_SHOULD_BE_UNEQUALS)
    }
    fn gt<T: PartialOrd>(&mut self, a: T, min: T) -> &mut Self {
        self.take(a.gt(&min), ASSERT_SUPERIOR, ASSERT_SHOULD_BE_SUPERIOR)
    }
    fn ge<T: PartialOrd>(&mut self, a: T, min: T) -> &mut Self {
        self.take(a.ge(&min), ASSERT_SUPERIOR, ASSERT_SHOULD_BE_SUPERIOR)
    }
    fn le<T: PartialOrd>(&mut self, a: T, min: T) -> &mut Self {
        self.take(a.le(&min), ASSERT_SUPERIOR, ASSERT_SHOULD_BE_SUPERIOR)
    }
    fn lt<T: PartialOrd>(&mut self, a: T, min: T) -> &mut Self {
        self.take(a.lt(&min), ASSERT_SUPERIOR, ASSERT_SHOULD_BE_SUPERIOR)
    }

    fn between<T: PartialOrd>(&mut self, a: T, min: T, max: T) -> &mut Self {
        self.take(a > min && a < max, ASSERT_BETWEEN, ASSERT_SHOULD_BE_BETWEEN)
    }

    fn vec_contains<T: PartialEq>(&mut self, a: Vec<T>, b: T) -> &mut Self {
        self.take(a.contains(&b), ASSERT_CONTAINS, ASSERT_SHOULD_CONTAINS)
    }

    fn exe(&mut self, p: &str) -> &mut Self {
        self.take(
            Path::new(p).is_executable(),
            ASSERT_IS_EXECUTABLE,
            ASSERT_SHOULD_BE_EXECUTABLE,
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

    fn str_contains(&mut self, a: &str, b: &str) -> &mut Self {
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

    fn start_with(&mut self, actual: &str, expected: &str) -> &mut Self {
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

#[cfg(test)]
mod test {
    use crate::assert_that;
    use crate::assertions::Assert;
    use crate::objects::{Testable, Theory};
    use crate::output::DISABLE_PROGRESS_TIME;
    use std::collections::HashSet;

    fn ok() -> bool {
        true
    }

    fn ko() -> bool {
        false
    }

    fn must_pass(u: &mut Assert) -> &mut Assert {
        u.ok(ok()).ko(ko())
    }

    fn must_exists(u: &mut Assert) -> &mut Assert {
        u.exists(".").exists("README.md")
    }

    fn must_linux(u: &mut Assert) -> &mut Assert {
        u
    }

    fn must_equals(u: &mut Assert) -> &mut Assert {
        u.eq("README.md", "README.md")
            .eq(4, 4)
            .eq(4.4, 4.4)
            .eq(true, true)
            .eq(false, false)
    }

    fn must_contains(u: &mut Assert) -> &mut Assert {
        let mut v: Vec<String> = Vec::new();
        let o = Some("a".to_string());
        v.push("value".to_string());
        v.push("h".to_string());
        u.vec_contains(v, "h".to_string())
            .option_contains(o, "a".to_string())
            .str_contains("linux", "linux")
            .file_contains("README.md", "Installation")
            .hash_contains(&mut HashSet::from(["a".to_string()]), "a".to_string())
    }

    fn must_unequals(u: &mut Assert) -> &mut Assert {
        u.ne("README.md", ".")
            .ne(4, 6)
            .ne(5.6, 4.4)
            .ne(false, true)
            .ne(false, true)
    }

    fn must_superior(u: &mut Assert) -> &mut Assert {
        u.gt(1, 0).gt(5, 2)
    }

    fn programs(u: &mut Assert) -> &mut Assert {
        u
    }

    fn no_programs(u: &mut Assert) -> &mut Assert {
        u
    }

    fn must_inferior(u: &mut Assert) -> &mut Assert {
        u.lt(10, 50).lt(50, 200)
    }

    fn must_between(u: &mut Assert) -> &mut Assert {
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

    #[test]
    pub fn all() {
        assert_that!(
            "Test the assert framework",
            "Check if all values passes on success, can't be have failures.",
            DISABLE_PROGRESS_TIME,
            vec![
                &must_between,
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
}
