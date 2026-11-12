from typing import BinaryIO

import polars as pl

from .df2md import *


def convert_xlsx(file_stream: BinaryIO) -> str:
    sheets = pl.read_excel(file_stream, sheet_id=0, drop_empty_cols=False, raise_if_empty=False, infer_schema_length=0)
    md = ""
    if isinstance(sheets, dict):
        for s in sheets:
            if sheets[s].shape == (0, 0):
                continue
            md += f"## {s}\n"
            md += _format_dataframe(sheets[s])
    else:
        md = _format_dataframe(sheets)
    return md


def convert_csv(file_stream: BinaryIO, separator: str = ",") -> str:
    df = pl.read_csv(file_stream, separator=separator, infer_schema=False, raise_if_empty=False, truncate_ragged_lines=True)
    if df.shape == (0, 0):
        return ""
    return _format_dataframe(df)
