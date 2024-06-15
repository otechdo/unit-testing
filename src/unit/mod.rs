use crate::objects::{Failure, Success, Take, Testable, Theory};
use crate::output::{
    IS_BEGIN, IS_BETWEEN, IS_CONTAINS, IS_EQUALS, IS_EXECUTABLE, IS_EXISTS, IS_FAIL, IS_FINNISH,
    IS_INFERIOR, IS_KO, IS_MATCH, IS_NOT_BEGIN, IS_NOT_BETWEEN, IS_NOT_CONTAINS, IS_NOT_EXECUTABLE,
    IS_NOT_EXISTS, IS_NOT_FAIL, IS_NOT_FINNISH, IS_NOT_MATCH, IS_NOT_SUCCESS, IS_OK, IS_SUCCESS,
    IS_SUPERIOR, IS_UNEQUALS, THEORY_IS_FALSE, THEORY_IS_TRUE,
};
use colored_truecolor::Colorize;
use is_executable::IsExecutable;
use progress_bar::{
    finalize_progress_bar, inc_progress_bar, init_progress_bar_with_eta,
    print_progress_bar_final_info, print_progress_bar_info, set_progress_bar_action, Color, Style,
};
use regex::Regex;
use std::cell::Cell;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::process::ExitStatus;
use std::thread::sleep;
use std::time::{Duration, Instant};
use std::{fs, io};

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

impl Success for Unit {
    fn run(&mut self, callbacks: Vec<&dyn Fn() -> Result<ExitStatus, io::Error>>) -> &mut Self {
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

impl Failure for Unit {
    fn command_fail(
        &mut self,
        callbacks: Vec<&dyn Fn() -> Result<ExitStatus, io::Error>>,
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

impl Testable for Unit {
    fn matches(&mut self, pattern: &str, values: Vec<String>) -> &mut Self {
        let r = Regex::new(pattern).unwrap();

        for x in &values {
            self.check(r.is_match(x.as_str()), IS_MATCH, IS_NOT_MATCH);
        }
        self
    }

    fn capture(&mut self, pattern: &str, x: &str, key: usize, values: Vec<String>) -> &mut Self {
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

    fn it(
        title: &str,
        description: &str,
        sleep_time: u64,
        callbacks: Vec<&dyn Fn(&mut Self) -> &mut Self>,
    ) {
        println!("\n{}\n", description.white().bold());
        println!(
            "      {}",
            format_args!("{} {}", "[ OK ]".green().bold(), title.blue().bold())
        );
        let mut x = Self::new(sleep_time);

        let mut j = &mut x;
        for &c in &callbacks {
            j = c(j);
        }

        let _ = j.end();
    }

    fn ok(&mut self, f: bool) -> &mut Self {
        self.take(f, IS_OK, IS_KO)
    }

    fn ko(&mut self, f: bool) -> &mut Self {
        self.take(!f, IS_KO, IS_OK)
    }

    fn assert(&mut self, test: bool) -> bool {
        if test {
            self.s.set(self.s.get() + 1);
        } else {
            self.f.set(self.f.get() + 1);
        }
        test
    }

    fn eq<T: PartialEq>(&mut self, a: T, b: T) -> &mut Self {
        self.take(a.eq(&b), IS_EQUALS, IS_UNEQUALS)
    }

    fn ne<T: PartialEq>(&mut self, a: T, b: T) -> &mut Self {
        self.take(a.ne(&b), IS_UNEQUALS, IS_EQUALS)
    }

    fn gt<T: PartialOrd>(&mut self, a: T, min: T) -> &mut Self {
        self.take(a.gt(&min), IS_SUPERIOR, IS_INFERIOR)
    }

    fn ge<T: PartialOrd>(&mut self, a: T, max: T) -> &mut Self {
        self.take(a.ge(&max), IS_INFERIOR, IS_SUPERIOR)
    }
    fn lt<T: PartialOrd>(&mut self, a: T, max: T) -> &mut Self {
        self.take(a.lt(&max), IS_SUPERIOR, IS_INFERIOR)
    }

    fn le<T: PartialOrd>(&mut self, a: T, max: T) -> &mut Self {
        self.take(a.le(&max), IS_INFERIOR, IS_SUPERIOR)
    }

    fn between<T: PartialOrd>(&mut self, a: T, min: T, max: T) -> &mut Self {
        self.take(a > min && a < max, IS_BETWEEN, IS_NOT_BETWEEN)
    }

    fn vec_contains<T: PartialEq>(&mut self, a: Vec<T>, b: T) -> &mut Self {
        self.take(a.contains(&b), IS_CONTAINS, IS_NOT_CONTAINS)
    }

    fn exe(&mut self, p: &str) -> &mut Self {
        self.take(
            Path::new(p).is_executable(),
            IS_EXECUTABLE,
            IS_NOT_EXECUTABLE,
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

    fn str_contains(&mut self, a: &str, b: &str) -> &mut Self {
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

    fn start_with(&mut self, actual: &str, expected: &str) -> &mut Self {
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

#[cfg(test)]
mod test {
    use crate::check_that;
    use crate::objects::{Testable, Theory};
    use crate::output::DISABLE_PROGRESS_TIME;
    use crate::unit::Unit;
    use std::collections::HashSet;

    fn ok() -> bool {
        true
    }

    fn ko() -> bool {
        false
    }

    fn must_pass(u: &mut Unit) -> &mut Unit {
        u.ok(ok()).ko(ko())
    }

    fn must_exists(u: &mut Unit) -> &mut Unit {
        u.exists(".").exists("README.md")
    }

    fn must_linux(u: &mut Unit) -> &mut Unit {
        u
    }

    fn must_equals(u: &mut Unit) -> &mut Unit {
        u.eq("README.md", "README.md")
            .eq(4, 4)
            .eq(4.4, 4.4)
            .eq(true, true)
            .eq(false, false)
    }

    fn must_contains(u: &mut Unit) -> &mut Unit {
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

    fn must_unequals(u: &mut Unit) -> &mut Unit {
        u.ne("README.md", ".")
            .ne(4, 6)
            .ne(5.6, 4.4)
            .ne(false, true)
            .ne(false, true)
    }

    fn must_superior(u: &mut Unit) -> &mut Unit {
        u.gt(1, 0).gt(5, 2)
    }

    fn programs(u: &mut Unit) -> &mut Unit {
        u
    }

    fn no_programs(u: &mut Unit) -> &mut Unit {
        u
    }

    fn must_inferior(u: &mut Unit) -> &mut Unit {
        u.lt(10, 50).lt(50, 200)
    }

    fn must_between(u: &mut Unit) -> &mut Unit {
        u.between(10, 5, 50).between(50, 10, 200)
    }

    fn pythagore() -> f32 {
        3.0_f32.hypot(4.0)
    }

    fn pythagore_not_work() -> bool {
        4.0_f32.hypot(4.0) == 5.0
    }

    fn must_theory(u: &mut Unit) -> &mut Unit {
        u.theory(5.0, &pythagore).chaos(&pythagore_not_work)
    }

    #[test]
    pub fn all() {
        check_that!(
            "Test the assert framework",
            "Check if all values passes on success, test can be have failures.",
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
