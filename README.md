# test_pyo3

`cargo test`

```
   Compiling test_pyo3 v0.1.0 (/home/rik/Projects/test_pyo3)
    Finished dev [unoptimized + debuginfo] target(s) in 1.89s
     Running target/debug/deps/pyo3-809f1d315edeb573

running 10 tests
test test_1 ... ok
Traceback (most recent call last):
Traceback (most recent call last):
test test_4 ... ok
  File "<string>", line 1, in <module>
  File "<string>", line 1, in <module>
Traceback (most recent call last):
  File "<string>", line 1, in <module>
NameError: name 'np' is not defined
test test_2 ... FAILED
NameError: name 'np' is not defined
test test_10 ... FAILED
NameError: name 'np' is not defined
test test_3 ... FAILED
test test_7 ... ok
test test_6 ... ok
test test_8 ... ok
test test_5 ... ok
test test_9 ... ok

failures:

---- test_2 stdout ----
Error calling _test_pyo3(): PyErr { type: Py(0x7fa116f8b4e0, PhantomData) }
thread 'test_2' panicked at 'called `Result::unwrap()` on an `Err` value: ()', src/libcore/result.rs:1009:5
note: Run with `RUST_BACKTRACE=1` for a backtrace.

---- test_10 stdout ----
Error calling _test_pyo3(): PyErr { type: Py(0x7fa116f8b4e0, PhantomData) }
thread 'test_10' panicked at 'called `Result::unwrap()` on an `Err` value: ()', src/libcore/result.rs:1009:5

---- test_3 stdout ----
Error calling _test_pyo3(): PyErr { type: Py(0x7fa116f8b4e0, PhantomData) }
thread 'test_3' panicked at 'called `Result::unwrap()` on an `Err` value: ()', src/libcore/result.rs:1009:5


failures:
    test_10
    test_2
    test_3

test result: FAILED. 7 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out
```

`RUST_TEST_THREADS=1 cargo test`

```
    Finished dev [unoptimized + debuginfo] target(s) in 0.07s
     Running target/debug/deps/pyo3-809f1d315edeb573

running 10 tests
test test_1 ... ok
test test_10 ... ok
test test_2 ... ok
test test_3 ... ok
test test_4 ... ok
test test_5 ... ok
test test_6 ... ok
test test_7 ... ok
test test_8 ... ok
test test_9 ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```
