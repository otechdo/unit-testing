pub mod unit {
    use crate::unit::consts::unit::{
        THEORY_IS_TRUE, THEORY_SHOULD_BE_FALSE, THEORY_SHOULD_BE_TRUE,
    };
    use crossterm_cursor::TerminalCursor;
    use progress_bar::*;
    use std::collections::HashSet;
    use std::path::Path;
    use std::process::{exit, ExitCode};
    use std::thread::sleep;
    use std::time::Duration;
    use std::{cell::Cell, fs};

    use self::consts::unit::{
        IS_EXECUTABLE, IS_EXISTS, IS_NOT_BETWEEN, IS_NOT_EXECUTABLE, IS_NOT_EXISTS,
        SHOULD_BE_EXECUTABLE, SHOULD_NOT_CONTAINS, SHOULD_NOT_EXISTS, SOULD_BE_NOT_EXECUTABLE,
        THEORY_IS_FALSE,
    };
    use self::{
        consts::unit::{
            IS_BETWEEN, IS_CONTAINS, IS_EQUALS, IS_INFERIOR, IS_KO, IS_NOT_CONTAINS, IS_OK,
            IS_SUPERIOR, IS_UNEQUALS, SHOULD_BE_BETWEEN, SHOULD_BE_EQUALS, SHOULD_BE_INFERIOR,
            SHOULD_BE_KO, SHOULD_BE_OK, SHOULD_BE_SUPERIOR, SHOULD_BE_UNEQUALS, SHOULD_CONTAINS,
        },
        traits::unit::{Testable, Theory},
    };
    use colored_truecolor::Colorize;
    use is_executable::IsExecutable;

    pub mod consts;
    pub mod enums;
    pub mod traits;

    #[derive()]
    pub struct Assert {
        assertions: Cell<usize>,
        messages: Vec<String>,
    }

    pub struct Unit {
        assertions: Cell<usize>,
        failures: Cell<usize>,
        success: Vec<String>,
        failure: Vec<String>,
    }

    impl Theory for Unit {
        fn chaos(&mut self, callback: &dyn Fn() -> bool) -> &mut Self {
            self.theory(false, &callback)
        }

        fn theory<T: PartialEq>(&mut self, expected: T, callback: &dyn Fn() -> T) -> &mut Self {
            self.assert(callback() == expected, THEORY_IS_TRUE, THEORY_IS_FALSE)
        }
    }

    impl Testable for Unit {
        fn it(callbacks: Vec<&dyn Fn(&mut Self) -> &mut Self>) -> ExitCode {
            let mut x = Self {
                assertions: Cell::new(0),
                failures: Cell::new(0),
                success: Vec::new(),
                failure: Vec::new(),
            };

            let mut j = &mut x;
            for &c in callbacks.iter() {
                j = c(j);
            }

            j.end().expect("a");
            exit(0)
        }

        fn ok(&mut self, f: &dyn Fn() -> bool) -> &mut Self {
            self.assert(f(), IS_OK, IS_KO)
        }

        fn ko(&mut self, f: &dyn Fn() -> bool) -> &mut Self {
            self.assert(!f(), IS_KO, IS_OK)
        }

        fn assert(&mut self, test: bool, s: &str, e: &str) -> &mut Self {
            if test {
                self.assertions.set(self.assertions.get() + 1);
                self.success.push(s.to_string());
            } else {
                self.failures.set(self.failures.get() + 1);
                self.failure.push(e.to_string());
            }
            self
        }

        fn equals<T: PartialEq>(&mut self, a: T, b: T) -> &mut Self {
            self.assert(a == b, IS_EQUALS, IS_UNEQUALS)
        }

        fn unequals<T: PartialEq>(&mut self, a: T, b: T) -> &mut Self {
            self.assert(a != b, IS_UNEQUALS, IS_EQUALS)
        }

        fn superior<T: PartialOrd>(&mut self, a: T, min: T) -> &mut Self {
            self.assert(a > min, IS_SUPERIOR, IS_INFERIOR)
        }

        fn inferior<T: PartialOrd>(&mut self, a: T, max: T) -> &mut Self {
            self.assert(a < max, IS_INFERIOR, IS_SUPERIOR)
        }

        fn between<T: PartialOrd>(&mut self, a: T, min: T, max: T) -> &mut Self {
            self.assert(a > min && a < max, IS_BETWEEN, IS_NOT_BETWEEN)
        }

        fn vec_contains<T: PartialEq>(&mut self, a: Vec<T>, b: T) -> &mut Self {
            self.assert(a.contains(&b), IS_CONTAINS, IS_NOT_CONTAINS)
        }

        fn is_program(&mut self, p: &str) -> &mut Self {
            self.assert(
                Path::new(p).is_executable(),
                IS_EXECUTABLE,
                IS_NOT_EXECUTABLE,
            )
        }

        fn not_program(&mut self, p: &str) -> &mut Self {
            self.assert(
                !Path::new(p).is_executable(),
                IS_NOT_EXECUTABLE,
                IS_EXECUTABLE,
            )
        }

        fn vec_no_contains<T: PartialEq>(&mut self, a: Vec<T>, b: T) -> &mut Self {
            self.assert(!a.contains(&b), IS_NOT_CONTAINS, IS_CONTAINS)
        }

        fn option_contains<T: PartialEq>(&mut self, a: Option<T>, b: T) -> &mut Self {
            self.assert(a.expect("failed") == b, IS_CONTAINS, IS_NOT_CONTAINS)
        }

        fn hash_contains(&mut self, a: &mut HashSet<String>, b: String) -> &mut Self {
            self.assert(a.contains(&b), IS_CONTAINS, IS_NOT_CONTAINS)
        }

        fn string_contains(&mut self, a: &str, b: &str) -> &mut Self {
            self.assert(a.contains(b), IS_CONTAINS, IS_NOT_CONTAINS)
        }

        fn file_contains(&mut self, f: &str, v: &str) -> &mut Self {
            self.assert(
                fs::read_to_string(f)
                    .unwrap_or_else(|_| panic!("The filename {} has not been founded", f))
                    .contains(v),
                IS_CONTAINS,
                SHOULD_CONTAINS,
            )
        }

        fn exists(&mut self, p: &str) -> &mut Self {
            self.assert(Path::new(p).exists(), IS_EXISTS, IS_NOT_EXISTS)
        }

        fn not_exists(&mut self, p: &str) -> &mut Self {
            self.assert(!Path::new(p).exists(), IS_NOT_EXISTS, IS_EXISTS)
        }

        fn end(&mut self) -> Result<&mut Self, String> {
            let total: usize = self.assertions.get() + self.failures.get();
            println!();
            let success_i = Cell::new(0);
            let failure_i = Cell::new(0);
            init_progress_bar_with_eta(total);
            set_progress_bar_action("[ :: ]", Color::Green, Style::Bold);
            for _i in 0..total {
                let s = self.success.get(success_i.get());
                let f = self.failure.get(failure_i.get());
                sleep(Duration::from_millis(100));

                if let Some(x) = s {
                    print_progress_bar_info(
                        "[ OK ]",
                        x.blue().bold().to_string().as_str(),
                        Color::Green,
                        Style::Bold,
                    );
                    success_i.set(success_i.get() + 1);
                }

                if let Some(x) = f {
                    print_progress_bar_info(
                        "[ KO ]",
                        x.purple().bold().to_string().as_str(),
                        Color::Red,
                        Style::Bold,
                    );
                    failure_i.set(failure_i.get() + 1);
                }
                inc_progress_bar();
            }

            print_progress_bar_final_info(
                "[ OK ]",
                format!(
                    "{} {} {} {}",
                    "Assertions :".blue().bold(),
                    self.assertions.get().to_string().green().bold(),
                    "Failures :".blue().bold(),
                    self.failures.get().to_string().red().bold(),
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
            self.assert(!callback(), THEORY_IS_TRUE, THEORY_SHOULD_BE_FALSE)
        }

        fn theory<T: PartialEq>(&mut self, expected: T, callback: &dyn Fn() -> T) -> &mut Self {
            self.assert(
                expected == callback(),
                THEORY_IS_TRUE,
                THEORY_SHOULD_BE_TRUE,
            )
        }
    }

    impl Testable for Assert {
        fn ok(&mut self, f: &dyn Fn() -> bool) -> &mut Self {
            self.assert(f(), IS_OK, SHOULD_BE_OK)
        }

        fn ko(&mut self, f: &dyn Fn() -> bool) -> &mut Self {
            self.assert(!f(), IS_KO, SHOULD_BE_KO)
        }

        fn assert(&mut self, test: bool, s: &str, e: &str) -> &mut Self {
            assert!(test, "{}", format!("[ {} ] {}", "KO".red().bold(), e));
            self.assertions.set(self.assertions.get() + 1);
            self.messages.push(s.to_string());
            self
        }

        fn it(callbacks: Vec<&dyn Fn(&mut Self) -> &mut Self>) -> ExitCode {
            let mut x = Self {
                assertions: Cell::new(0),
                messages: Vec::new(),
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

        fn end(&mut self) -> Result<&mut Self, String> {
            let total: usize = self.assertions.get();
            println!();

            init_progress_bar_with_eta(total);
            set_progress_bar_action("[ ✓ ]", Color::Green, Style::Bold);

            for x in self.messages.iter() {
                sleep(Duration::from_millis(100));

                print_progress_bar_info(
                    "[ ✓ ]",
                    x.blue().bold().to_string().as_str(),
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

        fn exists(&mut self, p: &str) -> &mut Self {
            self.assert(
                Path::new(p).exists(),
                format!("The path {} exists", p).as_str(),
                format!("The path {} should be exist", p).as_str(),
            )
        }
        fn equals<T: PartialEq>(&mut self, a: T, b: T) -> &mut Self {
            self.assert(a == b, IS_EQUALS, SHOULD_BE_EQUALS)
        }

        fn unequals<T: PartialEq>(&mut self, a: T, b: T) -> &mut Self {
            self.assert(a != b, IS_UNEQUALS, SHOULD_BE_UNEQUALS)
        }

        fn superior<T: PartialOrd>(&mut self, a: T, min: T) -> &mut Self {
            self.assert(a > min, IS_SUPERIOR, SHOULD_BE_SUPERIOR)
        }

        fn inferior<T: PartialOrd>(&mut self, a: T, max: T) -> &mut Self {
            self.assert(a < max, IS_INFERIOR, SHOULD_BE_INFERIOR)
        }

        fn between<T: PartialOrd>(&mut self, a: T, min: T, max: T) -> &mut Self {
            self.assert(a > min && a < max, IS_BETWEEN, SHOULD_BE_BETWEEN)
        }

        fn vec_contains<T: PartialEq>(&mut self, a: Vec<T>, b: T) -> &mut Self {
            self.assert(a.contains(&b), IS_CONTAINS, SHOULD_CONTAINS)
        }

        fn option_contains<T: PartialEq>(&mut self, a: Option<T>, b: T) -> &mut Self {
            self.assert(a.expect("") == b, IS_CONTAINS, SHOULD_CONTAINS)
        }

        fn string_contains(&mut self, a: &str, b: &str) -> &mut Self {
            self.assert(a.contains(b), IS_CONTAINS, SHOULD_CONTAINS)
        }

        fn file_contains(&mut self, f: &str, v: &str) -> &mut Self {
            self.assert(
                fs::read_to_string(f)
                    .unwrap_or_else(|_| panic!("The filename {} has not been founded", f))
                    .contains(v),
                IS_CONTAINS,
                SHOULD_CONTAINS,
            )
        }

        fn hash_contains(&mut self, a: &mut HashSet<String>, b: String) -> &mut Self {
            self.assert(a.contains(&b), IS_CONTAINS, SHOULD_CONTAINS)
        }

        fn not_exists(&mut self, p: &str) -> &mut Self {
            self.assert(!Path::new(p).exists(), IS_NOT_EXISTS, SHOULD_NOT_EXISTS)
        }

        fn vec_no_contains<T: PartialEq>(&mut self, a: Vec<T>, b: T) -> &mut Self {
            self.assert(!a.contains(&b), IS_NOT_CONTAINS, SHOULD_NOT_CONTAINS)
        }

        fn is_program(&mut self, p: &str) -> &mut Self {
            self.assert(
                Path::new(p).is_executable(),
                IS_EXECUTABLE,
                SHOULD_BE_EXECUTABLE,
            )
        }

        fn not_program(&mut self, p: &str) -> &mut Self {
            self.assert(
                !Path::new(p).is_executable(),
                IS_NOT_EXECUTABLE,
                SOULD_BE_NOT_EXECUTABLE,
            )
        }
    }
}
