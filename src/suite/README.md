# Usage

```rust
#[cfg(test)]
mod test {
    use std::fs;
    use unit_testing::suite::describe;

    #[test]
    fn suite() -> std::io::Result<()> {
        describe(
            "Check the suite it test case",
            "Suite test accept no test failure, for guaranty the source code.",
            || {
                println!("I'm executed after all test");
            },
            || {
                println!("I'm executed after each test");
            },
            ||
            {
                println!("I'm executed before all test");
            },
            || {
                println!("I'm executed before each test");
            },
            |s| {
                s.group("Should be contains", |s| {
                    s.str_contains(
                        fs::read_to_string("README.md").expect("Failed to parse README.md"),
                        "cargo add unit-testing",
                    )
                })
                    .group("Check path", |s| {
                        s.path_exists("README.md", true)
                            .path_exists(".", true)
                            .path_exists("alexandrie", false)
                            .exists(".")
                            .exists("README.md")
                    })
                    .group("Should be not contains", |s| {
                        s.str_not_contains(
                            fs::read_to_string("README.md").expect("Failed to parse README.md"),
                            "cargo add continuous-testing",
                        )
                    })
                    .group("Should be equals", |s| s.eq(1, 1).eq(2, 2))
                    .group("Should be unequal", |s| s.ne(1, 2).ne(3, 2))
            },
        )
            .end()
    }
}
```

![Suite](https://raw.githubusercontent.com/taishingi/unit-testing/master/suite.gif)