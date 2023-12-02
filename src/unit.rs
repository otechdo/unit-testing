pub mod unit {
    use crate::unit::consts::unit::{ASSERT_PROGRESS_TIME, UNIT_PROGRESS_TIME};

    use self::consts::unit::{
        ASSERT_BETWEEN, ASSERT_CONTAINS, ASSERT_EQUALS, ASSERT_EXISTS, ASSERT_INFERIOR,
        ASSERT_IS_EXECUTABLE, ASSERT_IS_NOT_EXECUTABLE, ASSERT_NOT_CONTAINS, ASSERT_OK,
        ASSERT_SHOULD_BE_BETWEEN, ASSERT_SHOULD_BE_EQUALS, ASSERT_SHOULD_BE_EXECUTABLE,
        ASSERT_SHOULD_BE_INFERIOR, ASSERT_SHOULD_BE_KO, ASSERT_SHOULD_BE_NOT_CONTAINS,
        ASSERT_SHOULD_BE_NOT_EXECUTABLE, ASSERT_SHOULD_BE_OK, ASSERT_SHOULD_BE_SUPERIOR,
        ASSERT_SHOULD_BE_UNEQUALS, ASSERT_SHOULD_CONTAINS, ASSERT_SOULD_BE_EXISTS, ASSERT_SUPERIOR,
        ASSERT_THEORY_IS_FALSE, ASSERT_THEORY_IS_TRUE, ASSERT_THEORY_SHOULD_BE_FALSE,
        ASSERT_THEORY_SHOULD_BE_TRUE, ASSERT_UNEQUALS, IS_BETWEEN, IS_CONTAINS, IS_EQUALS,
        IS_EXECUTABLE, IS_EXISTS, IS_INFERIOR, IS_KO, IS_NOT_BETWEEN, IS_NOT_CONTAINS,
        IS_NOT_EXECUTABLE, IS_NOT_EXISTS, IS_OK, IS_SUPERIOR, IS_UNEQUALS, THEORY_IS_FALSE,
        THEORY_IS_TRUE,
    };

    use self::traits::unit::{Take, Testable, Theory};
    use colored_truecolor::Colorize;
    use crossterm_cursor::TerminalCursor;
    use is_executable::IsExecutable;
    use progress_bar::*;
    use std::cell::Cell;
    use std::collections::{HashMap, HashSet};
    use std::fs;
    use std::path::Path;
    use std::process::{exit, ExitCode};
    use std::thread::sleep;
    use std::time::{Duration, Instant};

    pub mod consts;
    pub mod enums;
    pub mod traits;

    pub struct Assert {
        c: Cell<usize>,
        messages: HashMap<usize, String>,
        take: HashMap<usize, u128>,
    }

    pub struct Unit {
        s: Cell<usize>,
        f: Cell<usize>,
        success_take: HashMap<usize, u128>,
        failure_take: HashMap<usize, u128>,
        success: HashMap<usize, String>,
        failure: HashMap<usize, String>,
    }

    impl Take for Unit {
        fn assert_that(&mut self, t: bool) -> bool {
            self.assert(t)
        }

        fn take<'a>(&'a mut self, t: bool, s: &'a str, e: &'a str) -> &mut Self {
            let i = Instant::now();
            match self.assert_that(t) {
                true => {
                    self.success.insert(self.s.get(), s.to_string());
                    self.success_take
                        .insert(self.s.get(), i.elapsed().as_nanos());
                }
                false => {
                    self.failure.insert(self.f.get(), e.to_string());
                    self.failure_take
                        .insert(self.f.get(), i.elapsed().as_nanos());
                }
            };
            self
        }
    }

    impl Take for Assert {
        fn assert_that(&mut self, t: bool) -> bool {
            self.assert(t)
        }

        fn take<'a>(&'a mut self, t: bool, s: &'a str, _e: &'a str) -> &mut Self {
            let i = Instant::now();
            match self.assert_that(t) {
                true => {
                    self.messages.insert(self.c.get(), s.to_string());
                    self.take.insert(self.c.get(), i.elapsed().as_nanos());
                }
                false => panic!("not possible"),
            };
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
    }

    impl Testable for Unit {
        fn it(callbacks: Vec<&dyn Fn(&mut Self) -> &mut Self>) -> ExitCode {
            let mut x = Self {
                success: HashMap::new(),
                failure: HashMap::new(),
                s: Cell::new(0),
                f: Cell::new(0),
                success_take: HashMap::new(),
                failure_take: HashMap::new(),
            };

            let mut j = &mut x;
            for &c in callbacks.iter() {
                j = c(j);
            }

            j.end().expect("a");
            if x.failure.capacity() > 0 {
                exit(1)
            }
            exit(0)
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
                    .unwrap_or_else(|_| panic!("The filename {} has not been founded", f))
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

        fn end(&mut self) -> Result<&mut Self, String> {
            let total: usize = self.f.get() + self.s.get();
            println!();
            init_progress_bar_with_eta(total);
            set_progress_bar_action("[ :: ]", Color::Green, Style::Bold);

            let mut failure = self.failure.values();
            let mut success = self.success.values();
            let mut success_take = self.success_take.values();
            let mut failures_take = self.failure_take.values();

            for _i in 0..total {
                sleep(Duration::from_millis(UNIT_PROGRESS_TIME));

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
            Ok(self)
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

        fn theory<T: PartialEq>(&mut self, expected: T, callback: &dyn Fn() -> T) -> &mut Self {
            self.take(
                expected == callback(),
                ASSERT_THEORY_IS_TRUE,
                ASSERT_THEORY_SHOULD_BE_TRUE,
            )
        }
    }

    impl Testable for Assert {
        fn it(callbacks: Vec<&dyn Fn(&mut Self) -> &mut Self>) -> ExitCode {
            let mut x = Self {
                messages: HashMap::new(),
                c: Cell::new(0),
                take: HashMap::new(),
            };
            let cursor = TerminalCursor::new();
            cursor.hide().expect("failed to hide cursor");
            let mut j = &mut x;
            for c in callbacks.iter() {
                j = c(j);
            }

            j.end().expect("a");
            cursor.show().expect("failed to re show cursor");
            exit(0)
        }

        fn ok(&mut self, f: &dyn Fn() -> bool) -> &mut Self {
            self.take(f(), ASSERT_OK, ASSERT_SHOULD_BE_OK)
        }

        fn ko(&mut self, f: &dyn Fn() -> bool) -> &mut Self {
            self.take(!f(), ASSERT_OK, ASSERT_SHOULD_BE_KO)
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
                    .unwrap_or_else(|_| panic!("The filename {} has not been founded", f))
                    .contains(v),
                ASSERT_CONTAINS,
                ASSERT_SHOULD_CONTAINS,
            )
        }

        fn exists(&mut self, p: &str) -> &mut Self {
            self.take(Path::new(p).exists(), ASSERT_EXISTS, ASSERT_SOULD_BE_EXISTS)
        }

        fn not_exists(&mut self, p: &str) -> &mut Self {
            self.take(
                !Path::new(p).exists(),
                ASSERT_EXISTS,
                ASSERT_SOULD_BE_EXISTS,
            )
        }

        fn end(&mut self) -> Result<&mut Self, String> {
            let total: usize = self.c.get();
            println!();
            init_progress_bar_with_eta(total);
            set_progress_bar_action("[ ✓ ]", Color::Green, Style::Bold);

            let mut take = self.take.values();
            let mut messages = self.messages.values();
            for _i in 0..total {
                sleep(Duration::from_millis(ASSERT_PROGRESS_TIME));
                print_progress_bar_info(
                    "[ ✓ ]",
                    format!(
                        "{} {} {} {}",
                        messages.next().expect("").blue().bold(),
                        "take".white().bold(),
                        take.next().expect("").to_string().cyan().bold(),
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
            Ok(self)
        }
    }
}

#[cfg(test)]
mod test {
    use std::{collections::HashSet, env::consts::OS, process::ExitCode};

    use num::Float;
    use unit::{
        traits::unit::{Testable, Theory},
        Assert,
    };

    use crate::unit;

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
            .not_exists("C:\\WINDOWS\\system32")
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
        let o = Some("a".to_string());
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

    fn must_between(u: &mut Assert) -> &mut Assert {
        u.between(10, 5, 50).between(50, 10, 200)
    }

    fn pythagore() -> f32 {
        Float::hypot(3.0, 4.0)
    }

    fn pythagore_not_work() -> bool {
        Float::hypot(4.0, 4.0) == 5.0
    }

    fn must_theory(u: &mut Assert) -> &mut Assert {
        u.theory(5.0, &pythagore).chaos(&pythagore_not_work)
    }

    #[test]
    pub fn all() -> ExitCode {
        Assert::it(vec![
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
        ])
    }
}
