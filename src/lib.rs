
#[macro_use] extern crate cpython;

// use cpython::{PyObject, PyResult, Python, PyTuple, PyDict};

// py_module_initializer!(libpyoxide, libpyoxide, PyInit_libpyoxide, |py, m| {
//     try!(m.add(py, "__doc__", "Module documentation string"));
//     try!(m.add(py, "run", py_fn!(py, run(*args, **kwargs))));
//     try!(m.add(py, "val", py_fn!(py, val())));
//     Ok(())
// });

// fn run(py: Python, args: &PyTuple, kwargs: Option<&PyDict>) -> PyResult<PyObject> {
//     println!("Rust says: Hello Python!");
//     for arg in args.iter(py) {
//         println!("Rust got {}", arg);
//     }
//     if let Some(kwargs) = kwargs {
//         for (key, val) in kwargs.items(py) {
//             println!("{} = {}", key, val);
//         }
//     }
//     Ok(py.None())
// }

// fn val(_: Python) -> PyResult<i32> {
//     Ok(42)
// }


use cpython::{PyResult, Python, PyTuple, PyDict, PyObject, PyErr, exc, ToPyObject, PythonObject, PyString, PyList}; // PyTuple, PyErr, PyDict, PyObject, ToPyObject, PythonObject};

mod pyoxide {
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


py_module_initializer!(libpyoxide, initlibpyoxide, PyInit_libpyoxide, |py, m| {
    try!(m.add(py, "__doc__", "Module documentation string"));
	try!(m.add(py, "val", py_fn!(py, val())));
	try!(m.add(py, "fib", py_fn!(py, fib(*args, **kwargs))));
	try!(m.add(py, "fib2", py_fn!(py, fib2(arg0: PyObject))));
	try!(m.add(py, "reverse", py_fn!(py, tokenize(text: PyObject))));
    Ok(())
});


fn val(_: Python) -> PyResult<i32> {
	Ok(pyoxide::val())
}

fn fib(py: Python, args: &PyTuple, kwargs: Option<&PyDict>) -> PyResult<u64> {
	// let arg0 = match args.get_item(0).extract::<u64>() {
	let arg0 = match args.get_item(py, 0).extract::<u64>(py) {
		Ok(x) => x,
		Err(_) => 1337,
	};
	
	Ok(pyoxide::fib(arg0))
}

fn fib2(py: Python, arg0: PyObject) -> PyResult<u64> {
	let arg = match arg0.extract::<u64>(py) {
		Ok(x) => x,
		Err(_) => {
			let errmsg = "colin doesn't know rust";
			let pyerr = PyErr::new_lazy_init(py.get_type::<exc::ValueError>(), Some(errmsg.to_py_object(py).into_object()));
			return Err(pyerr);
		}
	};
	Ok(pyoxide::fib(arg))

}

fn tokenize(py: Python, text: PyObject) -> PyResult<PyList> {
	
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
	 use super::pyoxide;
    #[test]
    fn it_works() {
    	assert_eq!(pyoxide::fib(1), 1);
    	assert_eq!(pyoxide::fib(5), 8);
    }
}
