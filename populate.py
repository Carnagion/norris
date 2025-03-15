"""
Script used to add students from a CSV file to Norris' database in bulk.
Each row of the CSV data must be in the format `firstname,lastname`.
Haskell or Rust versions of this script are also available on request.
"""

import sys

args = sys.argv
file_path = args[1] # NOTE: 0th argument skipped since it is usually the process name
user_kind = args[2]

with open(file_path, "r") as file:
    names = map(lambda line: line.strip().split(",", 1), file.readlines())
    rows = map(lambda name: f"(0, \"{name[0]} {name[1]}\", \"{user_kind}\", null)", names)
    values = ",\n".join(rows)
    print(f"insert into users\nvalues\n{values};")
