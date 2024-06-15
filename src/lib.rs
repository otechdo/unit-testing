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
