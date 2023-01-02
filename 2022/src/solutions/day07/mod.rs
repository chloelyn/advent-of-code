use pyo3::{
    types::{PyModule, PyTuple},
    Py, PyAny, PyResult, Python,
};

const CODE: &str = include_str!("solve.py");

pub fn input() -> &'static str {
    include_str!("../../../input/day07.txt")
}

pub fn test_input() -> &'static str {
    include_str!("../../../input/tests/day07.txt")
}

pub fn solve(input: &str) -> (usize, usize) {
    Python::with_gil(|py| -> PyResult<(usize, usize)> {
        let fun: Py<PyAny> = PyModule::from_code(py, CODE, "", "")?
            .getattr("solve")?
            .into();
        let args = (input,);
        let res = fun.call1(py, args)?;
        let solution: &PyTuple = res.as_ref(py).downcast()?;

        Ok((item(&solution, 0), item(&solution, 1)))
    })
    .unwrap()
}

fn item(tuple: &PyTuple, index: usize) -> usize {
    let obj = tuple.get_item(index);
    obj.unwrap().extract::<usize>().unwrap()
}

common::test!(day07, (95437, 24933642), (1555642, 5974547));
