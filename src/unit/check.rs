use crate::unit::output::{IS_EQUALS, IS_INFERIOR, IS_SUPERIOR, IS_UNEQUALS};

///
/// # To check with failures
///
pub struct Check {}

impl Check {
    ///
    /// # Check if a and b are equals
    ///
    /// - `a`   The first value
    /// - `b`   The second value
    ///
    /// # Examples
    ///
    /// ```
    /// use unit_testing::unit::check::Check;
    ///
    /// assert_eq!(Check::eq(&41, &41), true);
    /// assert_eq!(Check::eq(&false,&false), true);
    /// assert_eq!(Check::eq(&3.41, &3.41), true);
    /// assert_eq!(Check::eq(&true,&true), true);
    /// assert_eq!(Check::eq(&"true",&"true"), true);
    /// assert_eq!(Check::eq(&"false",&"false"), true);
    /// ```
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
    /// # Examples
    ///
    /// ```
    /// use unit_testing::unit::check::Check;
    ///
    /// assert_eq!(Check::ne(&41, &42), true);
    /// assert_eq!(Check::ne(&true,&false), true);
    /// assert_eq!(Check::ne(&1.62, &3.41), true);
    /// assert_eq!(Check::ne(&false,&true), true);
    /// assert_eq!(Check::ne(&"false",&"true"), true);
    /// assert_eq!(Check::ne(&"true",&"false"), true);
    /// ```
    ///
    pub fn ne<T: PartialEq>(a: &T, b: &T) -> bool {
        a.ne(b)
    }

    ///
    /// # Check if a is greater than b
    ///
    /// - `a`   The first value
    /// - `b`   The second value
    ///
    ///  # Examples
    ///
    /// ```
    /// use unit_testing::unit::check::Check;
    ///
    /// assert_eq!(Check::gt(&43, &42), true);
    /// assert_eq!(Check::gt(&true,&false), true);
    /// assert_eq!(Check::gt(&3.14, &1.62), true);
    /// ```
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
    /// # Examples
    ///
    /// ```
    /// use unit_testing::unit::check::Check;
    ///
    /// assert_eq!(Check::lt(&42, &43), true);
    /// assert_eq!(Check::lt(&false,&true), true);
    /// assert_eq!(Check::lt(&1.62, &3.14), true);
    /// ```
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
    /// # Examples
    ///
    /// ```
    /// use unit_testing::unit::check::Check;
    ///
    /// assert_eq!(Check::le(&42, &42), true);
    /// assert_eq!(Check::le(&true,&true), true);
    /// assert_eq!(Check::le(&1.62, &1.62), true);
    /// ```
    pub fn le<T: PartialOrd>(a: &T, b: &T) -> bool {
        a.le(b)
    }

    ///
    /// # Check if a is greater or equal to b
    ///
    /// - `a`   The first value
    /// - `b`   The second value
    ///
    /// # Examples
    ///
    /// ```
    /// use unit_testing::unit::check::Check;
    ///
    /// assert_eq!(Check::ge(&42, &42), true);
    /// assert_eq!(Check::ge(&true,&true), true);
    /// assert_eq!(Check::ge(&1.62, &1.62), true);
    /// ```
    pub fn ge<T: PartialOrd>(a: &T, b: &T) -> bool {
        a.ge(b)
    }
}

///
/// # To check without failures
///
pub struct Assertion {}
impl Assertion {
    ///
    /// # Check if a and b are equals
    ///
    /// - `a`   The first value
    /// - `b`   The second value
    ///
    /// # Panics
    ///
    /// If a not equal to b
    ///
    /// # Examples
    ///
    /// ```
    /// use unit_testing::unit::check::Assertion;
    ///
    /// Assertion::eq(&1,&1);
    /// Assertion::eq(&true,&true);
    /// Assertion::eq(&false,&false);
    /// Assertion::eq(&3.14,&3.14);
    /// ```
    ///
    pub fn eq<T: PartialEq>(a: &T, b: &T) {
        assert!(a.eq(b), "{IS_UNEQUALS}");
    }

