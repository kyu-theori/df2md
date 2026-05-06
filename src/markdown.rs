use polars::chunked_array::ChunkedArray;
use polars::datatypes::StringType;

pub struct ColumnRepr<'a> {
    pub name: String,
    pub cell_width: usize,
    pub rows: &'a ChunkedArray<StringType>,
}

fn estimate_table_size<'a>(columns: &Vec<ColumnRepr<'a>>) -> usize {
    let num_rows = if let Some(first) = columns.first() {
        first.rows.len()
    } else {
        0
    };

    if num_rows == 0 {
        return 0;
    }

    let num_columns = columns.iter().map(|v| v.cell_width + 3).sum::<usize>() + 2;

    num_columns * (num_rows + 2) + 1
}

pub fn build_markdown_table<'a>(columns: Vec<ColumnRepr<'a>>) -> String {
    let mut output = String::with_capacity(estimate_table_size(&columns));

    for column in columns.iter() {
        output.push_str(&format!(
            "| {}{} ",
            column.name.replace("\n", " "),
            " ".repeat(column.cell_width - column.name.len())
        ));
    }
    output.push_str("|\n");
    for column in columns.iter() {
        output.push_str(&format!("| {} ", "-".repeat(column.cell_width)));
    }
    output.push_str("|\n");

    let mut iterators = columns.iter().map(|v| v.rows.iter()).collect::<Vec<_>>();
    'outer: loop {
        for (column, it) in columns.iter().zip(iterators.iter_mut()) {
            let Some(cell) = it.next() else {
                break 'outer;
            };

            if let Some(cell) = cell {
                output.push_str(&format!(
                    "| {}{} ",
                    cell.replace("\n", " "),
                    " ".repeat(column.cell_width - cell.len())
                ));
            } else {
                output.push_str(&format!("| {} ", " ".repeat(column.cell_width)));
            }
        }
        output.push_str("|\n");
    }

    output.push('\n');

    output
}
