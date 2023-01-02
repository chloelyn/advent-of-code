use pyo3::{
    types::{PyModule, PyTuple},
    Py, PyAny, PyResult, Python,
};

pub fn solve(input: &str, code: &str) -> (usize, usize) {
    Python::with_gil(|py| -> PyResult<(usize, usize)> {
        let fun: Py<PyAny> = PyModule::from_code(py, code, "", "")?
            .getattr("solve")?
            .into();
        let args = (input,);
        let res = fun.call1(py, args)?;
        let solution: &PyTuple = res.as_ref(py).downcast()?;

        Ok((item(solution, 0), item(solution, 1)))
    })
    .unwrap()
}

fn item(tuple: &PyTuple, index: usize) -> usize {
    let obj = tuple.get_item(index);
    obj.unwrap().extract::<usize>().unwrap()
}
