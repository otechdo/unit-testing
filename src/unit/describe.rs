pub mod unit {
    pub trait It {
        fn it<T: PartialEq>(description: &str, expected: T, callback: &dyn Fn() -> T);
    }
}
