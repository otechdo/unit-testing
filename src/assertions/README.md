# Usage

````rust
#[cfg(test)]
mod test {
    use std::collections::HashSet;
    use unit_testing::assert_that;
    use unit_testing::assertions::Assert;
    use unit_testing::objects::{Testable, Theory};
    use unit_testing::output::ASSERT_PROGRESS_TIME;

    fn ok() -> bool {
        true
    }

    fn ko() -> bool {
        false
    }

    fn must_pass(u: &mut Assert) -> &mut Assert {
        u.ok(ok()).ko(ko())
    }

    fn must_exists(u: &mut Assert) -> &mut Assert {
        u.exists(".").exists("README.md")
    }

    fn must_linux(u: &mut Assert) -> &mut Assert {
        u
    }

    fn must_equals(u: &mut Assert) -> &mut Assert {
        u.eq("README.md", "README.md")
            .eq(4, 4)
            .eq(4.4, 4.4)
            .eq(true, true)
            .eq(false, false)
    }

    fn must_contains(u: &mut Assert) -> &mut Assert {
        let mut v: Vec<String> = Vec::new();
        let o = Some("a".to_string());
        v.push("value".to_string());
        v.push("h".to_string());
        u.vec_contains(v, "h".to_string())
            .option_contains(o, "a".to_string())
            .str_contains("linux", "linux")
            .file_contains("README.md", "Installation")
            .hash_contains(&mut HashSet::from(["a".to_string()]), "a".to_string())
    }

    fn must_unequals(u: &mut Assert) -> &mut Assert {
        u.ne("README.md", ".")
            .ne(4, 6)
            .ne(5.6, 4.4)
            .ne(false, true)
            .ne(false, true)
    }

    fn must_superior(u: &mut Assert) -> &mut Assert {
        u.gt(1, 0).gt(5, 2)
    }

    fn programs(u: &mut Assert) -> &mut Assert {
        u
    }

    fn no_programs(u: &mut Assert) -> &mut Assert {
        u
    }

    fn must_inferior(u: &mut Assert) -> &mut Assert {
        u.lt(10, 50).lt(50, 200)
    }

    fn must_between(u: &mut Assert) -> &mut Assert {
        u.between(10, 5, 50).between(50, 10, 200)
    }

    fn pythagore() -> f32 {
        3.0_f32.hypot(4.0)
    }

    fn pythagore_not_work() -> bool {
        4.0_f32.hypot(4.0) == 5.0
    }

    fn must_theory(u: &mut Assert) -> &mut Assert {
        u.theory(5.0, &pythagore).chaos(&pythagore_not_work)
    }


    #[test]
    pub fn all() {
        assert_that!(
            "Test the assert framework",
            "Check if all values passes on success, can't be have failures.",
            ASSERT_PROGRESS_TIME,
            vec![
                &must_between,
                &programs,
                &must_theory,
                &no_programs,
                &must_unequals,
                &must_linux,
                &must_equals,
                &must_exists,
                &must_pass,
                &must_contains,
                &must_superior,
                &must_inferior,
            ]
        );
    }
}
````

## Output

![Assertion](https://raw.githubusercontent.com/otechdo/unit-testing/master/src/assertions/assert.gif)