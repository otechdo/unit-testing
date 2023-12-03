use std::{collections::HashSet, env::consts::OS, process::ExitCode};

use num::Float;
use unit::unit::{traits::unit::{Testable, Theory}, Assert, Describe};
use unit::unit::describe::unit::It;

fn ok() -> bool {
    true
}

fn ko() -> bool {
    false
}

fn must_pass(u: &mut Assert) -> &mut Assert {
    u.ok(&ok).ko(&ko)
}

fn must_exists(u: &mut Assert) -> &mut Assert {
    u.exists(".").exists("README.md").exists("/").exists("/lib")
}

fn must_linux(u: &mut Assert) -> &mut Assert {
    u.not_exists("C:\\Users")
        .not_exists("C:\\ProgramData")
        .not_exists("C:\\WINDOWS\\system32")
}

fn must_equals(u: &mut Assert) -> &mut Assert {
    u.equals("README.md", "README.md")
        .equals(4, 4)
        .equals(4.4, 4.4)
        .equals(true, true)
        .equals(false, false)
}

fn must_contains(u: &mut Assert) -> &mut Assert {
    let mut v: Vec<String> = Vec::new();
    let o = Some("a".to_string());
    v.push("value".to_string());
    v.push("h".to_string());
    u.vec_contains(v, "h".to_string())
        .option_contains(o, "a".to_string())
        .string_contains(OS, "linux")
        .file_contains("README.md", "Installation")
        .hash_contains(&mut HashSet::from(["a".to_string()]), "a".to_string())
}

fn must_unequals(u: &mut Assert) -> &mut Assert {
    u.unequals("README.md", ".")
        .unequals(4, 6)
        .unequals(5.6, 4.4)
        .unequals(false, true)
        .unequals(false, true)
}

fn must_superior(u: &mut Assert) -> &mut Assert {
    u.superior(1, 0).superior(5, 2)
}

fn programs(u: &mut Assert) -> &mut Assert {
    u.program("/usr/bin/git").program("/usr/bin/curl")
}

fn no_programs(u: &mut Assert) -> &mut Assert {
    u.not_program("cmd")
}

fn must_inferior(u: &mut Assert) -> &mut Assert {
    u.inferior(10, 50).inferior(50, 200)
}

fn must_beetween(u: &mut Assert) -> &mut Assert {
    u.between(10, 5, 50).between(50, 10, 200)
}

fn pythagore() -> f32 {
    Float::hypot(3.0, 4.0)
}

fn pythagore_not_work() -> bool {
    Float::hypot(4.0, 4.0) == 5.0
}

fn must_theory(u: &mut Assert) -> &mut Assert {
    u.theory(5.0, &pythagore).chaos(&pythagore_not_work)
}

fn main() -> ExitCode {
    Describe::it("Ok should be return true", true, &ok);
    Describe::it("Ko should be return false", false, &ko);

    Assert::it(vec![
        &must_beetween,
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
    ])
}
