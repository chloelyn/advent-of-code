use pyo3::{
    types::{PyModule, PyTuple},
    Py, PyAny, PyResult, Python,
};

pub fn input() -> &'static str {
    include_str!("../../../input/day07.txt")
}

pub fn test_input() -> &'static str {
    include_str!("../../../input/tests/day07.txt")
}

pub fn solve(input: &str) -> (usize, usize) {
    let (mut part_one, mut part_two) = (0, 0);
    let code = include_str!("solve.py");
    Python::with_gil(|py| -> PyResult<()> {
        let fun: Py<PyAny> = PyModule::from_code(py, code, "", "")?
            .getattr("solve")?
            .into();
        let args: (&str,) = (input,);
        let res = fun.call1(py, args)?;
        let solution: &PyTuple = res.as_ref(py).downcast()?;
        part_one = item(&solution, 0);
        part_two = item(&solution, 1);

        Ok(())
    })
    .unwrap();
    (part_one, part_two)
}

fn item(tuple: &PyTuple, index: usize) -> usize {
    let obj = tuple.get_item(index);
    obj.unwrap().extract::<usize>().unwrap()
}

common::test!(day07, (95437, 24933642), (1555642, 5974547));
