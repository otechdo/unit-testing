///
/// # To run test case with description
///
pub trait It {
    ///
    /// # Describe a test
    ///
    /// - `description` The test description
    /// - `expected`    The test expected result
    /// - `callback`    The callback to execute
    ///
    /// ```
    /// use unit_testing::unit::Describe;
    /// use crate::unit_testing::unit::describe::It;
    ///
    /// fn py()-> bool
    /// {
    ///     4*4 + 3*3 == 25
    /// }
    /// Describe::it("The triangle is rectangle",true,&py)
    /// ```
    ///  
    fn it<T: PartialEq>(description: &str, expected: T, callback: &dyn Fn() -> T);
}
