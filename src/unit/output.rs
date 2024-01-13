pub const ASSERT_MATCH: &str = "The value match the regex";
pub const ASSERT_SHOULD_MATCH: &str = "The value should be match the regex";
pub const ASSERT_BEGIN: &str = "The data begin with the expected value";
pub const ASSERT_FAIL: &str = "The callback has a failure exit status code";
pub const ASSERT_SUCCESS: &str = "The callback has a success exit status code";
pub const ASSERT_SHOULD_BE_SUCCESS: &str = "The callback should have a success exit status code";
pub const ASSERT_SHOULD_BE_FAIL: &str = "The callback should be have a failure exit status code";
pub const ASSERT_FINNISH: &str = "The data finnish by the expected value";
pub const ASSERT_SHOULD_BE_BEGIN: &str =
    "The data don't begin with the expected value and should be begin with";
pub const ASSERT_SHOULD_BE_FINNISH: &str =
    "The data don't finnish with the value and she should be";
pub const ASSERT_OK: &str = "The value match true";
pub const ASSERT_SHOULD_BE_OK: &str = "The value match false and should be match true";
pub const ASSERT_KO: &str = "The value match false";
pub const ASSERT_SHOULD_BE_KO: &str = "The value match true and should be match false";
pub const ASSERT_EQUALS: &str = "The values are equals";
pub const ASSERT_SHOULD_BE_EQUALS: &str = "The values match inequality but should be equals";
pub const ASSERT_UNEQUALS: &str = "The values are unequals";
pub const ASSERT_SHOULD_BE_UNEQUALS: &str = "The values are equals but values should be unequals";
pub const ASSERT_BETWEEN: &str = "The value are between min and max";
pub const ASSERT_SHOULD_BE_BETWEEN: &str =
    "The value are not between min and max but should be between the values";
pub const ASSERT_INFERIOR: &str = "The value is inferior to the maximum value";
pub const ASSERT_SHOULD_BE_INFERIOR: &str = "The value is inferior to the maximum value";
pub const ASSERT_SUPERIOR: &str = "The value is superior to the minimum value";
pub const ASSERT_SHOULD_BE_SUPERIOR: &str =
    "The value is inferior to the minimum value and should be superior";

pub const ASSERT_EMPTY: &str = "The value is empty";
pub const ASSERT_SHOULD_BE_EMPTY: &str = "The value is not empty but should be empty";

pub const ASSERT_EXISTS: &str = "The path exists";
pub const ASSERT_SHOULD_BE_EXISTS: &str = "The path not exist but it's should be exists";

pub const ASSERT_CONTAINS: &str = "The value contains the expected value";
pub const ASSERT_SHOULD_CONTAINS: &str = "The value has not been founded";
pub const ASSERT_IS_EXECUTABLE: &str = "The file is executable";
pub const ASSERT_SHOULD_BE_EXECUTABLE: &str = "The file is not an executable but it's should be";
pub const ASSERT_THEORY_IS_TRUE: &str = "The theory is true";
pub const ASSERT_THEORY_SHOULD_BE_TRUE: &str =
    "The theory match false but she should be match true";
pub const ASSERT_THEORY_IS_FALSE: &str = "The theory is false";
pub const ASSERT_THEORY_SHOULD_BE_FALSE: &str = "The theory match true she should be match false";
pub const ASSERT_IS_NOT_EXECUTABLE: &str = "The file is not an executable";
pub const ASSERT_SHOULD_BE_NOT_EXECUTABLE: &str = "The file is executable and should be not";
pub const ASSERT_NOT_EXISTS: &str = "The path not exists";
pub const ASSERT_SHOULD_BE_NOT_EXISTS: &str = "The path exists and should be not";
pub const ASSERT_NOT_CONTAINS: &str = "The data don't contains the value";
pub const ASSERT_SHOULD_BE_NOT_CONTAINS: &str =
    "The value has been founded in data and should be not";
pub const ASSERT_NOT_EMPTY: &str = "The value is not empty";
pub const ASSERT_SHOULD_BE_NOT_EMPTY: &str = "The value is empty and should be not";
pub const ASSERT_PROGRESS_TIME: u64 = 100;
pub const DISABLE_PROGRESS_TIME: u64 = 0;
pub const IS_MATCH: &str = "The value match the regex";
pub const IS_NOT_MATCH: &str = "The value no match the regex";
pub const IS_FAIL: &str = "The callback has a failure exit status code";
pub const IS_NOT_FAIL: &str = "The callback has not a failure exit status";
pub const IS_SUCCESS: &str = "The callback has a success exit status code";
pub const IS_NOT_SUCCESS: &str = "The callback has not a success exit status code";
pub const IS_BEGIN: &str = "The value begin with the expected value";
pub const IS_FINNISH: &str = "The value finnish by the expected value";
pub const IS_NOT_FINNISH: &str = "The value don't finnish by the expected value";
pub const IS_NOT_BEGIN: &str = "The value not begin with the expected value";
pub const IS_OK: &str = "The test match true";
pub const IS_KO: &str = "The test match false";
pub const IS_EQUALS: &str = "The values are equals";
pub const IS_UNEQUALS: &str = "The values are unequals";
pub const IS_BETWEEN: &str = "The value is between min and max";
pub const IS_NOT_BETWEEN: &str = "The value is not between min and max";
pub const IS_INFERIOR: &str = "The value is inferior to the maximum value";
pub const IS_SUPERIOR: &str = "The value is superior to the minimum value";
pub const IS_EMPTY: &str = "The value is empty";
pub const IS_NOT_EMPTY: &str = "The value is not empty";
pub const IS_EXISTS: &str = "The path exists";
pub const IS_CONTAINS: &str = "The value contains the data";
pub const IS_NOT_CONTAINS: &str = "The value don't contains the data";
pub const IS_EXECUTABLE: &str = "The file is executable";
pub const IS_NOT_EXECUTABLE: &str = "The file is not an executable";
pub const THEORY_IS_TRUE: &str = "The theory is true";
pub const THEORY_IS_FALSE: &str = "The theory if false";
pub const IS_NOT_EXISTS: &str = "The path not exists";
pub const UNIT_PROGRESS_TIME: u64 = 100;
