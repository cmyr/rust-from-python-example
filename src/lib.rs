
#[macro_use] extern crate cpython;

use cpython::{PyResult, Python, ToPyObject, PyList};

// add bindings to the generated python module
// N.B: names here are important; "rust2py" should be replaced by the lib name in your Cargo.toml
py_module_initializer!(librust2py, initlibrust2py, PyInit_librust2py, |py, m| {
    try!(m.add(py, "__doc__", "Module documentation string"));
    try!(m.add(py, "val", py_fn!(py, val())));
    try!(m.add(py, "fib", py_fn!(py, fib(arg0: u64))));
    try!(m.add(py, "reversed_words", py_fn!(py, reversed_words(text: &str))));
    Ok(())
});


// An example of keeping an actual implementation in a seperate module
// code from http://ehiggs.github.io/2015/07/Python-Modules-In-Rust/ 
mod module_example {
    pub fn fib(n : u64) -> u64 {
    if n < 2 {
        return 1
    }
    let mut prev1 = 1;
    let mut prev2 = 1;
    for _ in 1..n {
        let new = prev1 + prev2;
        prev2 = prev1;
        prev1 = new;
    }
    prev1 
    }

    pub fn val() -> i32 {
        42
    }
}

/// wrapper around a function which takes no arguments
fn val(_: Python) -> PyResult<i32> {
    Ok(module_example::val())
}

/// wrapper for a function which takes one argument
fn fib(_: Python, arg0: u64) -> PyResult<u64> {
    Ok(module_example::fib(arg0))
}

/// fully contained function returning a list
fn reversed_words(py: Python, text: &str) -> PyResult<PyList> {
    let out: Vec<String> = text.split_whitespace()
        .rev()
        .map(|x| x.to_string())
        .collect();
    Ok(out.to_py_object(py))
}

#[cfg(test)]
mod tests {
     use super::module_example;
    #[test]
    fn it_works() {
        assert_eq!(module_example::fib(1), 1);
        assert_eq!(module_example::fib(5), 8);
    }
}
