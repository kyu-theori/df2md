use polars::chunked_array::ChunkedArray;
use polars::datatypes::StringType;

pub struct ColumnRepr<'a> {
    pub name: String,
    pub rows: &'a ChunkedArray<StringType>,
}

pub fn build_markdown_table<'a>(columns: Vec<ColumnRepr<'a>>) -> String {
    let mut output = String::new();

    for column in columns.iter() {
        output.push_str(&format!(
            "| {} ",
            column.name.replace("\n", " ").replace("|", "\\|"),
        ));
    }
    output.push_str("|\n");
    for column in columns.iter() {
        output.push_str(&format!("| {} ", "-".repeat(column.name.len())));
    }
    output.push_str("|\n");

    let mut iterators = columns.iter().map(|v| v.rows.iter()).collect::<Vec<_>>();
    'outer: loop {
        for it in iterators.iter_mut() {
            let Some(cell) = it.next() else {
                break 'outer;
            };

            if let Some(cell) = cell {
                output.push_str(&format!(
                    "| {} ",
                    cell.replace("\n", " ").replace("|", "\\|"),
                ));
            } else {
                output.push_str("| ");
            }
        }
        output.push_str("|\n");
    }

    output.push('\n');

    output
}
