///
/// # Failures are prohibited
///
/// - `describe`    The assert description
/// - `time`        The assert sleep time
/// - `callbacks`    The callbacks to execute
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
/// - `describe`    The assert description
/// - `time`        The assert sleep time
/// - `callbacks`    The callbacks to execute
///
#[macro_export]
macro_rules! check_that {
    ($describe:expr,$time:expr,$callbacks:expr) => {
        Unit::it($describe, $time, $callbacks);
    };
}
