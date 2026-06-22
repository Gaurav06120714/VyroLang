//! Tests for arrays, the standard library, and classes.

use vyro::run_source;

fn run(src: &str) -> String {
    run_source(src).expect("program should run without error")
}

// ---- Arrays ----

#[test]
fn array_literal_and_index() {
    assert_eq!(run("let a = [10, 20, 30]\nprint(a[0], a[2])"), "10 30\n");
}

#[test]
fn array_index_assign() {
    let out = run(r#"
        let a = [1, 2, 3]
        a[1] = 99
        print(a)
    "#);
    assert_eq!(out, "[1, 99, 3]\n");
}

#[test]
fn array_push_pop_len() {
    let out = run(r#"
        let a = [1, 2]
        push(a, 3)
        push(a, 4)
        print(len(a))
        print(pop(a))
        print(a)
    "#);
    assert_eq!(out, "4\n4\n[1, 2, 3]\n");
}

#[test]
fn array_iteration_sum() {
    let out = run(r#"
        let nums = [5, 10, 15, 20]
        let total = 0
        for i in 0..len(nums) {
            total = total + nums[i]
        }
        print(total)
    "#);
    assert_eq!(out, "50\n");
}

#[test]
fn array_out_of_bounds_errors() {
    let err = run_source("let a = [1]\nprint(a[5])").unwrap_err();
    assert!(err.contains("out of bounds"), "got: {}", err);
}

// ---- Standard library ----

#[test]
fn stdlib_numeric() {
    assert_eq!(run("print(sqrt(16.0))"), "4.0\n");
    assert_eq!(run("print(abs(-7))"), "7\n");
    assert_eq!(run("print(pow(2, 10))"), "1024\n");
    assert_eq!(run("print(min(3, 1, 2))"), "1\n");
    assert_eq!(run("print(max(3, 1, 2))"), "3\n");
    assert_eq!(run("print(floor(3.7))"), "3\n");
    assert_eq!(run("print(ceil(3.2))"), "4\n");
}

#[test]
fn stdlib_conversions_and_strings() {
    assert_eq!(run(r#"print(int("42") + 1)"#), "43\n");
    assert_eq!(run(r#"print(float("1.5") * 2.0)"#), "3.0\n");
    assert_eq!(run(r#"print(str(123) + "!")"#), "123!\n");
    assert_eq!(run(r#"print(upper("vyro"))"#), "VYRO\n");
    assert_eq!(run(r#"print(lower("VYRO"))"#), "vyro\n");
    assert_eq!(run(r#"print(type([1, 2]))"#), "Array\n");
    assert_eq!(run(r#"print(type(3.14))"#), "Float\n");
}

#[test]
fn string_length_and_index() {
    assert_eq!(run(r#"print(len("hello"))"#), "5\n");
    assert_eq!(run(r#"print("hello"[1])"#), "e\n");
}

// ---- Classes ----

#[test]
fn class_init_and_method() {
    let out = run(r#"
        class User {
            name
            age
            func init(name, age) {
                self.name = name
                self.age = age
            }
            func describe() {
                return self.name + " (" + self.age + ")"
            }
        }
        let u = User("Gaurav", 20)
        print(u.describe())
    "#);
    assert_eq!(out, "Gaurav (20)\n");
}

#[test]
fn class_field_access_and_mutation() {
    let out = run(r#"
        class Counter {
            func init() {
                self.count = 0
            }
            func inc() {
                self.count = self.count + 1
            }
        }
        let c = Counter()
        c.inc()
        c.inc()
        c.inc()
        print(c.count)
    "#);
    assert_eq!(out, "3\n");
}

#[test]
fn class_method_calls_other_method() {
    let out = run(r#"
        class Rect {
            func init(w, h) {
                self.w = w
                self.h = h
            }
            func area() {
                return self.w * self.h
            }
            func describe() {
                return "area = " + self.area()
            }
        }
        let r = Rect(4, 5)
        print(r.describe())
    "#);
    assert_eq!(out, "area = 20\n");
}

#[test]
fn class_without_init() {
    let out = run(r#"
        class Bag {
            func fill() {
                self.items = [1, 2, 3]
                return len(self.items)
            }
        }
        let b = Bag()
        print(b.fill())
    "#);
    assert_eq!(out, "3\n");
}

#[test]
fn method_arity_error() {
    let err = run_source(r#"
        class A {
            func go(x) { return x }
        }
        let a = A()
        print(a.go())
    "#)
    .unwrap_err();
    assert!(err.contains("expects 1 argument"), "got: {}", err);
}

// ---- Maps ----

#[test]
fn map_literal_get_set() {
    let out = run(r#"
        let m = { "a": 1, "b": 2 }
        print(m["a"], m["b"])
        m["c"] = 3
        print(m["c"])
        print(len(m))
    "#);
    assert_eq!(out, "1 2\n3\n3\n");
}

#[test]
fn map_missing_key_is_null_and_has() {
    let out = run(r#"
        let m = { "x": 10 }
        print(m["y"])
        print(has(m, "x"), has(m, "y"))
    "#);
    assert_eq!(out, "null\ntrue false\n");
}

#[test]
fn map_keys_iteration() {
    let out = run(r#"
        let m = { "a": 1, "b": 2, "c": 3 }
        let ks = keys(m)
        let total = 0
        for i in 0..len(ks) {
            total = total + m[ks[i]]
        }
        print(total)
    "#);
    assert_eq!(out, "6\n");
}

// ---- match ----

#[test]
fn match_literal_arms() {
    let out = run(r#"
        func name(n) {
            return match n {
                1 -> "one"
                2 -> "two"
                _ -> "many"
            }
        }
        print(name(1))
        print(name(2))
        print(name(9))
    "#);
    assert_eq!(out, "one\ntwo\nmany\n");
}

#[test]
fn match_string_subject() {
    let out = run(r#"
        let cmd = "stop"
        let code = match cmd {
            "go" -> 1
            "stop" -> 0
            _ -> -1
        }
        print(code)
    "#);
    assert_eq!(out, "0\n");
}

#[test]
fn match_no_wildcard_falls_to_null() {
    let out = run(r#"
        let r = match 5 { 1 -> "a" 2 -> "b" }
        print(r)
    "#);
    assert_eq!(out, "null\n");
}

#[test]
fn map_del_removes_key() {
    let out = run(r#"
        let m = { "a": 1, "b": 2 }
        let removed = del(m, "a")
        print(removed)
        print(has(m, "a"))
        print(len(m))
    "#);
    assert_eq!(out, "1\nfalse\n1\n");
}
