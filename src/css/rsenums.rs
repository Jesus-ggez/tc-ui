use pyo3::prelude::*;

//<···>
#[pyclass]
pub enum AlignItems {
    START,
    CENTER,
    END,
    STRETCH,
}

#[pymethods]
impl AlignItems {
    #[getter]
    fn value(&self) -> PyResult<String> {
        let value = match self {
            AlignItems::START => "flex-start",
            AlignItems::STRETCH => "stretch",
            AlignItems::CENTER => "center",
            AlignItems::END => "flex-end",
        };

        Ok(value.into())
    }
}
//<<··
#[pyclass]
pub enum AlignContent {
    START,
    CENTER,
    END,
    STRETCH,
    //SPACE_BETWEEN,
    SPACEBETWEEN,
    //SPACE_AROUND,
    SPACEAROUND,
}

#[pymethods]
impl AlignContent {
    #[getter]
    fn value(&self) -> PyResult<String> {
        let value = match self {
            //AlignContent::SPACE_BETWEEN => "space-between",
            AlignContent::SPACEBETWEEN => "space-between",
            //AlignContent::SPACE_AROUND => "space-around",
            AlignContent::SPACEAROUND => "space-around",
            AlignContent::START => "flex-start",
            AlignContent::STRETCH => "stretch",
            AlignContent::CENTER => "center",
            AlignContent::END => "flex-end",
        };

        Ok(value.into())
    }
}

//<<··
#[pyclass]
pub enum JustifyContent {
    START,
    CENTER,
    END,
    //SPACE_BETWEEN,
    SPACEBETWEEN,
    //SPACE_AROUND,
    SPACEAROUND,
    //SPACE_EVENLY,
    SPACEEVENLY,
}

#[pymethods]
impl JustifyContent {
    #[getter]
    fn value(&self) -> PyResult<String> {
        let value = match self {
            //JustifyContent::SPACE_BETWEEN => "space-between",
            JustifyContent::SPACEBETWEEN => "space-between",
            //JustifyContent::SPACE_AROUND => "space-around",
            JustifyContent::SPACEAROUND => "space-around",
            //JustifyContent::SPACE_EVENLY => "space-evenly",
            JustifyContent::SPACEEVENLY => "space-evenly",
            JustifyContent::START => "flex-start",
            JustifyContent::CENTER => "center",
            JustifyContent::END => "flex-end",
        };

        Ok(value.into())
    }
}

