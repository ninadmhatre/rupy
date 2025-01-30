
# pq

----

### Description

Parque file utility.

Create a simple Parquet file reader. It should be able to do head, tail, schema and single col query without needing to open the file.


### Usage

----

```bash
$ pq FILE               # show first 10 rows
$ pq -l 10 FILE         # show last 10 rows
$ pq -i FILE            # show schema, row count of the file
$ pq -q "<query>" FILE  # single col query on FILE
```

### Example

----

```bash
$ pq a.parquet                 # park --head|-h 10 a.parquet
$ pq --last|-l 10 a.parquet  
$ pq --info|-i a.parquet       # show schema, row count etc...
$ pq -q "age > 10" a.parquet   # quick query parqut file on single column.
```
