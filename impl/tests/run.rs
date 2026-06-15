//! End-to-end tests: source -> compile -> run -> captured output.

use vyro::run_source;

fn run(src: &str) -> String {
    run_source(src).expect("program should run without error")
}

#[test]
fn arithmetic_and_precedence() {
    assert_eq!(run("print((5 + 3) * 4)"), "32\n");
    assert_eq!(run("print(2 + 3 * 4)"), "14\n");
    assert_eq!(run("print(10 / 3)"), "3\n"); // integer division
    assert_eq!(run("print(10 % 3)"), "1\n");
    assert_eq!(run("print(10.0 / 4.0)"), "2.5\n");
    assert_eq!(run("print(-5 + 2)"), "-3\n");
}

#[test]
fn variables_and_strings() {
    let out = run(r#"
        let name = "Gaurav"
        let age = 20
        print(name + " is " + age)
    "#);
    assert_eq!(out, "Gaurav is 20\n");
}

#[test]
fn conditionals() {
    let out = run(r#"
        let age = 20
        if age > 18 { print("Adult") } else { print("Minor") }
    "#);
    assert_eq!(out, "Adult\n");
}

#[test]
fn logical_short_circuit() {
    assert_eq!(run("print(true && false)"), "false\n");
    assert_eq!(run("print(false || true)"), "true\n");
    assert_eq!(run("print(!false)"), "true\n");
}

#[test]
fn while_loop_and_mutation() {
    let out = run(r#"
        let i = 0
        let sum = 0
        while i < 5 {
            sum = sum + i
            i = i + 1
        }
        print(sum)
    "#);
    assert_eq!(out, "10\n"); // 0+1+2+3+4
}

#[test]
fn for_range() {
    let out = run(r#"
        let total = 0
        for i in 1..5 {
            total = total + i
        }
        print(total)
    "#);
    assert_eq!(out, "10\n"); // 1+2+3+4
}

#[test]
fn functions_and_recursion() {
    let out = run(r#"
        func fib(n) {
            if n < 2 { return n }
            return fib(n - 1) + fib(n - 2)
        }
        print(fib(10))
    "#);
    assert_eq!(out, "55\n");
}

#[test]
fn function_locals_dont_leak() {
    let out = run(r#"
        func add(a, b) {
            let c = a + b
            return c
        }
        print(add(2, 3))
        print(add(10, 20))
    "#);
    assert_eq!(out, "5\n30\n");
}

#[test]
fn fizzbuzz() {
    let out = run(r#"
        for n in 1..6 {
            if n % 15 == 0 { print("FizzBuzz") }
            else if n % 3 == 0 { print("Fizz") }
            else if n % 5 == 0 { print("Buzz") }
            else { print(n) }
        }
    "#);
    assert_eq!(out, "1\n2\nFizz\n4\nBuzz\n");
}

#[test]
fn runtime_error_div_zero() {
    let err = run_source("print(1 / 0)").unwrap_err();
    assert!(err.contains("division by zero"), "got: {}", err);
}

#[test]
fn type_error_add() {
    let err = run_source(r#"print(true + 1)"#).unwrap_err();
    assert!(err.contains("cannot add"), "got: {}", err);
}

#[test]
fn parse_error_reports_line() {
    let err = run_source("let x = ").unwrap_err();
    assert!(err.contains("line"), "got: {}", err);
}
