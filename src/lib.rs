mod markdown;

use pyo3::prelude::*;
use pyo3_polars::PolarsAllocator;

#[pymodule]
mod df2md {
    use pyo3::exceptions::PyIndexError;
    use pyo3::prelude::*;
    use pyo3_polars::types::PyDataFrame;

    use crate::markdown::{ColumnRepr, build_markdown_table};

    #[pyfunction]
    pub fn _format_dataframe(df: PyDataFrame) -> PyResult<String> {
        let df = df.as_ref();

        let mut columns = vec![];
        let mut column_names = vec![];

        let mut num_rows = 0;
        for column in df.columns() {
            let Ok(strings) = column.str() else {
                continue;
            };

            if num_rows == 0 {
                num_rows = strings.len();
            } else if num_rows != strings.len() {
                return Err(PyIndexError::new_err("Provided dataframe is not tabular."));
            }

            column_names.push(column.name().to_string());
            columns.push(strings);
        }

        let columns = column_names
            .into_iter()
            .zip(columns.iter())
            .map(|(name, rows)| {
                ColumnRepr {
                    name,
                    rows,
                }
            })
            .collect();

        Ok(build_markdown_table(columns))
    }
}

#[global_allocator]
static ALLOC: PolarsAllocator = PolarsAllocator::new();
