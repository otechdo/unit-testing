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
/// # Get an instance of Unit
///
/// - `describe`    The description
/// - `type`        The Apply type
/// - `time`        The sleep time
/// - `callbacks`   The callback to execute
///
#[macro_export]
macro_rules! apply_for {
    ($describe:expr,$type:ty,$time:expr,callbacks:expr) => {
        match $type {
            Apply::ASSERT => {
                assert_that!($describe, $time, $callbacks)
            }
            Apply::UNIT => {
                check_that!($describe, $time, $callbacks)
            }
        }
    };
}
