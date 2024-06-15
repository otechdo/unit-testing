use std::io;
use colored_truecolor::Colorize;
use crate::unit::output::{IS_EQUALS, IS_INFERIOR, IS_SUPERIOR, IS_UNEQUALS};


pub struct Suite
{
    before_each: fn(),
    after_each: fn(),
}

impl Suite {
    pub fn new(before_each: fn(), after_each: fn()) -> Self
    {
        Self {
            before_each,
            after_each,
        }
    }
    ///
    ///
    /// # Panics
    ///
    /// If test fail
    ///
    pub fn run(self, x: bool, s: &str, e: &str) -> Self {
        (self.before_each)();
        assert!(x, "{}", e);
        println!(
            "      {}",
            format_args!("{} {}", "✓".green().bold(), s.blue().bold())
        );
        (self.after_each)();
        self
    }
    pub fn end(&mut self) -> io::Result<()>
    {
        Ok(())
    }
    pub fn eq<X: PartialEq>(self, actual: X, expected: X) -> Self
    {
        self.run(actual.eq(&expected), IS_EQUALS, IS_UNEQUALS)
    }

    pub fn ne<X: PartialEq>(self, actual: X, expected: X) -> Self
    {
        self.run(actual.ne(&expected), IS_UNEQUALS, IS_EQUALS)
    }

    pub fn gt<X: PartialOrd>(self, actual: X, expected: X) -> Self
    {
        self.run(actual.gt(&expected), IS_SUPERIOR, IS_INFERIOR)
    }

    pub fn ge<X: PartialOrd>(self, actual: X, expected: X) -> Self
    {
        self.run(actual.ge(&expected), IS_SUPERIOR, IS_INFERIOR)
    }
    pub fn le<X: PartialOrd>(self, actual: X, expected: X) -> Self
    {
        self.run(actual.le(&expected), IS_INFERIOR, IS_SUPERIOR)
    }
    pub fn lt<X: PartialOrd>(self, actual: X, expected: X) -> Self
    {
        self.run(actual.lt(&expected), IS_INFERIOR, IS_SUPERIOR)
    }
}


pub fn describe(description: &str, after_all_hook: fn(),
                after_each_hook: fn(),
                before_all_hook: fn(),
                before_each_hook: fn(),
                callback: fn(Suite) -> Suite) -> Suite
{
    before_all_hook();
    println!("\n{}\n", description.cyan().bold());
    let data: Suite = callback(Suite::new(before_each_hook, after_each_hook));
    after_all_hook();
    data
}

#[cfg(test)]
mod test
{
    use crate::unit::helpers::describe;


    #[test]
    fn suite() -> std::io::Result<()>
    {
        describe(
            "Suite test case",
            || {},
            ||
            {},
            ||
            {}, || {},
            |s|
            {
                s.eq("ok", "ok").eq("0", "0").eq(2, 2).eq(3, 3)
            },
        ).end()
    }
}
