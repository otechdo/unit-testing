use crate::unit::output::{IS_EQUALS, IS_INFERIOR, IS_SUPERIOR, IS_UNEQUALS};

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
    /// # Check if a are greater than b
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
    ///  # Examples
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
    pub fn eq<T: PartialEq>(a: &T, b: &T) {
        assert!(a.eq(b), "{IS_UNEQUALS}");
    }
    ///
    /// # Check if a and b are unequals
    ///
    /// - `a`   The first value
    /// - `b`   The second value
    ///
    /// # Panics
    ///
    /// if a is not unequal to b
    ///
    pub fn ne<T: PartialEq>(a: &T, b: &T) {
        assert!(a.ne(b), "{IS_EQUALS}");
    }
    ///
    /// # Check if a is greater than b
    ///
    /// - `a`   The first value
    /// - `b`   The second value
    ///
    /// # Panics
    ///
    /// if a is not greater than b
    ///
    pub fn ge<T: PartialOrd>(a: &T, b: &T) {
        assert!(a.ge(b), "{IS_INFERIOR}");
    }

    ///
    /// # Check if a is greater than b
    ///
    /// - `a`   The first value
    /// - `b`   The second value
    ///
    /// # Panics
    ///
    /// if a is not greater than b
    ///
    pub fn gt<T: PartialOrd>(a: &T, b: &T) {
        assert!(a.gt(b), "{IS_INFERIOR}");
    }
    ///
    /// # Check if a is lower or equal than b
    ///
    /// - `a`   The first value
    /// - `b`   The second value
    ///
    /// # Panics
    ///
    /// if a is not lower than b
    ///
    pub fn le<T: PartialOrd>(a: &T, b: &T) {
        assert!(a.le(b), "{IS_SUPERIOR}");
    }
    ///
    /// # Check if a is lower than b
    ///
    /// - `a`   The first value
    /// - `b`   The second value
    ///
    /// # Panics
    ///
    /// if a is not lower than b
    ///
    /// # Examples
    ///
    /// ```
    /// use unit_testing::unit::check::Assertion;
    /// Assertion::lt(&1, &40);
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
