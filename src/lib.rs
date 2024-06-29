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

#[macro_export]
macro_rules! always_panic {
    () => {
        std::panic::set_hook(Box::new(|_| {}));
        panic!("This panic will have custom output");
    };
}

#[macro_export]
macro_rules! run {
    ($t:expr,$s:expr,$f:expr,$before:ident,$after:ident) => {
        $before();
        std::panic::set_hook(Box::new(|_| {
            println!(
                "      {}",
                format_args!("{} {}", "*".red().bold(), $f.yellow().bold())
            );
        }));
        if $t.eq(&false) {
            $after();
            panic!("Test failed");
        } else {
            println!(
                "      {}",
                format_args!("{} {}", "✓".green().bold(), $s.cyan().bold())
            );
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
    };
}
#[macro_export]
macro_rules! describe {
    ($title:expr,$description:expr,$before_all:ident,$before:ident,$after_all:ident,$after:ident,$main:ident) => {
        $crate::suite::describe(
            $title,
            $description,
            $after_all,
            $after,
            $before_all,
            $before,
            $main,
        )
    };
}
