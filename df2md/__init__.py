from typing import BinaryIO

import pandas as pd
import polars as pl

from .df2md import *


def convert_xlsx(file_stream: BinaryIO) -> str:
    sheets = pd.read_excel(file_stream, sheet_name=None)
    md = ""
    for s in sheets:
        md += f"## {s}\n"
        df = pl.from_pandas(sheets[s].astype(str))
        md += _format_dataframe(df)
    return md
