///
/// # Failures are prohibited
///
/// - `describe`    The description
/// - `time`        The sleep time
/// - `callbacks`   The callbacks to execute
///
#[macro_export]
macro_rules! assert_that {
    ($describe:expr,$time:expr,$callbacks:expr) => {
        Assert::it($describe, $time, $callbacks);
    };
}

///
/// # Failures are not prohibited
///
/// - `describe`    The description
/// - `time`        The sleep time
/// - `callbacks`   The callbacks to execute
///
#[macro_export]
macro_rules! check_that {
    ($describe:expr,$time:expr,$callbacks:expr) => {
        Unit::it($describe, $time, $callbacks);
    };
}

///
/// # Check with a personnal message  
///
/// - `description` The global description
///
#[macro_export]
macro_rules! describe {
    ($description:expr) => {
        $crate::unit::Describe::new($description)
    };
}
