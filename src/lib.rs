#![allow(clippy::multiple_crate_versions)]
pub mod assertions;
pub mod objects;
pub mod output;
pub mod suite;
pub mod unit;

///
/// # Failures are prohibited
///
/// - `title` The title
/// - `description` A detailed description
/// - `time` The asserting sleep time
/// - `callbacks` The callbacks to execute
///
#[macro_export]
macro_rules! assert_that {
    ($title:expr,$description:expr,$time:expr,$callbacks:expr) => {
        Assert::it($title, $description, $time, $callbacks);
    };
}

///
/// # Failures are not prohibited
///
/// - `title` A title
/// - `description` A detailed description
/// - `time` The asserting sleep time
/// - `callbacks` The callbacks to execute
///
#[macro_export]
macro_rules! check_that {
    ($title:expr,$description:expr,$time:expr,$callbacks:expr) => {
        Unit::it($title, $description, $time, $callbacks);
    };
}
///
/// # Always panic but disable output message
///
/// - `c` callback
///
#[macro_export]
macro_rules! always_panic {
    () => {
        std::panic::set_hook(Box::new(|_| {}));
        panic!("");
    };
}
///
///
/// # Run test suite
///
/// - `t` The test result (bool)
/// - `s` The test success message
/// - `e` The error message
/// - `before` The before each callback
/// - `after` The after each callback
///
#[macro_export]
macro_rules! run {
    ($t:expr,$s:expr,$e:expr,$before:ident,$after:ident) => {
        if let Some(a) = $before {
            a();
        }
        std::panic::set_hook(Box::new(|_| {
            println!(
                "{}\n",
                format_args!("\t\t{} {}", "*".red().bold(), $e.red().blink().bold())
            );
        }));
        if $t.eq(&false) {
            panic!("");
        }
        println!(
            "{}\n",
            format_args!(
                "\t\t{} {}",
                "".true_color(55, 190, 176).bold(),
                $s.true_color(55, 190, 176).bold()
            )
        );
        if let Some(b) = $after {
            b();
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
    };
}

///
///
/// # Run test suite
///
/// - `title` The test title
/// - `description` The test description message
/// - `before_all` The before all callback
/// - `before` The before each callback
/// - `after_all` The after all callback
/// - `after` The after each callback
/// - `main` The main callback
///
#[macro_export]
macro_rules! it {
    ($title:expr,$description:expr,$before_all:ident,$before:ident,$after_all:ident,$after:ident,$main:ident) => {
        assert!($crate::suite::describe(
            $title,
            $description,
            $after_all,
            $after,
            $before_all,
            $before,
            $main,
        )
        .end()
        .is_ok());
    };
}
