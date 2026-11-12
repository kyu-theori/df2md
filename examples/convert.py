import sys

import df2md

if len(sys.argv) < 2:
    print(f'usage: {sys.argv[0]} [filename]')
else:
    filename = sys.argv[1]
    with open(filename, 'rb') as f:
        if filename.lower().endswith('.csv'):
            print(df2md.convert_csv(f))
        elif filename.lower().endswith('.tsv'):
            print(df2md.convert_csv(f, separator='\t'))
        else:
            print(df2md.convert_xlsx(f))
