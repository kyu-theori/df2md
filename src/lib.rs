mod markdown;

use pyo3::prelude::*;
use pyo3_polars::PolarsAllocator;

#[pymodule]
mod df2md {
    use pyo3::{exceptions::PyIndexError, prelude::*};
    use pyo3_polars::types::PyDataFrame;
    use rayon::prelude::*;

    use crate::markdown::{ColumnRepr, build_markdown_table};

    static PARALLELIZE_THRESHOLD: usize = 1_000_000;

    #[pyfunction]
    pub fn _format_dataframe(df: PyDataFrame) -> PyResult<String> {
        let df = df.as_ref();

        let mut columns = vec![];
        let mut column_names = vec![];

        let mut num_columns = 0usize;
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
            num_columns += 1;
        }

        let items_count = num_columns * num_rows;

        let column_lengths: Vec<_> = if items_count > PARALLELIZE_THRESHOLD {
            columns
                .par_iter()
                .map(|v| {
                    v.iter()
                        .map(|i| i.map(str::len).unwrap_or(0))
                        .max()
                        .unwrap_or(0)
                })
                .collect()
        } else {
            columns
                .iter()
                .map(|v| {
                    v.iter()
                        .map(|i| i.map(str::len).unwrap_or(0))
                        .max()
                        .unwrap_or(0)
                })
                .collect()
        };

        let columns = column_names
            .into_iter()
            .zip(column_lengths.into_iter())
            .zip(columns.iter())
            .map(|((name, cell_width), rows)| {
                let max_width = std::cmp::max(cell_width, name.len());
                ColumnRepr {
                    name,
                    cell_width: max_width,
                    rows,
                }
            })
            .collect();

        Ok(build_markdown_table(columns))
    }
}

#[global_allocator]
static ALLOC: PolarsAllocator = PolarsAllocator::new();
