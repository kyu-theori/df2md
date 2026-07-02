from typing import BinaryIO

import polars as pl

from .df2md import *


def convert_xlsx(file_stream: BinaryIO) -> str:
    sheets = pl.read_excel(file_stream, sheet_id=0, drop_empty_cols=False, raise_if_empty=False)
    md = ""
    if isinstance(sheets, dict):
        for s in sheets:
            if len(sheets[s]) == 0:
                continue
            md += f"## {s}\n"
            md += _format_dataframe(sheets[s])
    else:
        md = _format_dataframe(sheets)
    return md
