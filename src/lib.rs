
#[macro_use] extern crate cpython;

use cpython::{PyResult, Python, PyObject, PyErr, exc, ToPyObject, PythonObject, PyString, PyList};

// add bindings to the generated python module
// N.B: names here are important; "rust2py" should be replaced by the lib name in your Cargo.toml
py_module_initializer!(librust2py, initlibrust2py, PyInit_librust2py, |py, m| {
    try!(m.add(py, "__doc__", "Module documentation string"));
	try!(m.add(py, "val", py_fn!(py, val())));
	try!(m.add(py, "fib", py_fn!(py, fib2(arg0: PyObject))));
	try!(m.add(py, "reverse", py_fn!(py, reverse_words(text: PyObject))));
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
fn fib2(py: Python, arg0: PyObject) -> PyResult<u64> {
	let arg = match arg0.extract::<u64>(py) {
		Ok(x) => x,
		Err(_) => {
			let errmsg = "colin doesn't know rust";
			let pyerr = PyErr::new_lazy_init(py.get_type::<exc::ValueError>(), Some(errmsg.to_py_object(py).into_object()));
			return Err(pyerr);
		}
	};
	Ok(module_example::fib(arg))

}

/// fully contained function returning a list
fn reversed_words(py: Python, text: PyObject) -> PyResult<PyList> {
	
	let inp = match text.extract::<PyString>(py) {
		Ok(x) => x.to_string_lossy(py).into_owned(),
		Err(_) => {
			let errmsg = "invalid type, we have type checking over here it's nice"
				.to_py_object(py)
				.into_object();
			return Err(PyErr::new_lazy_init(py.get_type::<exc::ValueError>(), Some(errmsg)));
		}
	};
	let out: Vec<String> = inp.split_whitespace()
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