    ///
    /// # Check if a and b are unequals
    ///
    /// - `a`   The first value
    /// - `b`   The second value
    ///
    /// # Panic
    ///
    /// if a is not unequal to b
    ///
    /// # Examples
    ///
    /// ```
    /// use unit_testing::unit::check::Assertion;
    ///
    /// Assertion::ne(&1,&10);
    /// Assertion::ne(&true,&false);
    /// Assertion::ne(&false,&true);
    /// Assertion::ne(&3.14,&1.62);
    /// ```
    pub fn ne<T: PartialEq>(a: &T, b: &T) {
        assert!(a.ne(b), "{IS_EQUALS}");
    }
    ///
    /// # Check if a is greater than b
    ///
    /// - `a`   The first value
    /// - `b`   The second value
    ///
    /// # Panics
    ///
    /// if a is not greater than b
    ///
    /// # Examples
    ///
    /// ```
    /// use unit_testing::unit::check::Assertion;
    ///
    /// Assertion::ge(&1,&1);
    /// Assertion::ge(&true,&true);
    /// Assertion::ge(&false,&false);
    /// Assertion::ge(&3.14,&3.14);
    /// ```
    pub fn ge<T: PartialOrd>(a: &T, b: &T) {
        assert!(a.ge(b), "{IS_INFERIOR}");
    }

    ///
    /// # Check if a is greater than b
    ///
    /// - `a`   The first value
    /// - `b`   The second value
    ///
    /// # Panics
    ///
    /// if a is not greater than b
    ///
    /// # Example
    ///
    /// ```
    /// use unit_testing::unit::check::Assertion;
    ///
    /// Assertion::gt(&10,&1);
    /// Assertion::gt(&true,&false);
    /// Assertion::gt(&3.14,&1.62);
    /// ```
    pub fn gt<T: PartialOrd>(a: &T, b: &T) {
        assert!(a.gt(b), "{IS_INFERIOR}");
    }
    ///
    /// # Check if a is lower or equal than b
    ///
    /// - `a`   The first value
    /// - `b`   The second value
    ///
    /// # Panics
    ///
    /// if a is not lower than b
    ///
    /// # Example
    ///
    /// ```
    /// use unit_testing::unit::check::Assertion;
    ///
    /// Assertion::le(&3,&10);
    /// Assertion::le(&false,&false);
    /// Assertion::le(&false,&true);
    /// Assertion::le(&true,&true);
    /// Assertion::le(&3,&3);
    /// Assertion::le(&3,&5);
    /// Assertion::le(&3.0,&3.14);
    /// Assertion::le(&3.14,&4.0);
    /// ```
    pub fn le<T: PartialOrd>(a: &T, b: &T) {
        assert!(a.le(b), "{IS_SUPERIOR}");
    }
    ///
    /// # Check if a is lower than b
    ///
    /// - `a`   The first value
    /// - `b`   The second value
    ///
    /// # Panics
    ///
    /// if a is not lower than b
    ///
    /// # Examples
    ///
    /// ```
    /// use unit_testing::unit::check::Assertion;
    /// Assertion::lt(&3,&10);
    /// Assertion::lt(&3,&5);
    /// Assertion::lt(&3,&5);
    /// Assertion::lt(&false,&true);
    /// Assertion::lt(&3.0,&3.14);
    /// Assertion::lt(&3.14,&4.0);
    /// ```
    ///
    pub fn lt<T: PartialOrd>(a: &T, b: &T) {
        assert!(a.lt(b), "{IS_INFERIOR}");
    }
}

#[cfg(test)]
mod test {
    use super::Assertion;

    #[test]
    #[should_panic]
    fn test_assert() {
        Assertion::eq(&1, &2);
        Assertion::ne(&2, &2);
        Assertion::ge(&0, &11);
        Assertion::gt(&01, &2);
        Assertion::lt(&11, &2);
        Assertion::le(&11, &2);
    }

    #[test]
    fn test_assert_continue() {
        Assertion::eq(&1, &1);
        Assertion::ne(&1, &2);
        Assertion::ge(&420, &420);
        Assertion::gt(&55, &22);
        Assertion::lt(&4, &12);
        Assertion::le(&2, &2);
    }
}
