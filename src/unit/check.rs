pub struct Check {}

impl Check {
    ///
    /// # Check if a and b are equals
    ///
    /// - `a`   The first value
    /// - `b`   The second value
    ///
    pub fn eq<T: PartialEq>(a: &T, b: &T) -> bool {
        a.eq(b)
    }

    ///
    /// # Check if a and b are unequals
    ///
    /// - `a`   The first value
    /// - `b`   The second value
    ///
    pub fn ne<T: PartialEq>(a: &T, b: &T) -> bool {
        a.ne(b)
    }

    ///
    /// # Check if a are greater than b
    ///
    /// - `a`   The first value
    /// - `b`   The second value
    ///
    pub fn gt<T: PartialOrd>(a: &T, b: &T) -> bool {
        a.gt(b)
    }

    ///
    /// # Check if a is lower than b
    ///
    /// - `a`   The first value
    /// - `b`   The second value
    ///
    pub fn lt<T: PartialOrd>(a: &T, b: &T) -> bool {
        a.lt(b)
    }

    ///
    /// # Check if a is lower or equal to b
    ///
    /// - `a`   The first value
    /// - `b`   The second value
    ///
    pub fn le<T: PartialOrd>(a: &T, b: &T) -> bool {
        a.le(b)
    }

    ///
    /// # Check if a is greater or equal to b
    ///
    /// - `a`   The first value
    /// - `b`   The second value
    ///
    pub fn ge<T: PartialOrd>(a: &T, b: &T) -> bool {
        a.ge(b)
    }
}
