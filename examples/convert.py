import sys

import df2md

if len(sys.argv) < 2:
    print(f'usage: {sys.argv[0]} [filename]')
else:
    print(df2md.convert_xlsx(open(sys.argv[1], 'rb')))
